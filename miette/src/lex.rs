pub struct Tokens {
    tokens: std::collections::VecDeque<Token>,
    contents: std::collections::VecDeque<char>,
    current_line: usize,
    current_token: Option<char>,
}

impl Tokens {
    pub fn new(file_name: String) -> Tokens {
        let contents: String = match std::fs::read_to_string(file_name) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read the file, please check the file path");
                return tokens.tokens;
            }
        };

        Tokens{
            tokens: std::collections::VecDeque::new(),
            current_line: 1,
            current_token: None,
        }
    }

    pub fn advance(&mut self) -> bool {
        self.current_token = self.tokens.pop_front();
        
        if self.current_token.is_some() {
            if self.current_token.unwrap() == '\n' {
                self.current_line = self.current_line + 1;
                self.advance();
            }
            return true;
        }

        false
    }
}

#[derive(Debug)]
pub struct Token {
    id: usize,
    kind: TokenKind,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(id: usize, kind: TokenKind, lexeme: String, line: usize) -> Self {
        Token {
            id,
            kind,
            lexeme,
            line,
        }
    }

    pub fn add_token(
        tokens: &mut std::collections::VecDeque<Token>,
        kind: TokenKind,
        lexeme: String,
        line: usize,
    ) {
        tokens.push_back(Token::new(tokens.len(), kind, lexeme, line))
    }
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

// Read in the file and parse for tokens
pub fn scan_tokens(file_name: String) -> std::collections::VecDeque<Token> {
    let tokens: Tokens = Tokens::new();



    let mut contents: std::collections::VecDeque<char> = contents.chars().collect();

    // Collects chars to be processes as a single token lexeme
    let mut buffer: std::collections::VecDeque<char> = std::collections::VecDeque::new();
    // Flag if currently iterating through a string "....."
    let mut is_text: bool = false;

    loop {
        let current: char = match contents.pop_front() {
            Some(c) => c,
            None => break
        }

        match current {
            '"' => {
                if is_text {
                    // end quote
                    continue;
                } else {
                    // start quote
                    is_text = true;
                    continue;
                }
            },
            '=' => {
                let peek:char = match contents.front() {
                    Some(c) => c,
                    None => 
                }

                if peek == '=' {
                    Token::add_token(&mut tokens, TokenKind::EqualEqual, current.to_string(), current_line);
                } else {
                    Token::add_token(&mut tokens,TokenKind::Equal, current.to_string(), current_line);
                }
            }
        }

    }

    tokens
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
