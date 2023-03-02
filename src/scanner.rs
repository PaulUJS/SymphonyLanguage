pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    pub fn new(_source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_tokens()?;
        }
        self.tokens.push(Token{ token_type: Eof, lexeme: "".to_string(), literal: None, line_number: self.line});
        Ok(self.tokens)
    }

    pub fn is_at_end(self: &self) -> bool {
        self.current >= self.source.len() as u64
    }

    pub fn scan_token(self: &mut Self) -> Result<Token, String> {
        todo!()
    }
}

#[derive(Debug)]
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
    String, 
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

#[derive(Debug)]
pub enum LiteralValue {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: u64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line_number: u64) -> Self {
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