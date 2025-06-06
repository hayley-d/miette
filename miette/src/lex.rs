#[derive(Debug)]
pub struct Token {
    id: u128,
    kind: TokenKind,
    lexeme: String,
    line: u128,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<10} {:<15} {:<15} {:<10}",
            self.id, self.kind, self.lexeme, self.line
        )
    }
}

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
            TokenKind::Identifier(name) => write!(f, "Identifier({})", name),
            TokenKind::Text(s) => write!(f, "String(\"{}\")", s),
            TokenKind::Number(n) => write!(f, "Number({})", n),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::BangEqual => write!(f, "!="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBracket => write!(f, "{{"),
            TokenKind::RightBracket => write!(f, "}}"),
            TokenKind::LeftSBracket => write!(f, "["),
            TokenKind::RightSBracket => write!(f, "]"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::And => write!(f, "and"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Fun => write!(f, "fun"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::While => write!(f, "while"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Var => write!(f, "var"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::Super => write!(f, "super"),
            TokenKind::This => write!(f, "this"),
            TokenKind::Nil => write!(f, "nil"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Print => write!(f, "print"),
            TokenKind::Break => write!(f, "continue"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::EOF => write!(f, "<EOF>"),
        }
    }
}
