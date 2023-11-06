use noirc_frontend::token::{SpannedToken, Token};
use std::path::Path;

pub struct Mutant<'a> {
    token: Token,
    string: String,
    span: (u32, u32),
    path: &'a Path,
}

impl<'a> Mutant<'a> {
    pub fn token(&self) -> Token {
        self.token.clone()
    }

    pub fn string(&self) -> String {
        self.string.clone()
    }

    pub fn span(&self) -> (u32, u32) {
        self.span
    }

    pub fn path(&self) -> &Path {
        self.path
    }

    pub fn start(&self) -> u32 {
        self.span.0
    }

    pub fn end(&self) -> u32 {
        self.span.1
    }

    pub fn string_as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}

// consider processing a token stream with this function.
pub fn mutant_builder(spanned_token: SpannedToken, path: &Path) -> Option<Mutant> {
    let token = spanned_token.token();
    let span = spanned_token.to_span();
    let start = span.start();
    let end = span.end();
    match token {
        Token::Equal => Some(Mutant {
            token: Token::NotEqual,
            string: "!=".to_string(),
            span: (start, end),
            path,
        }),
        Token::NotEqual => Some(Mutant {
            token: Token::Equal,
            string: "==".to_string(),
            span: (start, end),
            path,
        }),
        Token::Greater => Some(Mutant {
            token: Token::LessEqual,
            string: "<=".to_string(),
            span: (start, end),
            path,
        }),
        Token::GreaterEqual => Some(Mutant {
            token: Token::Less,
            string: "<".to_string(),
            span: (start, end),
            path,
        }),
        Token::Less => Some(Mutant {
            token: Token::GreaterEqual,
            string: ">=".to_string(),
            span: (start, end),
            path,
        }),
        Token::LessEqual => Some(Mutant {
            token: Token::Greater,
            string: ">".to_string(),
            span: (start, end),
            path,
        }),
        Token::Ampersand => Some(Mutant {
            token: Token::Pipe,
            string: "|".to_string(),
            span: (start, end),
            path,
        }),
        Token::Pipe => Some(Mutant {
            token: Token::Ampersand,
            string: "&".to_string(),
            span: (start, end),
            path,
        }),
        Token::Caret => Some(Mutant {
            token: Token::Ampersand,
            string: "&".to_string(),
            span: (start, end),
            path,
        }),
        Token::ShiftLeft => Some(Mutant {
            token: Token::ShiftRight,
            string: ">>".to_string(),
            span: (start, end),
            path,
        }),
        Token::ShiftRight => Some(Mutant {
            token: Token::ShiftLeft,
            string: "<<".to_string(),
            span: (start, end),
            path,
        }),
        Token::Plus => Some(Mutant {
            token: Token::Minus,
            string: "-".to_string(),
            span: (start, end),
            path,
        }),
        Token::Minus => Some(Mutant {
            token: Token::Plus,
            string: "+".to_string(),
            span: (start, end),
            path,
        }),
        Token::Star => Some(Mutant {
            token: Token::Slash,
            string: "/".to_string(),
            span: (start, end),
            path,
        }),
        Token::Slash => Some(Mutant {
            token: Token::Star,
            string: "*".to_string(),
            span: (start, end),
            path,
        }),
        Token::Percent => Some(Mutant {
            token: Token::Star,
            string: "*".to_string(),
            span: (start, end),
            path,
        }),
        _ => None,
    }
}
