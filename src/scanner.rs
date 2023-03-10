use std::string::String;
use std::collections::HashMap;
use crate::TokenType::*;
use crate::scanner::LiteralValue::*;

fn get_keywords_map() -> HashMap<&'static str, TokenType> {
     HashMap::from([
        ("and", And),
        ("class", Class),
        ("else", Else),
        ("false", False),
        ("for", For),
        ("fun", Fun),
        ("if", If),
        ("if", If),
        ("or", Or),
        ("print", Print),
        ("return", Return),
        ("super", Super),
        ("this", This),
        ("true", True),
        ("var", Var),
        ("while", While),
     ])
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>
}

impl Scanner {
    pub fn new(_source: &str) -> Self {
        Self {
            source: _source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords_map(),
        }
        
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }
        self.tokens.push(Token{ token_type: Eof, lexeme: "".to_string(), literal: None, line_number: self.line});
        if errors.len() > 0 {
            let mut joined = "".to_string();
            errors.iter().map(|msg| {
                joined.push_str(&msg);
                joined.push_str("\n");
            });
            return Err(joined);
        }
        return Ok(self.tokens.clone());
    }

    fn is_at_end(self: &Self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();
        
        match c {
            '('=> self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token = if self.char_match('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(token);
            },
            '=' => {
                let token = if self.char_match('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(token);
            },
            '<' => {
                let token = if self.char_match('=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(token);
            },
            '>' => {
                let token = if self.char_match('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token);
            },
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            },
            '"' => self.string_check()?,
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            c => {
                if self.is_num(c) {
                    self.number()?;
                } else if self.is_alpha(c) {
                    self.indentifier_check();
                } else {
                    return Err(format!("Unrecognized Character at line {}: {}", self.line, c));
                } 
            },
        }
        Ok(())
    }

    fn indentifier_check(self: &mut Self) {
        while self.is_alpha_num(self.peek()) {
            self.advance();
        }
        let substring = &self.source[self.start..self.current];
        if let Some(&t_type) = self.keywords.get(substring) {
            self.add_token(t_type)
        } else {
            self.add_token(Identifier);
        }
        
    }

    fn is_alpha(self: &Self, c: char) -> bool {
        let ch = c as u8;
        (ch >= 'a' as u8 && ch <= 'z' as u8) || (ch >= 'A' as u8 && ch <= 'Z' as u8) || (c == '_')
    }

    fn is_alpha_num(self: &mut Self, c: char) -> bool {
        self.is_alpha(c) || self.is_num(c)
    }

    fn is_num(self: &mut Self, n: char) -> bool {
        return n as u8 >= '0' as u8 && n as u8 <= '9' as u8;
    }

    fn number(self: &mut Self) -> Result<(), String> {
        while self.is_num(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_num(self.peek_next()) {
            self.advance();

            while self.is_num(self.peek()) {
                self.advance();
            }
        }
        let substring = &self.source[self.start..self.current];
        let value = substring.parse::<f64>();

        match value {
            Ok(value) => self.add_token_lit(Number, Some(FloatValue(value))),
            Err(_) => return Err(format!("Could not parse: {}", substring)),
        }
        Ok(())
    }

    fn peek_next(self: &Self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        return c;
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        let mut text = "".to_string();
        self.source[self.start..self.current]
            .chars()
            .map(|c| text.push(c));

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_number: self.line
        });
    }

    fn char_match(self: &mut Self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn string_check(self: &mut Self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }

        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        
        self.add_token_lit(StringLit, Some(StringValue(value.to_string())));
        return Ok(());
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, 
    RightParen, 
    LeftBrace, 
    RightBrace,
    Comma,
    Dot, 
    Minus, 
    Plus, 
    Semicolon, 
    Slash, 
    Star,

    // One or two character tokens.
    Bang, 
    BangEqual,
    Equal, 
    EqualEqual,
    Greater, 
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier, 
    StringLit, 
    Number,

    // Keywords.
    And, 
    Class, 
    Else, 
    False, 
    Fun, 
    For, 
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

    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line_number: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}   


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(( ))";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, LeftParen);
        assert_eq!(scanner.tokens[1].token_type, LeftParen);
        assert_eq!(scanner.tokens[2].token_type, RightParen);
        assert_eq!(scanner.tokens[3].token_type, RightParen);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_string_literal() {
        let source = r#""ABC""#;
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, StringLit);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_num_lit() {
        let source = "123.123\n321.0\n5";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 4);
        for i in 0..3 {
            assert_eq!(scanner.tokens[1].token_type, Number);
        }
        match scanner.tokens[0].literal {
            Some(FloatValue(val)) => assert_eq!(val, 123.123),
            _ => panic!("Incorrect literal type"),
        }
        match scanner.tokens[1].literal {
            Some(FloatValue(val)) => assert_eq!(val, 321.0),
            _ => panic!("Incorrect literal type"),
        }
        match scanner.tokens[2].literal {
            Some(FloatValue(val)) => assert_eq!(val, 5.0),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_indentifier() {
        let source = "this_is_a_var = 12;";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, Identifier);
        assert_eq!(scanner.tokens[1].token_type, Equal);
        assert_eq!(scanner.tokens[2].token_type, Number);
        assert_eq!(scanner.tokens[3].token_type, Semicolon);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_keywords() {
        let source = "var this_is_a_var = 12;\n while true { print 3 };";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 13);
        assert_eq!(scanner.tokens[0].token_type, Var);
        assert_eq!(scanner.tokens[1].token_type, Identifier);
        assert_eq!(scanner.tokens[2].token_type, Equal);
        assert_eq!(scanner.tokens[3].token_type, Number);
        assert_eq!(scanner.tokens[4].token_type, Semicolon);
        assert_eq!(scanner.tokens[5].token_type, While);
        assert_eq!(scanner.tokens[6].token_type, True);
        assert_eq!(scanner.tokens[7].token_type, LeftBrace);
        assert_eq!(scanner.tokens[8].token_type, Print);
        assert_eq!(scanner.tokens[9].token_type, Number);
        assert_eq!(scanner.tokens[10].token_type, RightBrace);
        assert_eq!(scanner.tokens[11].token_type, Semicolon);
        assert_eq!(scanner.tokens[12].token_type, Eof);
    }
}