#[derive(Debug)]
pub enum TokenKind {
    Identifier(String),
    Text(String),
    Number(f64),

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftSBracket,
    RightSBracket,
    Plus,
    Minus,
    Star,
    Slash,
    Comma,
    Colon,
    SemiColon,
    Equals,
    Dot,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Continue,
    Break,
    Class,
    Else,
    False,
    True,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,

    EOF,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Identifier(s) => write!(f, "Identifier: {}", s),
            TokenKind::Number(n) => write!(f, "{}", n),
            TokenKind::Text(s) => write!(f, "String: \"{:?}\"", s),
            _ => write!(f, "{:?}", self),
        }
    }
}
