use std::mem;
use std::rc::Rc;
use std::vec;

use crate::{
    chunk::{Chunk, Instruction, OpCode},
    compilation_context::CompilationContext,
    function::{Function, FunctionType},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    pub current: Option<Token<'a>>,
    pub previous: Option<Token<'a>>,
    pub error: Option<String>,
    panic_mode: bool,
    compilation_context: CompilationContext,
    function_types: Vec<FunctionType>,
    in_class: bool,
}

impl<'a> Parser<'a> {
    pub fn new(mut scanner: Scanner<'a>) -> Parser<'a> {
        let current = scanner.next();
        Parser {
            scanner,
            current,
            previous: None,
            error: None,
            panic_mode: false,
            compilation_context: CompilationContext::new(None),
            function_types: Vec::new(),
            in_class: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Instruction>, String> {
        let mut operations = Vec::new();

        self.function_types.push(FunctionType::Script);

        while !self.is_at_end() {
            operations.extend(self.declaration()?);
        }

        self.consume(TokenType::Eof, "Expected end of file")?;

        self.function_types.pop();

        Ok(operations)
    }

    fn is_at_end(&self) -> bool {
        self.current
            .as_ref()
            .map_or(true, |t| t.token_type == TokenType::Eof)
    }

    fn declaration(&mut self) -> Result<Vec<Instruction>, String> {
        let result;

        if self.match_token(TokenType::Class)? {
            result = self.class_declaration();
        } else if self.match_token(TokenType::Fun)? {
            result = self.fun_declaration();
        } else if self.match_token(TokenType::Var)? {
            result = self.var_declaration();
        } else {
            result = self.statement();
        }

        if self.panic_mode {
            self.syncronize()?;
        }

        result
    }

    fn class_declaration(&mut self) -> Result<Vec<Instruction>, String> {
        self.in_class = true;

        let classname = self.parse_variable("Expect class name")?;
        let line = self.previous.ok_or("Unexpected end of input")?.line;

        let mut result = vec![Instruction::new(OpCode::Class(classname.clone()), line)];
        let variable = self.define_variable(classname.clone(), line)?;
        result.extend(variable);
        result.extend(self.named_variable(&classname, line, false)?);

        if self.match_token(TokenType::Less)? {
            let superclass = self.consume(TokenType::Identifier, "Expect superclass name.")?;
            let line = superclass.line;
            let lexeme = superclass.lexeme.to_string();

            if superclass.lexeme == &classname {
                return Err(self.format_error(
                    line,
                    &lexeme,
                    "A class cannot inherit from itself.",
                ));
            }

            result.extend(self.named_variable(&classname, line, false)?);
            result.extend(self.variable(false)?);

            result.push(Instruction::new(OpCode::Inherit, line));
        }

        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            result.extend(self.method()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;

        result.push(Instruction::new(OpCode::Pop, line));

        self.in_class = false;

        Ok(result)
    }

    fn fun_declaration(&mut self) -> Result<Vec<Instruction>, String> {
        let name = self.parse_variable("Expect function name")?;

        self.function_types.push(FunctionType::Function);

        let function = self.function(name.clone(), FunctionType::Function)?;
        let variable = self.define_variable(
            name.clone(),
            self.previous.ok_or("Unexpected end of input")?.line,
        )?;

        self.function_types.pop();

        Ok([function, variable].concat())
    }

    fn function(
        &mut self,
        name: String,
        function_type: FunctionType,
    ) -> Result<Vec<Instruction>, String> {
        self.compilation_context =
            CompilationContext::new(Some(Box::new(mem::take(&mut self.compilation_context))));

        let zero_slot_name = match function_type {
            FunctionType::Method => "this",
            FunctionType::Initializer => "this",
            _ => "",
        };

        self.compilation_context
            .add_local(zero_slot_name.to_string())?;

        self.consume(TokenType::LeftParen, "Expect '(' after function name")?;
        let mut arity = 0;

        self.begin_scope();
        // Handle 0 arguments: only enter the loop if the next token is not ')'
        if !self.check(TokenType::RightParen) {
            loop {
                let param_token = self.consume(TokenType::Identifier, "Expect parameter name")?;
                let param_name = param_token.lexeme.to_string();

                self.compilation_context.add_local(param_name)?;
                self.compilation_context.mark_initialized()?;

                arity += 1;

                if arity > 255 {
                    return Err(format!("Cannot have more than 255 parameters"));
                }

                if !self.match_token(TokenType::Comma)? {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters")?;
        self.consume(TokenType::LeftBrace, "Expect '{' before function body")?;

        let block = self.block()?;
        let mut chunk = Chunk::new();
        chunk.extend(block);
        self.end_scope()?;

        let line = self.previous.ok_or("Unexpected end of input")?.line;

        if matches!(function_type, FunctionType::Initializer) {
            chunk.extend(vec![
                Instruction::new(OpCode::GetLocal(0), line),
                Instruction::new(OpCode::Return, line),
            ]);
        }

        if !matches!(
            chunk.get_last_instruction().map(|i| &i.op_code),
            Some(OpCode::Return)
        ) {
            chunk.extend(vec![
                Instruction::new(OpCode::Value(Value::Nil), line),
                Instruction::new(OpCode::Return, line),
            ]);
        }

        let function = Function {
            name: name.to_string(),
            arity,
            function_type,
            chunk,
        };

        let mut operations = vec![Instruction::new(OpCode::Closure(Rc::new(function)), line)];

        self.compilation_context
            .upvalues
            .iter()
            .for_each(|upvalue| {
                operations.push(Instruction::new(
                    OpCode::Upvalue(upvalue.index, upvalue.is_local),
                    line,
                ))
            });

        self.compilation_context = self
            .compilation_context
            .take_enclosing()
            .ok_or("Expected enclosing compilation context")?;

        Ok(operations)
    }

    fn method(&mut self) -> Result<Vec<Instruction>, String> {
        let name = self.parse_variable("Expect method name")?;
        let line = self.previous.ok_or("Unexpected end of input")?.line;

        let function_type = if name == "init" {
            FunctionType::Initializer
        } else {
            FunctionType::Method
        };

        self.function_types.push(function_type.clone());

        let mut operations = self.function(name.clone(), function_type)?;

        self.function_types.pop();

        operations.push(Instruction::new(OpCode::Method(name), line));

        Ok(operations)
    }

    fn var_declaration(&mut self) -> Result<Vec<Instruction>, String> {
        let name = self.parse_variable("Expect variable name")?;
        let line = self.previous.ok_or("Unexpected end of input")?.line;

        let mut operations = Vec::new();

        let match_equal = self.match_token(TokenType::Equal)?;
        if match_equal {
            operations = self.expression()?;
        } else {
            operations.push(Instruction::new(OpCode::Value(Value::Nil), line));
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration",
        )?;

        operations.extend(self.define_variable(name, line)?);
        Ok(operations)
    }

    /// Parses a variable name, declares it if in a local scope, and returns the name.
    fn parse_variable(&mut self, error_message: &str) -> Result<String, String> {
        let identifier = self.consume(TokenType::Identifier, error_message)?;
        let name = identifier.lexeme.to_string();

        if self.compilation_context.get_depth() > 0 {
            self.declare_variable(name.clone())?;
        }

        Ok(name)
    }

    /// Defines a variable: if in a local scope, marks it initialized; if global, emits a DefineGlobal instruction.
    /// Returns any instructions to emit (for global scope), or an empty Vec for local scope.
    fn define_variable(&mut self, name: String, line: usize) -> Result<Vec<Instruction>, String> {
        if self.compilation_context.get_depth() > 0 {
            self.compilation_context.mark_initialized()?;
            Ok(vec![])
        } else {
            Ok(vec![Instruction::new(OpCode::DefineGlobal(name), line)])
        }
    }

    /// Declares a new variable in the current scope.
    ///
    /// Checks if a variable with the same name already exists in the current lexical scope.
    /// If so, returns an error. Otherwise, adds the variable to the local scope registry.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to declare.
    ///
    /// # Errors
    ///
    /// Returns an error if the variable is already declared in the current scope.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the variable was successfully declared, or an error message otherwise.
    fn declare_variable(&mut self, name: String) -> Result<(), String> {
        for variable in self.compilation_context.iter() {
            match variable.depth {
                Some(depth)
                    if depth == self.compilation_context.get_depth() && variable.name == name =>
                {
                    return Err(format!(
                        "Variable '{}' already declared in this scope",
                        name
                    ));
                }
                _ => {}
            }
        }

        self.compilation_context.add_local(name)?;
        Ok(())
    }

    fn statement(&mut self) -> Result<Vec<Instruction>, String> {
        if self.match_token(TokenType::Print)? {
            return self.print_statement();
        }

        if self.match_token(TokenType::If)? {
            return self.if_statement();
        }

        if self.match_token(TokenType::Return)? {
            return self.return_statement();
        }

        if self.match_token(TokenType::While)? {
            return self.while_statement();
        }

        if self.match_token(TokenType::For)? {
            return self.for_statement();
        }

        let mut operations = Vec::new();

        if self.match_token(TokenType::LeftBrace)? {
            self.begin_scope();
            operations = self.block()?;
            operations.extend(self.end_scope()?);
        } else {
            operations.extend(self.expression_statement()?);
        }

        Ok(operations)
    }

    fn print_statement(&mut self) -> Result<Vec<Instruction>, String> {
        let mut operations = self.expression()?;
        operations.push(Instruction::new(OpCode::Print, self.get_line()?));

        self.consume(TokenType::Semicolon, "Expect ';' after value")?;

        Ok(operations)
    }

    fn if_statement(&mut self) -> Result<Vec<Instruction>, String> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'")?;
        let mut operations = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition")?;

        let mut then_statement = self.statement()?;

        let match_else = self.match_token(TokenType::Else)?;

        operations.push(Instruction::new(
            OpCode::JumpIfFalse(then_statement.len() + 2), // + Pop + Jump
            self.get_line()?,
        ));
        operations.push(Instruction::new(OpCode::Pop, self.get_line()?));
        operations.append(&mut then_statement);

        if match_else {
            let mut else_statement = self.statement()?;

            operations.push(Instruction::new(
                OpCode::Jump(else_statement.len() + 1), // + Pop
                self.get_line()?,
            ));
            operations.push(Instruction::new(OpCode::Pop, self.get_line()?));
            operations.append(&mut else_statement);
        } else {
            operations.push(Instruction::new(OpCode::Jump(1), self.get_line()?));
            operations.push(Instruction::new(OpCode::Pop, self.get_line()?));
        }

        Ok(operations)
    }

    fn return_statement(&mut self) -> Result<Vec<Instruction>, String> {
        let mut operations = vec![];

        match self.function_types.last() {
            Some(FunctionType::Script) => {
                return Err("Can't return from top-level code.".to_string());
            }
            Some(FunctionType::Initializer) => {
                return Err("Can't return a value from an initializer.".to_string());
            }
            _ => (),
        }

        if self.match_token(TokenType::Semicolon)? {
            operations.push(Instruction::new(
                OpCode::Value(Value::Nil),
                self.get_line()?,
            ));
            operations.push(Instruction::new(OpCode::Return, self.get_line()?));
        } else {
            operations.append(&mut self.expression()?);
            self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
            operations.push(Instruction::new(OpCode::Return, self.get_line()?));
        }

        Ok(operations)
    }

    fn while_statement(&mut self) -> Result<Vec<Instruction>, String> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let mut operations = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let mut body = self.statement()?;

        operations.push(Instruction::new(
            OpCode::JumpIfFalse(body.len() + 2), // + Pop + Loop
            self.get_line()?,
        ));
        operations.push(Instruction::new(OpCode::Pop, self.get_line()?));
        operations.append(&mut body);
        operations.push(Instruction::new(
            OpCode::Loop(operations.len() + 1), // + Loop itself
            self.get_line()?,
        ));

        Ok(operations)
    }

    fn for_statement(&mut self) -> Result<Vec<Instruction>, String> {
        self.begin_scope();
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let mut operations = match (
            self.match_token(TokenType::Var),
            self.match_token(TokenType::Semicolon),
        ) {
            (Ok(true), Ok(false)) => self.var_declaration(),
            (Ok(false), Ok(true)) => Ok(vec![]), // no initializer
            (Ok(false), Ok(false)) => self.expression_statement(),
            (_, _) => Err("Invalid for statement".to_string()),
        }?;

        let loop_start_index = operations.len();

        let mut condition_jump_index = None;

        if !self.match_token(TokenType::Semicolon)? {
            let condition = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;
            operations.extend(condition);

            condition_jump_index = Some(operations.len());
            operations.push(Instruction::new(OpCode::JumpIfFalse(0), self.get_line()?));
            operations.push(Instruction::new(OpCode::Pop, self.get_line()?));
        }

        let increment = if !self.check(TokenType::RightParen) {
            let mut expression = self.expression()?;
            expression.push(Instruction::new(OpCode::Pop, self.get_line()?));
            expression
        } else {
            vec![]
        };

        self.consume(
            TokenType::RightParen,
            "Expect ')' after for loop condition.",
        )?;

        let mut body = self.statement()?;
        body.extend(increment);
        body.push(Instruction::new(
            OpCode::Loop(body.len() + operations.len() - loop_start_index + 1), // + Loop
            self.get_line()?,
        ));

        if let Some(index) = condition_jump_index {
            operations[index].op_code = OpCode::JumpIfFalse(body.len() + 1);
        }

        operations.extend(body);
        operations.push(Instruction::new(OpCode::Pop, self.get_line()?)); // for JumpIfFalse to pop "false"

        operations.extend(self.end_scope()?);
        Ok(operations)
    }

    fn block(&mut self) -> Result<Vec<Instruction>, String> {
        let mut operations = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            operations.extend(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block")?;

        Ok(operations)
    }

    fn begin_scope(&mut self) {
        self.compilation_context.increment_depth();
    }

    fn end_scope(&mut self) -> Result<Vec<Instruction>, String> {
        self.compilation_context.decrement_depth();

        let line = self.previous.ok_or("Unexpected end of input")?.line;

        let mut instructions = Vec::new();

        loop {
            if self.compilation_context.is_empty() {
                break;
            }

            let variable = self
                .compilation_context
                .peek()
                .ok_or("Unexpected end of input")?;

            match variable.depth {
                Some(depth) if depth <= self.compilation_context.get_depth() => break,
                _ => {}
            }

            match self.compilation_context.pop() {
                Some(variable) => {
                    if variable.is_captured {
                        instructions.push(Instruction::new(OpCode::CloseUpvalue, line));
                    } else {
                        instructions.push(Instruction::new(OpCode::Pop, line));
                    }
                }
                None => {}
            }
        }

        Ok(instructions)
    }

    fn expression_statement(&mut self) -> Result<Vec<Instruction>, String> {
        let mut operations = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;

        operations.push(Instruction::new(OpCode::Pop, self.get_line()?));

        Ok(operations)
    }

    fn expression(&mut self) -> Result<Vec<Instruction>, String> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<Vec<Instruction>, String> {
        self.advance()?;

        let previous = self
            .previous
            .ok_or("Expected to parse an expression, found nothing")?;

        let prefix = self.get_rule(previous.token_type).prefix.ok_or(format!(
            "Expected prefix rule for {:?}",
            previous.token_type
        ))?;

        let can_assign = !precedence.greater_than(Precedence::Assignment);
        let mut operations = match prefix {
            PrefixParseFn::ParseFn(f) => f(self)?,
            PrefixParseFn::ParseFnCanAssign(f) => f(self, can_assign)?,
        };

        loop {
            let current = self
                .current
                .ok_or("Expected expression, but reached end of file")?;
            let next_precedence = self.get_rule(current.token_type).precedence;

            if precedence.greater_than(next_precedence) {
                break;
            }
            self.advance()?;

            let infix = self.get_rule(current.token_type).infix;

            if let Some(infix) = infix {
                match infix {
                    InfixParseFn::ParseFn(f) => {
                        operations.extend(f(self)?);
                    }
                    InfixParseFn::ParseFnCanAssign(f) => {
                        operations.extend(f(self, can_assign)?);
                    }
                }
            }
        }

        if can_assign && self.match_token(TokenType::Equal)? {
            return Err("Invalid assignment target.".to_string());
        }

        Ok(operations)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, String> {
        match self.current.clone() {
            Some(token) if token.token_type == token_type => {
                self.advance()?;
                Ok(token)
            }
            Some(token) => {
                self.error_at(&token, message)?;
                Err(message.to_string())
            }
            None => {
                self.error_at(
                    &Token {
                        token_type: TokenType::Error,
                        lexeme: "",
                        line: 0,
                    },
                    "Unexpected end of file",
                )?;
                Err("Unexpected end of file".to_string())
            }
        }
    }

    fn advance(&mut self) -> Result<(), String> {
        self.previous = self.current.take();

        loop {
            let next = &self.scanner.next();

            if next.is_none() {
                self.current = None;
                break;
            }

            let next = next.unwrap();

            if next.token_type != TokenType::Error {
                self.current = Some(next);
                break;
            }

            self.error_at(&next, "Unexpected token")?;
        }

        match self.error {
            Some(ref e) => Err(e.to_string()),
            None => Ok(()),
        }
    }

    fn get_rule(&self, operator: TokenType) -> ParseRule {
        let grouping = Box::new(|parser: &mut Parser| parser.grouping());
        let unary = Box::new(|parser: &mut Parser| parser.unary());
        let binary = Box::new(|parser: &mut Parser| parser.binary());
        let number = Box::new(|parser: &mut Parser| parser.number());
        let literal = Box::new(|parser: &mut Parser| parser.literal());
        let variable =
            Box::new(|parser: &mut Parser, can_assign: bool| parser.variable(can_assign));
        let and = Box::new(|parser: &mut Parser| parser.and());
        let or = Box::new(|parser: &mut Parser| parser.or());
        let call = Box::new(|parser: &mut Parser| parser.call());
        let dot = Box::new(|parser: &mut Parser, can_assign: bool| parser.dot(can_assign));
        let this = Box::new(|parser: &mut Parser| parser.this());

        match operator {
            TokenType::LeftParen => ParseRule {
                prefix: Some(PrefixParseFn::ParseFn(grouping)),
                infix: Some(InfixParseFn::ParseFn(call)),
                precedence: Precedence::Call,
            },
            TokenType::Minus => ParseRule {
                prefix: Some(PrefixParseFn::ParseFn(unary)),
                infix: Some(InfixParseFn::ParseFn(binary)),
                precedence: Precedence::Term,
            },
            TokenType::Plus => ParseRule {
                prefix: None,
                infix: Some(InfixParseFn::ParseFn(binary)),
                precedence: Precedence::Term,
            },
            TokenType::Star | TokenType::Slash => ParseRule {
                prefix: None,
                infix: Some(InfixParseFn::ParseFn(binary)),
                precedence: Precedence::Factor,
            },
            TokenType::Number => ParseRule {
                prefix: Some(PrefixParseFn::ParseFn(number)),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::True | TokenType::False | TokenType::Nil | TokenType::String => ParseRule {
                prefix: Some(PrefixParseFn::ParseFn(literal)),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Bang => ParseRule {
                prefix: Some(PrefixParseFn::ParseFn(unary)),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::EqualEqual | TokenType::BangEqual => ParseRule {
                prefix: None,
                infix: Some(InfixParseFn::ParseFn(binary)),
                precedence: Precedence::Equality,
            },
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => ParseRule {
                prefix: None,
                infix: Some(InfixParseFn::ParseFn(binary)),
                precedence: Precedence::Comparison,
            },
            TokenType::Identifier => ParseRule {
                prefix: Some(PrefixParseFn::ParseFnCanAssign(variable)),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::And => ParseRule {
                prefix: None,
                infix: Some(InfixParseFn::ParseFn(and)),
                precedence: Precedence::And,
            },
            TokenType::Or => ParseRule {
                prefix: None,
                infix: Some(InfixParseFn::ParseFn(or)),
                precedence: Precedence::Or,
            },
            TokenType::Dot => ParseRule {
                prefix: None,
                infix: Some(InfixParseFn::ParseFnCanAssign(dot)),
                precedence: Precedence::Call,
            },
            TokenType::This => ParseRule {
                prefix: Some(PrefixParseFn::ParseFn(this)),
                infix: None,
                precedence: Precedence::None,
            },
            _ => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        }
    }

    fn grouping(&mut self) -> Result<Vec<Instruction>, String> {
        let expression = self.expression()?;

        self.consume(
            TokenType::RightParen,
            "Expect ')' after grouping expression",
        )?;

        Ok(expression)
    }

    fn and(&mut self) -> Result<Vec<Instruction>, String> {
        let mut expression = self.parse_precedence(Precedence::And)?;
        let mut operations = vec![Instruction::new(
            OpCode::JumpIfFalse(expression.len()),
            self.get_line()?,
        )];
        operations.append(&mut expression);

        return Ok(operations);
    }

    fn or(&mut self) -> Result<Vec<Instruction>, String> {
        let mut operations = vec![Instruction::new(OpCode::JumpIfFalse(1), self.get_line()?)];

        let mut expression = self.parse_precedence(Precedence::Or)?;

        operations.push(Instruction::new(
            OpCode::Jump(expression.len()),
            self.get_line()?,
        ));
        operations.append(&mut expression);

        return Ok(operations);
    }

    fn binary(&mut self) -> Result<Vec<Instruction>, String> {
        let previous = self
            .previous
            .ok_or("Expected binary operator, found nothing")?;
        let token_type = previous.token_type;
        let line = previous.line;
        let precedence = self.get_rule(token_type).precedence.next().ok_or(format!(
            "Can not determine precedence for token: {:?}",
            token_type
        ))?;
        let mut operations = self.parse_precedence(precedence)?;

        match token_type {
            TokenType::Plus => operations.push(Instruction::new(OpCode::Add, line)),
            TokenType::Minus => operations.push(Instruction::new(OpCode::Subtract, line)),
            TokenType::Star => operations.push(Instruction::new(OpCode::Multiply, line)),
            TokenType::Slash => operations.push(Instruction::new(OpCode::Divide, line)),
            TokenType::EqualEqual => operations.push(Instruction::new(OpCode::Equal, line)),
            TokenType::BangEqual => {
                operations.push(Instruction::new(OpCode::Equal, line));
                operations.push(Instruction::new(OpCode::Not, line));
            }
            TokenType::Greater => operations.push(Instruction::new(OpCode::Greater, line)),
            TokenType::GreaterEqual => {
                operations.push(Instruction::new(OpCode::Less, line));
                operations.push(Instruction::new(OpCode::Not, line));
            }
            TokenType::Less => operations.push(Instruction::new(OpCode::Less, line)),
            TokenType::LessEqual => {
                operations.push(Instruction::new(OpCode::Greater, line));
                operations.push(Instruction::new(OpCode::Not, line));
            }
            _ => {
                return Err(format!("Unexpected binary operator type: {:?}", token_type));
            }
        }

        Ok(operations)
    }

    fn unary(&mut self) -> Result<Vec<Instruction>, String> {
        let previous = self
            .previous
            .ok_or("No operand found when parsing unary expression")?;

        let mut instructions = self.parse_precedence(Precedence::Unary)?;

        match previous.token_type {
            TokenType::Minus => instructions.push(Instruction::new(OpCode::Negate, previous.line)),
            TokenType::Bang => instructions.push(Instruction::new(OpCode::Not, previous.line)),
            _ => {
                return Err(format!(
                    "Unexpected unary operator type: {:?}",
                    previous.token_type
                ));
            }
        };

        Ok(instructions)
    }

    fn number(&mut self) -> Result<Vec<Instruction>, String> {
        let previous = self
            .previous
            .ok_or("Expected number when parsing number, found nothing")?;

        previous
            .lexeme
            .parse::<f64>()
            .map(|value| {
                vec![Instruction::new(
                    OpCode::Value(Value::Number(value)),
                    previous.line,
                )]
            })
            .map_err(|e| e.to_string())
    }

    fn literal(&mut self) -> Result<Vec<Instruction>, String> {
        let previous = self.previous.ok_or("Expected literal, found nothing")?;
        let lexeme = previous.lexeme;
        let line = previous.line;

        match previous.token_type {
            TokenType::True | TokenType::False => Ok(vec![Instruction::new(
                OpCode::Value(Value::Bool(previous.token_type == TokenType::True)),
                line,
            )]),
            TokenType::Nil => Ok(vec![Instruction::new(OpCode::Value(Value::Nil), line)]),
            TokenType::String => Ok(vec![Instruction::new(
                OpCode::Value(Value::String(Rc::new(lexeme.to_string()))),
                line,
            )]),
            _ => Err(format!(
                "Unexpected literal type: {:?}",
                previous.token_type
            )),
        }
    }

    /// Parses a variable expression.
    ///
    /// This function is called when encountering a variable in an expression.
    /// The `can_assign` parameter indicates whether the variable is a valid assignment target
    /// in the current context (e.g., on the left side of an assignment).
    ///
    /// # Arguments
    ///
    /// * `can_assign` - If `true`, assignment to this variable is allowed (e.g., in `var a = 1;`).
    ///                  If `false`, assignment is not allowed (e.g., in `a + 1`).
    ///
    /// # Returns
    ///
    /// Returns a vector of `Instruction` representing the code to access or assign to the variable,
    /// or an error if the variable name is missing.
    fn variable(&mut self, can_assign: bool) -> Result<Vec<Instruction>, String> {
        let previous = self
            .previous
            .ok_or("Expected variable name, found nothing")?;

        let name = previous.lexeme;
        let line = previous.line;

        self.named_variable(name, line, can_assign)
    }

    fn call(&mut self) -> Result<Vec<Instruction>, String> {
        let (mut instructions, count) = self.arguments()?;
        let line = self.previous.ok_or("Unexpected end of input")?.line;

        instructions.push(Instruction::new(OpCode::Call(count), line));

        Ok(instructions)
    }

    fn dot(&mut self, can_assign: bool) -> Result<Vec<Instruction>, String> {
        let identifier = self.consume(TokenType::Identifier, "Expected identifier after '.'.")?;
        let lexeme = identifier.lexeme.to_string();

        let line = self.previous.ok_or("Unexpected end of input")?.line;

        if can_assign && self.match_token(TokenType::Equal)? {
            let mut instructions = self.expression()?;
            instructions.push(Instruction::new(OpCode::SetProperty(lexeme), line));
            Ok(instructions)
        } else if self.match_token(TokenType::LeftParen)? {
            let (mut instructions, count) = self.arguments()?;
            instructions.push(Instruction::new(OpCode::Invoke(lexeme, count), line));
            Ok(instructions)
        } else {
            Ok(vec![Instruction::new(OpCode::GetProperty(lexeme), line)])
        }
    }

    fn this(&mut self) -> Result<Vec<Instruction>, String> {
        if !self.in_class {
            return self.error_at(
                &self.previous.unwrap(),
                "Can't use 'this' outside of a class.",
            );
        }

        self.variable(false)
    }

    fn named_variable(
        &mut self,
        name: &str,
        line: usize,
        can_assign: bool,
    ) -> Result<Vec<Instruction>, String> {
        let mut set_operation = OpCode::SetGlobal(name.to_string());
        let mut get_operation = OpCode::GetGlobal(name.to_string());

        if let Some(local) = self.compilation_context.resolve_local(name)? {
            set_operation = OpCode::SetLocal(local.clone());
            get_operation = OpCode::GetLocal(local.clone());
        } else if let Some(upvalue) = self.compilation_context.resolve_upvalue(name)? {
            set_operation = OpCode::SetUpvalue(upvalue);
            get_operation = OpCode::GetUpvalue(upvalue);
        }

        if can_assign && self.match_token(TokenType::Equal)? {
            let mut operations = self.expression()?;
            operations.push(Instruction::new(set_operation, line));
            return Ok(operations);
        }

        Ok(vec![Instruction::new(get_operation, line)])
    }

    fn arguments(&mut self) -> Result<(Vec<Instruction>, usize), String> {
        let mut arguments = Vec::new();
        let mut count = 0;

        if !self.match_token(TokenType::RightParen)? {
            loop {
                count += 1;
                let arg = self.expression()?;
                arguments.extend(arg);

                if !self.match_token(TokenType::Comma)? {
                    self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;
                    break;
                }
            }
        }

        Ok((arguments, count))
    }

    fn match_token(&mut self, token_type: TokenType) -> Result<bool, String> {
        if self.check(token_type) {
            self.advance()?;
            return Ok(true);
        }

        Ok(false)
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        match &self.current {
            Some(token) => token.token_type == token_type,
            None => false,
        }
    }

    #[allow(unreachable_code)]
    fn syncronize(&mut self) -> Result<(), String> {
        self.panic_mode = false;

        loop {
            if self.current.is_none() {
                return Ok(());
            }

            if self.previous.is_some() && self.previous.unwrap().token_type == TokenType::Semicolon
            {
                return Ok(());
            }

            match self.current.unwrap().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return Ok(()),
                _ => (),
            }

            self.advance()?;
        }

        Ok(())
    }

    fn error_at(&mut self, token: &Token<'a>, message: &str) -> Result<Vec<Instruction>, String> {
        if self.panic_mode {
            return Ok(vec![]);
        }

        self.panic_mode = true;

        if token.token_type == TokenType::Error {
            return Ok(vec![]);
        }

        let error_string = self.format_error(token.line, token.lexeme, message);

        self.error = Some(error_string.clone());

        Err(error_string)
    }

    fn get_line(&mut self) -> Result<usize, String> {
        Ok(self.previous.ok_or("Cannot get current line")?.line)
    }

    fn format_error(&self, line: usize, lexeme: &str, message: &str) -> String {
        format!("[line {}] Error at {}: {}", line, lexeme, message)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl PartialEq for Precedence {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Precedence::None, Precedence::None) => true,
            (Precedence::Assignment, Precedence::Assignment) => true,
            (Precedence::Or, Precedence::Or) => true,
            (Precedence::And, Precedence::And) => true,
            (Precedence::Equality, Precedence::Equality) => true,
            (Precedence::Comparison, Precedence::Comparison) => true,
            (Precedence::Term, Precedence::Term) => true,
            (Precedence::Factor, Precedence::Factor) => true,
            (Precedence::Unary, Precedence::Unary) => true,
            (Precedence::Call, Precedence::Call) => true,
            (Precedence::Primary, Precedence::Primary) => true,
            _ => false,
        }
    }
}

impl Precedence {
    pub fn next(&self) -> Option<Precedence> {
        match self {
            Precedence::None => Some(Precedence::Assignment),
            Precedence::Assignment => Some(Precedence::Or),
            Precedence::Or => Some(Precedence::And),
            Precedence::And => Some(Precedence::Equality),
            Precedence::Equality => Some(Precedence::Comparison),
            Precedence::Comparison => Some(Precedence::Term),
            Precedence::Term => Some(Precedence::Factor),
            Precedence::Factor => Some(Precedence::Unary),
            Precedence::Unary => Some(Precedence::Call),
            Precedence::Call => Some(Precedence::Primary),
            Precedence::Primary => None,
        }
    }

    pub fn greater_than(&self, other: Precedence) -> bool {
        if self == &other {
            return false;
        }

        let mut other = Some(other);

        while other.is_some() {
            if *self == other.unwrap() {
                return true;
            }

            other = other.unwrap().next();
        }

        false
    }
}

type ParseFn = Box<dyn Fn(&mut Parser) -> Result<Vec<Instruction>, String>>;
type ParseFnCanAssign = Box<dyn Fn(&mut Parser, bool) -> Result<Vec<Instruction>, String>>;

enum PrefixParseFn {
    ParseFn(ParseFn),
    ParseFnCanAssign(ParseFnCanAssign),
}

enum InfixParseFn {
    ParseFn(ParseFn),
    ParseFnCanAssign(ParseFnCanAssign),
}

struct ParseRule {
    prefix: Option<PrefixParseFn>,
    infix: Option<InfixParseFn>,
    precedence: Precedence,
}
