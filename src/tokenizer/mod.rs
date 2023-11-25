use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum TokenType {
    Error,
    Eof,

    Comment,

    LeftParen,  // `(`
    RightParen, // `)`
    LeftBrace,  // `{`
    RightBrace, // `}`

    Comma,        // `,`
    Dot,          // `.`
    Minus,        // `-`
    Plus,         // `+`
    Colon,        // `:`
    Semicolon,    // `;`
    Slash,        // `/`
    Star,         // `*`
    Bang,         // `!`
    Equal,        // `=`
    EqualEqual,   // `==`
    Greater,      // `>`
    GreaterEqual, // `>=`
    Less,         // `<`
    LessEqual,    // `<=`

    Identifier,
    String,
    Number,

    If,
    Else,
    While,
    For,
    False,
    True,
    Const,
    Let,
    Function,
    Class,
    Null,
    Return,
    Break,
    Continue,
}

fn create_keywords_hashmap() -> HashMap<&'static str, TokenType> {
    let mut keywords = HashMap::with_capacity(16);
    keywords.insert("if", TokenType::If);
    keywords.insert("else", TokenType::Else);
    keywords.insert("while", TokenType::While);
    keywords.insert("for", TokenType::For);
    keywords.insert("false", TokenType::False);
    keywords.insert("true", TokenType::True);
    keywords.insert("const", TokenType::Const);
    keywords.insert("let", TokenType::Let);
    keywords.insert("function", TokenType::Function);
    keywords.insert("class", TokenType::Class);
    keywords.insert("null", TokenType::Null);
    keywords.insert("return", TokenType::Return);
    keywords.insert("break", TokenType::Break);
    keywords.insert("continue", TokenType::Continue);
    return keywords;
}

#[derive(Copy, Clone, Debug)]
pub struct Token<'source> {
    pub kind: TokenType,
    pub line: usize,
    pub lexeme: &'source str,
}

pub struct Tokenizer<'source> {
    keywords: HashMap<&'static str, TokenType>,
    code: &'source str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'source> Tokenizer<'source> {
    pub fn new(code: &'source str) -> Tokenizer {
        let keywords = create_keywords_hashmap();
        Tokenizer {
            keywords,
            code,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenizer(&mut self) -> Vec<Token<'source>> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.scan_token();
            // println!("{:?}", &tok);
            tokens.push(tok);
            if tok.kind == TokenType::Eof {
                break;
            }
        }
        tokens
    }

    fn scan_token(&mut self) -> Token<'source> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }
        match self.advance() {
            b'(' => self.make_token(TokenType::LeftParen),
            b')' => self.make_token(TokenType::RightParen),
            b'{' => self.make_token(TokenType::LeftBrace),
            b'}' => self.make_token(TokenType::RightBrace),
            b':' => self.make_token(TokenType::Colon),
            b';' => self.make_token(TokenType::Semicolon),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b'/' if self.matches(b'/') => self.comment(),
            b'/' => self.make_token(TokenType::Slash),
            b'*' => self.make_token(TokenType::Star),
            b'!' => self.make_token(TokenType::Bang),
            b'=' if self.matches(b'=') => self.make_token(TokenType::EqualEqual),
            b'=' => self.make_token(TokenType::Equal),
            b'<' if self.matches(b'=') => self.make_token(TokenType::LessEqual),
            b'<' => self.make_token(TokenType::Less),
            b'>' if self.matches(b'=') => self.make_token(TokenType::GreaterEqual),
            b'>' => self.make_token(TokenType::Greater),
            b'"' => self.string(),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.code.len()
    }

    fn lexeme(&self) -> &'source str {
        &self.code[self.start..self.current]
    }

    fn make_token(&self, kind: TokenType) -> Token<'source> {
        Token {
            kind,
            lexeme: self.lexeme(),
            line: self.line,
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            0
        } else {
            self.code.as_bytes()[self.current]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.current > self.code.len() - 2 {
            b'\0'
        } else {
            self.code.as_bytes()[self.current + 1]
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'static> {
        Token {
            kind: TokenType::Error,
            lexeme: message,
            line: self.line,
        }
    }

    fn advance(&mut self) -> u8 {
        let char = self.peek();
        self.current += 1;
        char
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => return,
            }
        }
    }

    fn comment(&mut self) -> Token<'source> {
        while self.peek() != b'\n' && !self.is_at_end() {
            self.advance();
        }
        Token {
            kind: TokenType::Comment,
            lexeme: self.lexeme(),
            line: self.line,
        }
    }

    fn string(&mut self) -> Token<'source> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error_token("Unterminated string.")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn number(&mut self) -> Token<'source> {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token<'source> {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        self.keywords
            .get(self.lexeme())
            .cloned()
            .unwrap_or(TokenType::Identifier)
    }
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_is_digit() {
        assert_eq!(is_digit(b'0'), true);
        assert_eq!(is_digit(b'9'), true);
    }
    #[test]
    fn alpha_is_not_digit() {
        assert_eq!(is_digit(b'a'), false);
        assert_eq!(is_digit(b'A'), false);
    }

    #[test]
    fn tokenizer_comment() {
        let code = "// it is a comment";
        let mut scanner = Tokenizer::new(&code);
        let tok = scanner.scan_token();
        assert_eq!(tok.kind, TokenType::Comment);
        assert_eq!(tok.lexeme, code);
    }
}
