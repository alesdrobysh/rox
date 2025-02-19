use crate::{
    chunk::OpCode,
    logger,
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    pub current: Option<Token<'a>>,
    pub previous: Option<Token<'a>>,
    pub had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    pub fn new(mut scanner: Scanner<'a>) -> Parser<'a> {
        let current = scanner.next();
        Parser {
            scanner,
            current,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn expression(&mut self) -> Result<Vec<OpCode>, String> {
        self.parse_precedence(Precedence::Assignment)
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) -> Result<Vec<OpCode>, String> {
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

        let mut operations = prefix(self)?;

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

            if infix.is_some() {
                logger::debug(&format!(
                    "Found infix operation for current token_type {:?}",
                    current.token_type
                ));
                let next_operations = infix.unwrap()(self)?;
                operations.extend(next_operations);
            } else {
                logger::debug(&format!("No infix rule found for {:?}", current.token_type));
            }
        }

        Ok(operations)
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), String> {
        match self.current.clone() {
            Some(token) if token.token_type == token_type => self.advance(),
            Some(token) => self.error_at(&token, message),
            None => self.error_at(
                &Token {
                    token_type: TokenType::Error,
                    lexeme: "",
                    line: 0,
                },
                "Unexpected end of file",
            ),
        }
    }

    pub fn advance(&mut self) -> Result<(), String> {
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

        if self.had_error {
            Err("Error".to_string())
        } else {
            Ok(())
        }
    }

    fn get_rule(&self, operator: TokenType) -> ParseRule {
        let grouping = Box::new(|parser: &mut Parser| parser.grouping());
        let unary = Box::new(|parser: &mut Parser| parser.unary());
        let binary = Box::new(|parser: &mut Parser| parser.binary());
        let number = Box::new(|parser: &mut Parser| parser.number());
        let literal = Box::new(|parser: &mut Parser| parser.literal());

        match operator {
            TokenType::LeftParen => ParseRule {
                prefix: Some(grouping),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Minus => ParseRule {
                prefix: Some(unary),
                infix: Some(binary),
                precedence: Precedence::Term,
            },
            TokenType::Plus => ParseRule {
                prefix: None,
                infix: Some(binary),
                precedence: Precedence::Term,
            },
            TokenType::Star | TokenType::Slash => ParseRule {
                prefix: None,
                infix: Some(binary),
                precedence: Precedence::Factor,
            },
            TokenType::Number => ParseRule {
                prefix: Some(number),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::True | TokenType::False | TokenType::Nil | TokenType::String => ParseRule {
                prefix: Some(literal),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Bang => ParseRule {
                prefix: Some(unary),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::EqualEqual | TokenType::BangEqual => ParseRule {
                prefix: None,
                infix: Some(binary),
                precedence: Precedence::Equality,
            },
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => ParseRule {
                prefix: None,
                infix: Some(binary),
                precedence: Precedence::Comparison,
            },
            _ => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        }
    }

    fn grouping(&mut self) -> Result<Vec<OpCode>, String> {
        logger::debug("grouping");

        let expression = self.expression()?;

        self.consume(
            TokenType::RightParen,
            "Expect ')' after grouping expression",
        )?;

        Ok(expression)
    }

    fn binary(&mut self) -> Result<Vec<OpCode>, String> {
        logger::debug("binary");

        let token_type = self
            .previous
            .ok_or("Expected binary operator, found nothing")?
            .token_type;
        let precedence = self.get_rule(token_type).precedence.next().ok_or(format!(
            "Can not determine precedence for token: {:?}",
            token_type
        ))?;
        let mut operations = self.parse_precedence(precedence)?;

        match token_type {
            TokenType::Plus => operations.push(OpCode::Add),
            TokenType::Minus => operations.push(OpCode::Subtract),
            TokenType::Star => operations.push(OpCode::Multiply),
            TokenType::Slash => operations.push(OpCode::Divide),
            TokenType::EqualEqual => operations.push(OpCode::Equal),
            TokenType::BangEqual => {
                operations.push(OpCode::Equal);
                operations.push(OpCode::Not);
            }
            TokenType::Greater => operations.push(OpCode::Greater),
            TokenType::GreaterEqual => {
                operations.push(OpCode::Less);
                operations.push(OpCode::Not);
            }
            TokenType::Less => operations.push(OpCode::Less),
            TokenType::LessEqual => {
                operations.push(OpCode::Greater);
                operations.push(OpCode::Not);
            }
            _ => {
                return Err(format!("Unexpected binary operator type: {:?}", token_type));
            }
        }

        Ok(operations)
    }

    fn unary(&mut self) -> Result<Vec<OpCode>, String> {
        logger::debug("unary");

        let previous = self
            .previous
            .ok_or("No operand found when parsing unary expression")?;

        let mut operations = self.parse_precedence(Precedence::Unary)?;

        match previous.token_type {
            TokenType::Minus => operations.push(OpCode::Negate),
            TokenType::Bang => operations.push(OpCode::Not),
            _ => {
                return Err(format!(
                    "Unexpected unary operator type: {:?}",
                    previous.token_type
                ));
            }
        };

        Ok(operations)
    }

    fn number(&mut self) -> Result<Vec<OpCode>, String> {
        logger::debug("number");

        self.previous
            .ok_or("Expected number when parsing number, found nothing")?
            .lexeme
            .parse::<f64>()
            .map(|value| vec![OpCode::Value(Value::Number(value))])
            .map_err(|e| e.to_string())
    }

    fn literal(&mut self) -> Result<Vec<OpCode>, String> {
        logger::debug("literal");

        let previous = self.previous.ok_or("Expected literal, found nothing")?;
        let lexeme = previous.lexeme;

        match previous.token_type {
            TokenType::True | TokenType::False => Ok(vec![OpCode::Value(Value::Bool(
                previous.token_type == TokenType::True,
            ))]),
            TokenType::Nil => Ok(vec![OpCode::Value(Value::Nil)]),
            TokenType::String => Ok(vec![OpCode::Value(Value::String(lexeme.to_string()))]),
            _ => Err(format!(
                "Unexpected literal type: {:?}",
                previous.token_type
            )),
        }
    }

    fn error_at(&mut self, token: &Token<'a>, message: &str) -> Result<(), String> {
        if self.panic_mode {
            return Ok(());
        }

        self.panic_mode = true;

        if token.token_type == TokenType::Error {
            return Ok(());
        }

        self.had_error = true;

        Err(format!(
            "[line {}] Error at {}: {}",
            token.line, token.lexeme, message
        ))
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

type ParseFn = Box<dyn Fn(&mut Parser) -> Result<Vec<OpCode>, String>>;

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}
