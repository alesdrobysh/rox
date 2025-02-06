pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            print!("{:?} ", self.scan_token());
        }
    }

    fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.advance() {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            ';' => self.make_token(TokenType::Semicolon),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_expected('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_expected('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_expected('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_expected('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
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
                    self.make_token(TokenType::Error)
                }
            }
            '"' => self.scan_string(),
            c if c.is_digit(10) => self.scan_number(),
            c if self.is_id_start(c) => self.scan_identifier(),
            '\0' => self.make_token(TokenType::Eof),
            _ => self.make_token(TokenType::Error),
        }
    }

    fn is_id_start(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_id_part(&self, c: char) -> bool {
        self.is_id_start(c) || c.is_digit(10)
    }

    fn scan_identifier(&mut self) -> Token {
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
                return self.make_token(*token_type);
            }
        }

        self.make_token(TokenType::Identifier)
    }

    fn scan_number(&mut self) -> Token {
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

        self.make_token(TokenType::Number)
    }

    fn scan_string(&mut self) -> Token {
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
            return self.make_token(TokenType::Error);
        }

        self.advance();

        self.make_token(TokenType::String)
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

#[derive(Copy, Debug)]
enum TokenType {
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
    Eof,
}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        *self
    }
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
}
