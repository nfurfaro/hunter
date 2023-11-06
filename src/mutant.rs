use noirc_frontend::token::Token;
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
pub fn mutant_builder(token: Token, span: (u32, u32), path: &Path) -> Option<Mutant> {
    let start = span.0;
    let end = span.1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use noirc_frontend::token::Token;
    use std::path::PathBuf;

    #[test]
    fn test_mutant_builder_equal() {
        let path = PathBuf::from("test.noir");
        let token = Token::Equal;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::NotEqual);
        assert_eq!(mutant.string(), "!=");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_not_equal() {
        let path = PathBuf::from("test.noir");
        let token = Token::NotEqual;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Equal);
        assert_eq!(mutant.string(), "==");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_greater() {
        let path = PathBuf::from("test.noir");
        let token = Token::Greater;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::LessEqual);
        assert_eq!(mutant.string(), "<=");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_greater_equal() {
        let path = PathBuf::from("test.noir");
        let token = Token::GreaterEqual;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Less);
        assert_eq!(mutant.string(), "<");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_less() {
        let path = PathBuf::from("test.noir");
        let token = Token::Less;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::GreaterEqual);
        assert_eq!(mutant.string(), ">=");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_less_equal() {
        let path = PathBuf::from("test.noir");
        let token = Token::LessEqual;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Greater);
        assert_eq!(mutant.string(), ">");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_ampersand() {
        let path = PathBuf::from("test.noir");
        let token = Token::Ampersand;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Pipe);
        assert_eq!(mutant.string(), "|");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_pipe() {
        let path = PathBuf::from("test.noir");
        let token = Token::Pipe;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Ampersand);
        assert_eq!(mutant.string(), "&");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_caret() {
        let path = PathBuf::from("test.noir");
        let token = Token::Caret;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Ampersand);
        assert_eq!(mutant.string(), "&");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_plus() {
        let path = PathBuf::from("test.noir");
        let token = Token::Plus;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Minus);
        assert_eq!(mutant.string(), "-");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_minus() {
        let path = PathBuf::from("test.noir");
        let token = Token::Minus;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Plus);
        assert_eq!(mutant.string(), "+");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_star() {
        let path = PathBuf::from("test.noir");
        let token = Token::Star;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Slash);
        assert_eq!(mutant.string(), "/");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_slash() {
        let path = PathBuf::from("test.noir");
        let token = Token::Slash;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Star);
        assert_eq!(mutant.string(), "*");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }

    #[test]
    fn test_mutant_builder_percent() {
        let path = PathBuf::from("test.noir");
        let token = Token::Percent;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Star);
        assert_eq!(mutant.string(), "*");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }
}
