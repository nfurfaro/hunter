use noirc_frontend::token::Token;
use std::path::Path;

pub struct Mutant<'a> {
    token: Token,
    bytes: Vec<u8>,
    span: (u32, u32),
    path: &'a Path,
}

impl<'a> Mutant<'a> {
    pub fn token(&self) -> Token {
        self.token.clone()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
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
}

// consider processing a token stream with this function.
pub fn mutant_builder(token: Token, span: (u32, u32), path: &Path) -> Option<Mutant> {
    let start = span.0;
    let end = span.1;
    match token {
        Token::Equal => Some(Mutant {
            token: Token::NotEqual,
            bytes: "!=".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::NotEqual => Some(Mutant {
            token: Token::Equal,
            bytes: "==".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Greater => Some(Mutant {
            token: Token::LessEqual,
            bytes: "<=".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::GreaterEqual => Some(Mutant {
            token: Token::Less,
            bytes: "<".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Less => Some(Mutant {
            token: Token::GreaterEqual,
            bytes: ">=".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::LessEqual => Some(Mutant {
            token: Token::Greater,
            bytes: ">".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Ampersand => Some(Mutant {
            token: Token::Pipe,
            bytes: "|".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Pipe => Some(Mutant {
            token: Token::Ampersand,
            bytes: "&".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Caret => Some(Mutant {
            token: Token::Ampersand,
            bytes: "&".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::ShiftLeft => Some(Mutant {
            token: Token::ShiftRight,
            bytes: ">>".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::ShiftRight => Some(Mutant {
            token: Token::ShiftLeft,
            bytes: "<<".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Plus => Some(Mutant {
            token: Token::Minus,
            bytes: "-".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Minus => Some(Mutant {
            token: Token::Plus,
            bytes: "+".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Star => Some(Mutant {
            token: Token::Slash,
            bytes: "/".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Slash => Some(Mutant {
            token: Token::Star,
            bytes: "*".as_bytes().to_vec(),
            span: (start, end),
            path,
        }),
        Token::Percent => Some(Mutant {
            token: Token::Star,
            bytes: "*".as_bytes().to_vec(),
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
    fn test_mutant_methods() {
        let path = PathBuf::from("test.noir");
        let token = Token::Equal;
        let span = (0, 1);
        let mutant = Mutant {
            token: token.clone(),
            bytes: "==".as_bytes().to_vec(),
            span,
            path: &path,
        };

        // Test token method
        assert_eq!(mutant.token(), token);

        // Test string method
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "==");

        // Test span method
        assert_eq!(mutant.span(), span);

        // Test path method
        assert_eq!(mutant.path(), &path);

        // Test start method
        assert_eq!(mutant.start(), 0);

        // Test end method
        assert_eq!(mutant.end(), 1);
    }

    #[test]
    fn test_mutant_methods_complex() {
        let path = PathBuf::from("complex/path/to/test.noir");
        let token = Token::Plus;
        let span = (10, 20);
        let mutant = Mutant {
            token: token.clone(),
            bytes: "+".as_bytes().to_vec(),
            span,
            path: &path,
        };

        // Test token method
        assert_eq!(mutant.token(), token);

        // Test string method
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "+");

        // Test span method
        assert_eq!(mutant.span(), span);

        // Test path method
        assert_eq!(mutant.path(), &path);

        // Test start method
        assert_eq!(mutant.start(), 10);

        // Test end method
        assert_eq!(mutant.end(), 20);
    }

    #[test]
    fn test_mutant_methods_extreme() {
        let path = PathBuf::from(
            "extremely/long/and/complex/path/to/the/test/file/for/testing/purposes.noir",
        );
        let token = Token::Star;
        let span = (1000, 2000);
        let mutant = Mutant {
            token: token.clone(),
            bytes: "*".as_bytes().to_vec(),
            span,
            path: &path,
        };

        // Test token method
        assert_eq!(mutant.token(), token);

        // Test string method
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "*");

        // Test span method
        assert_eq!(mutant.span(), span);

        // Test path method
        assert_eq!(mutant.path(), &path);

        // Test start method
        assert_eq!(mutant.start(), 1000);

        // Test end method
        assert_eq!(mutant.end(), 2000);
    }

    #[test]
    fn test_mutant_builder_equal() {
        let path = PathBuf::from("test.noir");
        let token = Token::Equal;
        let span = (0, 1);
        let mutant = mutant_builder(token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::NotEqual);
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "!=");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "==");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "<=");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "<");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, ">=");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, ">");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "|");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "&");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "&");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "-");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "+");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "/");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "*");
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
        let bytes_str = String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "*");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }
}
