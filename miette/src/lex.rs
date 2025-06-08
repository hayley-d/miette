use crate::error::MietteError;

pub struct Tokens {
    tokens: std::collections::VecDeque<Token>,
    contents: std::collections::VecDeque<char>,
    current_line: usize,
    current_token: Option<char>,
    token_count: usize,
}

impl Tokens {
    pub fn new(file_name: String) -> Result<Tokens, Box<dyn std::error::Error>> {
        let contents: String = match std::fs::read_to_string(file_name) {
            Ok(s) => s,
            Err(_) => {
                return Err(Box::new(MietteError::new(
                    "Failed to read the file contents".to_string(),
                )));
            }
        };

        Ok(Tokens {
            tokens: std::collections::VecDeque::new(),
            contents: contents.chars().collect(),
            current_line: 0,
            current_token: None,
            token_count: 0,
        })
    }

    pub fn advance(&mut self) -> Option<char> {
        self.current_token = self.contents.pop_front();

        if self.current_token.is_some() {
            self.token_count += 1;

            if self.current_token.unwrap() == '\n' {
                self.current_line += 1;
                self.advance();
            }

            return self.current_token;
        }

        self.current_token
    }

    pub fn peek(&mut self) -> Option<&char> {
        let temp: Option<&char> = self.contents.front();
        temp
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
pub fn scan_tokens(
    file_name: String,
) -> Result<std::collections::VecDeque<Token>, Box<dyn std::error::Error>> {
    let mut tokens: Tokens = Tokens::new(file_name)?;

    // Collects chars to be processes as a single token lexeme
    let mut buffer: std::collections::VecDeque<char> = std::collections::VecDeque::new();

    // Flag if currently iterating through a string "....."
    let mut is_text: bool = false;
    let mut is_number: bool = false;
    let mut is_identifier: bool = false;

    while let Some(current) = tokens.advance() {
        match current {
            '"' => {
                if is_text {
                    // end quote
                    continue;
                } else {
                    // start quote
                    is_text = true;
                    buffer = std::collections::VecDeque::new();
                    while let Some(peek) = tokens.peek() {
                        // If not the closing " continue
                        if *peek != '"' || *peek != '\\' {
                            buffer.push_back(tokens.advance().unwrap());
                        } else if *peek == '\\' {
                            // ignore the escape char
                            tokens.advance();
                            let string_current = match tokens.advance() {
                                Some(c) => c,
                                None => {
                                    return Err(Box::new(MietteError::new("Tokenize Error: Expected token after escape character \\ but got nothing".to_string())));
                                }
                            };
                            buffer.push_back(string_current);
                        } else {
                            // closing "
                            is_text = false;
                            let temp: String = buffer.clone().into_iter().collect();
                            Token::add_token(
                                &mut tokens.tokens,
                                TokenKind::Text(temp.clone()),
                                temp,
                                tokens.current_line,
                            );
                            buffer.clear();
                            break;
                        }
                    }
                    continue;
                }
            }
            '=' => {
                let peek: &char = match tokens.peek() {
                    Some(c) => c,
                    None => {
                        // Equals with nothing after
                        return Err(Box::new(MietteError::new(
                            "Lexer Error: Expected token after equals but got nothing".to_string(),
                        )));
                    }
                };

                if *peek == '=' {
                    // Double equals '=='
                    Token::add_token(
                        &mut tokens.tokens,
                        TokenKind::EqualEqual,
                        "==".to_string(),
                        tokens.current_line,
                    );
                } else {
                    Token::add_token(
                        &mut tokens.tokens,
                        TokenKind::Equal,
                        "=".to_string(),
                        tokens.current_line,
                    );
                }
            } // end equal '='
            '>' => {
                let peek: &char = match tokens.peek() {
                    Some(c) => c,
                    None => {
                        // Gtreater with nothing after
                        return Err(Box::new(MietteError::new(
                            "Lexer Error: Expected token after greater than token but got nothing"
                                .to_string(),
                        )));
                    }
                };

                if *peek == '=' {
                    // Greater equals '>='
                    Token::add_token(
                        &mut tokens.tokens,
                        TokenKind::GreaterEqual,
                        ">=".to_string(),
                        tokens.current_line,
                    );
                } else {
                    Token::add_token(
                        &mut tokens.tokens,
                        TokenKind::Greater,
                        ">".to_string(),
                        tokens.current_line,
                    );
                }
            } // end greater '>'
            '<' => {
                let peek: &char = match tokens.peek() {
                    Some(c) => c,
                    None => {
                        // Less than with nothing after
                        return Err(Box::new(MietteError::new(
                            "Lexer Error: Expected token after Less Than token but got nothing"
                                .to_string(),
                        )));
                    }
                };

                if *peek == '=' {
                    // Less than equals '<='
                    Token::add_token(
                        &mut tokens.tokens,
                        TokenKind::LessEqual,
                        "<=".to_string(),
                        tokens.current_line,
                    );
                } else {
                    Token::add_token(
                        &mut tokens.tokens,
                        TokenKind::Less,
                        "<".to_string(),
                        tokens.current_line,
                    );
                }
            } // end less '<'
            '!' => {
                let peek: &char = match tokens.peek() {
                    Some(c) => c,
                    None => {
                        // Bang than with nothing after
                        return Err(Box::new(MietteError::new(
                            "Lexer Error: Expected token after bang token but got nothing"
                                .to_string(),
                        )));
                    }
                };

                if *peek == '=' {
                    // bang than equals '!='
                    Token::add_token(
                        &mut tokens.tokens,
                        TokenKind::BangEqual,
                        "!=".to_string(),
                        tokens.current_line,
                    );
                } else {
                    Token::add_token(
                        &mut tokens.tokens,
                        TokenKind::Bang,
                        "!".to_string(),
                        tokens.current_line,
                    );
                }
            } // end bang '!'
            '\n' => {
                let _peek: &char = match tokens.peek() {
                    Some(c) => c,
                    None => {
                        // new line than with nothing after
                        Token::add_token(
                            &mut tokens.tokens,
                            TokenKind::EOF,
                            "EOF".to_string(),
                            tokens.current_line,
                        );
                        continue;
                    }
                };
                continue;
            } // end new line '\n'
            c if c.is_ascii_digit() || c == '.' => {
                if is_number {
                } else {
                    if is_text {
                        // Part of a user string
                    } else {
                        // Can either be part of an identifier or number
                        if is_identifier {
                            buffer.push_back(c);
                            continue;
                        } else {
                            // Probably start of a number
                        }
                    }
                }
            } // end si digit
            '{' => {
                    let _peek: &char = match tokens.peek() {
                    Some(c) => c,
                    None => {
                        // Bracket with nothing after
                        return Err(Box::new(MietteError::new(
                            "Lexer Error: Expected token after left bracket '{' token but got nothing"
                                .to_string(),
                        )));
                    }
                };

                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::LeftBracket,
                    "{{".to_string(),
                    tokens.current_line,
                );
 
            }
            '}' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::RightBracket,
                    "}}".to_string(),
                    tokens.current_line,
                );
            } // end right bracket
            '[' => {
                    let _peek: &char = match tokens.peek() {
                    Some(c) => c,
                    None => {
                        // Square Bracket with nothing after
                        return Err(Box::new(MietteError::new(
                            "Lexer Error: Expected token after left square bracket '[' token but got nothing"
                                .to_string(),
                        )));
                    }
                };

                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::LeftSBracket,
                    "[".to_string(),
                    tokens.current_line,
                );
            } // end left square bracket
            ']' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::RightSBracket,
                    "]".to_string(),
                    tokens.current_line,
                );
            } // end right square bracket
            '(' => {
                    let _peek: &char = match tokens.peek() {
                    Some(c) => c,
                    None => {
                        // Parenthesis with nothing after
                        return Err(Box::new(MietteError::new(
                            "Lexer Error: Expected token after left parenthesis '(' token but got nothing"
                                .to_string(),
                        )));
                    }
                };

                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::LeftParen,
                    "(".to_string(),
                    tokens.current_line,
                );
            } // end left parenthesis
            ')' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::RightParen,
                    ")".to_string(),
                    tokens.current_line,
                );
            } // end right parenthesis
            '+' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::Plus,
                    "+".to_string(),
                    tokens.current_line,
                );
            } // end plus
            '-' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::Minus,
                    "-".to_string(),
                    tokens.current_line,
                );
            } // end minus
            '/' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::Slash,
                    "/".to_string(),
                    tokens.current_line,
                );
            } // end slash
            '*' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::Star,
                    ")".to_string(),
                    tokens.current_line,
                );
            } // end star
            ',' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::Comma,
                    ",".to_string(),
                    tokens.current_line,
                );
            } // end star
            ':' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::Colon,
                    ":".to_string(),
                    tokens.current_line,
                );
            } // end star
            ';' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::SemiColon,
                    ";".to_string(),
                    tokens.current_line,
                );
            } // end semi colon 
            '.' => {
                Token::add_token(
                    &mut tokens.tokens,
                    TokenKind::Dot,
                    ".".to_string(),
                    tokens.current_line,
                );
            } // end dot 

            _ => {
                continue;
            }
        };
    }

    Ok(tokens.tokens)
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
