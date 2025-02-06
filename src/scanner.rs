pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_token(&mut self) -> Option<TokenType> {
        if self.is_at_end() {
            return None;
        }

        self.skip_whitespace();

        self.start = self.current;

        match self.advance() {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            '!' => {
                if self.match_expected('=') {
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_expected('=') {
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_expected('=') {
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                }
            }
            '>' => {
                if self.match_expected('=') {
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                }
            }
            '\n' => {
                self.line += 1;
                self.advance();
                self.scan_token()
            }
            '/' => {
                if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    self.scan_token()
                } else {
                    Some(TokenType::Error)
                }
            }
            '"' => Some(self.scan_string()),
            c if c.is_digit(10) => Some(self.scan_number()),
            c if self.is_id_start(c) => Some(self.scan_identifier()),
            '\0' => None,
            _ => Some(TokenType::Error),
        }
    }

    fn is_id_start(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_id_part(&self, c: char) -> bool {
        self.is_id_start(c) || c.is_digit(10)
    }

    fn scan_identifier(&mut self) -> TokenType {
        let keywords = [
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ];

        loop {
            if !self.is_id_part(self.peek()) {
                break;
            }

            self.advance();
        }

        let text = &self.source[self.start..self.current];

        for (keyword, token_type) in keywords.iter() {
            if text == *keyword {
                return *token_type;
            }
        }

        TokenType::Identifier
    }

    fn scan_number(&mut self) -> TokenType {
        loop {
            if !self.peek().is_digit(10) {
                break;
            }

            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            loop {
                if !self.peek().is_digit(10) {
                    break;
                }

                self.advance();
            }
        }

        TokenType::Number
    }

    fn scan_string(&mut self) -> TokenType {
        loop {
            if self.peek() == '"' {
                break;
            }

            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return TokenType::Error;
        }

        self.advance();

        TokenType::String
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn peek(&self) -> char {
        match self.source.chars().nth(self.current) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn peek_next(&self) -> char {
        match self.source.chars().nth(self.current + 1) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn match_expected(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        match self.source.chars().nth(self.current) {
            Some(c) => {
                if c != expected {
                    return false;
                }

                self.current += 1;
                true
            }
            None => false,
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        match self.source.chars().nth(self.current - 1) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
            .map(|token_type| self.make_token(token_type))
    }
}

#[derive(Copy, Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        *self
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
}
