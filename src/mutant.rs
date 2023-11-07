use noirc_frontend::token::Token;
use std::{fmt, path::Path};

#[derive(Debug, Clone)]
pub struct Mutant<'a> {
    id: u32,
    token: Token,
    bytes: Vec<u8>,
    span: (u32, u32),
    src_path: &'a Path,
}

impl<'a> fmt::Display for Mutant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Id: {:?}, Token: {:?}, Bytes: {:?}, Span: {:?}, Source Path: {:?}",
            self.id,
            self.token,
            self.bytes,
            self.span,
            self.src_path.display()
        )
    }
}

impl<'a> Mutant<'a> {
    pub fn id(&self) -> u32 {
        self.id
    }

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
        self.src_path
    }

    pub fn start(&self) -> u32 {
        self.span.0
    }

    pub fn end(&self) -> u32 {
        self.span.1
    }
}

// consider processing a token stream with this function.
pub fn mutant_builder(id: u32, token: Token, span: (u32, u32), src_path: &Path) -> Option<Mutant> {
    match token {
        Token::Equal => Some(Mutant {
            id,
            token: Token::NotEqual,
            bytes: "!=".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::NotEqual => Some(Mutant {
            id,
            token: Token::Equal,
            bytes: "==".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Greater => Some(Mutant {
            id,
            token: Token::LessEqual,
            bytes: "<=".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::GreaterEqual => Some(Mutant {
            id,
            token: Token::Less,
            bytes: "<".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Less => Some(Mutant {
            id,
            token: Token::GreaterEqual,
            bytes: ">=".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::LessEqual => Some(Mutant {
            id,
            token: Token::Greater,
            bytes: ">".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Ampersand => Some(Mutant {
            id,
            token: Token::Pipe,
            bytes: "|".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Pipe => Some(Mutant {
            id,
            token: Token::Ampersand,
            bytes: "&".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Caret => Some(Mutant {
            id,
            token: Token::Ampersand,
            bytes: "&".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::ShiftLeft => Some(Mutant {
            id,
            token: Token::ShiftRight,
            bytes: ">>".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::ShiftRight => Some(Mutant {
            id,
            token: Token::ShiftLeft,
            bytes: "<<".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Plus => Some(Mutant {
            id,
            token: Token::Minus,
            bytes: "-".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Minus => Some(Mutant {
            id,
            token: Token::Plus,
            bytes: "+".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Star => Some(Mutant {
            id,
            token: Token::Slash,
            bytes: "/".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Slash => Some(Mutant {
            id,
            token: Token::Star,
            bytes: "*".as_bytes().to_vec(),
            span,
            src_path,
        }),
        Token::Percent => Some(Mutant {
            id,
            token: Token::Star,
            bytes: "*".as_bytes().to_vec(),
            span,
            src_path,
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
            id: 0,
            token: token.clone(),
            bytes: "==".as_bytes().to_vec(),
            span,
            src_path: &path,
        };

        // Test token method
        assert_eq!(mutant.token(), token);

        // Test string method
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
            id: 42,
            token: token.clone(),
            bytes: "+".as_bytes().to_vec(),
            span,
            src_path: &path,
        };

        // Test token method
        assert_eq!(mutant.token(), token);

        // Test string method
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
            id: 42,
            token: token.clone(),
            bytes: "*".as_bytes().to_vec(),
            span,
            src_path: &path,
        };

        // Test token method
        assert_eq!(mutant.token(), token);

        // Test string method
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::NotEqual);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Equal);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::LessEqual);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Less);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::GreaterEqual);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Greater);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Pipe);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Ampersand);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Ampersand);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Minus);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Plus);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Slash);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Star);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
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
        let id = 42;
        let mutant = mutant_builder(id, token, span, &path).unwrap();
        assert_eq!(mutant.token(), Token::Star);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "*");
        assert_eq!(mutant.start(), 0);
        assert_eq!(mutant.end(), 1);
        assert_eq!(mutant.path(), &path);
    }
}
