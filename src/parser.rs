use crate::{
    chunk::{Instruction, OpCode},
    lexical_scope::LexicalScopeRegistry,
    logger,
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    pub current: Option<Token<'a>>,
    pub previous: Option<Token<'a>>,
    pub error: Option<String>,
    panic_mode: bool,
    lexical_scope_registry: LexicalScopeRegistry,
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
            lexical_scope_registry: LexicalScopeRegistry::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("parse");

        let mut operations = Vec::new();

        while !self.is_at_end() {
            operations.extend(self.declaration()?);
        }

        self.consume(TokenType::Eof, "Expected end of file")?;

        Ok(operations)
    }

    fn is_at_end(&self) -> bool {
        self.current
            .as_ref()
            .map_or(true, |t| t.token_type == TokenType::Eof)
    }

    fn declaration(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("declaration");

        let result;

        if self.match_token(TokenType::Var)? {
            result = self.var_declaration();
        } else {
            result = self.statement();
        }

        if self.panic_mode {
            self.syncronize()?;
        }

        result
    }

    fn var_declaration(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("var_declaration");

        let depth = self.lexical_scope_registry.depth;
        let identifier = self.consume(TokenType::Identifier, "Expect variable name")?;

        let name = identifier.lexeme.to_string();
        let line = identifier.line;

        let mut operations = Vec::new();

        if depth > 0 {
            self.declare_variable(name.clone())?;
        }

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

        if depth > 0 {
            self.lexical_scope_registry.mark_initialized()?;
            return Ok(operations);
        }

        operations.push(Instruction::new(OpCode::DefineGlobal(name), line));

        if match_equal {
            operations.push(Instruction::new(OpCode::Pop, line));
        }

        Ok(operations)
    }

    fn declare_variable(&mut self, name: String) -> Result<(), String> {
        for variable in self.lexical_scope_registry.iter() {
            match variable.depth {
                Some(depth)
                    if depth == self.lexical_scope_registry.depth && variable.name == name =>
                {
                    return Err(format!(
                        "Variable '{}' already declared in this scope",
                        name
                    ));
                }
                _ => {}
            }
        }

        self.lexical_scope_registry.add_local(name)?;
        Ok(())
    }

    fn statement(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("statement");

        if self.match_token(TokenType::Print)? {
            return self.print_statement();
        }

        if self.match_token(TokenType::If)? {
            return self.if_statement();
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

            logger::debug("END BLOCK");
        } else {
            operations.extend(self.expression_statement()?);
        }

        Ok(operations)
    }

    fn print_statement(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("print_statement");

        let mut operations = self.expression()?;
        operations.push(Instruction::new(OpCode::Print, self.get_line()?));

        self.consume(TokenType::Semicolon, "Expect ';' after value")?;

        Ok(operations)
    }

    fn if_statement(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("if_statement");

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
        logger::debug("block");

        let mut operations = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            operations.extend(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block")?;

        Ok(operations)
    }

    fn begin_scope(&mut self) {
        self.lexical_scope_registry.increment_depth();
    }

    fn end_scope(&mut self) -> Result<Vec<Instruction>, String> {
        self.lexical_scope_registry.decrement_depth();

        let line = self.previous.ok_or("Unexpected end of input")?.line;

        let mut instructions = Vec::new();

        loop {
            if self.lexical_scope_registry.is_empty() {
                break;
            }

            let variable = self
                .lexical_scope_registry
                .peek()
                .ok_or("Unexpected end of input")?;

            match variable.depth {
                Some(depth) if depth <= self.lexical_scope_registry.depth => break,
                _ => {}
            }

            self.lexical_scope_registry.pop();
            instructions.push(Instruction::new(OpCode::Pop, line));
        }

        Ok(instructions)
    }

    fn expression_statement(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("expression_statement");

        let mut operations = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;

        operations.push(Instruction::new(OpCode::Pop, self.get_line()?));

        Ok(operations)
    }

    fn expression(&mut self) -> Result<Vec<Instruction>, String> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<Vec<Instruction>, String> {
        logger::debug(&format!("parse_precedence: {:?}", precedence));

        self.advance()?;

        let previous = self
            .previous
            .ok_or("Expected to parse an expression, found nothing")?;

        let prefix = self.get_rule(previous.token_type).prefix.ok_or(format!(
            "Expected prefix rule for {:?}",
            previous.token_type
        ))?;

        logger::debug(&format!(
            "Found prefix operation for previous token_type {:?}",
            previous.token_type
        ));

        let can_assign = !precedence.greater_than(Precedence::Assignment);
        let mut operations = match prefix {
            PrefixParseFn::ParseFn(f) => f(self)?,
            PrefixParseFn::ParseFnCanAssign(f) => f(self, can_assign)?,
        };

        loop {
            logger::debug(&format!(
                "parse_precedence: {:?} next iteration",
                precedence
            ));
            let current = self
                .current
                .ok_or("Expected expression, but reached end of file")?;
            let next_precedence = self.get_rule(current.token_type).precedence;

            logger::debug(&format!("  current: {:?}", current));
            logger::debug(&format!("  next_precedence: {:?}", next_precedence));

            if precedence.greater_than(next_precedence) {
                logger::debug(&format!(
                    "Precedence {:?} is > than {:?}, breaking the loop",
                    precedence, next_precedence
                ));
                break;
            } else {
                logger::debug(&format!(
                    "Precedence {:?} is <= than {:?}, continuing the loop",
                    precedence, next_precedence
                ));
            }

            self.advance()?;

            let infix = self.get_rule(current.token_type).infix;

            if let Some(infix) = infix {
                logger::debug(&format!(
                    "Found infix operation for current token_type {:?}",
                    current.token_type
                ));

                match infix {
                    InfixParseFn::ParseFn(f) => {
                        operations.extend(f(self)?);
                    }
                }
            } else {
                logger::debug(&format!("No infix rule found for {:?}", current.token_type));
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

        logger::debug("advance - after");
        logger::debug(&format!("  previous: {:?}", self.previous));
        logger::debug(&format!("  current: {:?}", self.current));

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

        match operator {
            TokenType::LeftParen => ParseRule {
                prefix: Some(PrefixParseFn::ParseFn(grouping)),
                infix: None,
                precedence: Precedence::None,
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
            _ => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        }
    }

    fn grouping(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("grouping");

        let expression = self.expression()?;

        self.consume(
            TokenType::RightParen,
            "Expect ')' after grouping expression",
        )?;

        Ok(expression)
    }

    fn and(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("and");

        let mut expression = self.parse_precedence(Precedence::And)?;
        let mut operations = vec![Instruction::new(
            OpCode::JumpIfFalse(expression.len()),
            self.get_line()?,
        )];
        operations.append(&mut expression);

        return Ok(operations);
    }

    fn or(&mut self) -> Result<Vec<Instruction>, String> {
        logger::debug("or");

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
        logger::debug("binary");

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
        logger::debug("unary");

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
        logger::debug("number");

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
        logger::debug("literal");

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
                OpCode::Value(Value::String(lexeme.to_string())),
                line,
            )]),
            _ => Err(format!(
                "Unexpected literal type: {:?}",
                previous.token_type
            )),
        }
    }

    fn variable(&mut self, can_assign: bool) -> Result<Vec<Instruction>, String> {
        logger::debug("variable");

        let previous = self
            .previous
            .ok_or("Expected variable name, found nothing")?;

        let name = previous.lexeme;
        let line = previous.line;

        self.named_variable(name, line, can_assign)
    }

    fn named_variable(
        &mut self,
        name: &str,
        line: usize,
        can_assign: bool,
    ) -> Result<Vec<Instruction>, String> {
        logger::debug("named_variable");

        let mut set_operation = OpCode::SetGlobal(name.to_string());
        let mut get_operation = OpCode::GetGlobal(name.to_string());

        if let Some(local) = self.lexical_scope_registry.resolve_local(name)? {
            set_operation = OpCode::SetLocal(local.clone());
            get_operation = OpCode::GetLocal(local.clone());
        }

        if can_assign && self.match_token(TokenType::Equal)? {
            let mut operations = self.expression()?;
            operations.push(Instruction::new(set_operation, line));
            return Ok(operations);
        }

        Ok(vec![Instruction::new(get_operation, line)])
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

    fn error_at(&mut self, token: &Token<'a>, message: &str) -> Result<(), String> {
        if self.panic_mode {
            return Ok(());
        }

        self.panic_mode = true;

        if token.token_type == TokenType::Error {
            return Ok(());
        }

        let error_string = format!(
            "[line {}] Error at {}: {}",
            token.line, token.lexeme, message
        );

        self.error = Some(error_string.clone());

        Err(error_string)
    }

    fn get_line(&mut self) -> Result<usize, String> {
        Ok(self.previous.ok_or("Cannot get current line")?.line)
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
}

struct ParseRule {
    prefix: Option<PrefixParseFn>,
    infix: Option<InfixParseFn>,
    precedence: Precedence,
}
