use std::{fmt, path::Path};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// <
    Less,
    /// <=
    LessEqual,
    /// >
    Greater,
    /// >=
    GreaterEqual,
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// %
    Percent,
    /// &
    Ampersand,
    /// ^
    Caret,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,
    /// |
    Pipe,
    /// ++
    Increment,
    /// --
    Decrement,
    // Bang,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpannedToken {
    token: Token,
    span: (u32, u32),
}

impl SpannedToken {
    pub fn new(token: Token, span: (u32, u32)) -> Self {
        Self { token, span }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn span(&self) -> (u32, u32) {
        self.span
    }

    pub fn set_span(&mut self, new_span: (u32, u32)) {
        self.span = new_span;
    }

    pub fn span_start(&self) -> u32 {
        self.span.0
    }

    pub fn span_end(&self) -> u32 {
        self.span.1
    }
}

#[derive(Debug, Clone)]
pub struct Mutant<'a> {
    id: u32,
    mutation: Token,
    bytes: Vec<u8>,
    span: (u32, u32),
    src_path: &'a Path,
    status: MutationStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MutationStatus {
    Pending,
    Survived,
    Killed,
}

impl<'a> fmt::Display for Mutant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Id: {:?}, Token: {:?}, Bytes: {:?}, Span: {:?}, Source Path: {:?}, Status: {:?}",
            self.id,
            self.mutation,
            self.bytes,
            self.span,
            self.src_path.display(),
            self.status,
        )
    }
}

impl<'a> Mutant<'a> {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn token(&self) -> Token {
        self.mutation.clone()
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

    pub fn span_start(&self) -> u32 {
        self.span.0
    }

    pub fn span_end(&self) -> u32 {
        self.span.1
    }

    pub fn status(&self) -> MutationStatus {
        self.status.clone()
    }

    // Method to update the status
    pub fn set_status(&mut self, new_status: MutationStatus) {
        self.status = new_status;
    }
}

pub fn token_regex_patterns<'a>() -> Vec<(&'a str, Token)> {
    vec![
        (r"<=", Token::LessEqual),
        (r"<", Token::Less),
        (r">=", Token::GreaterEqual),
        (r">", Token::Greater),
        (r"==", Token::Equal),
        (r"!=", Token::NotEqual),
        (r"\+", Token::Plus),
        (r"-", Token::Minus),
        (r"\*", Token::Star),
        (r"/", Token::Slash),
        (r"%", Token::Percent),
        (r"&", Token::Ampersand),
        (r"\^", Token::Caret),
        (r"<<", Token::ShiftLeft),
        (r">>", Token::ShiftRight),
        (r"\|", Token::Pipe),
        (r"\+\+", Token::Increment),
        (r"--", Token::Decrement),
    ]
}

pub fn token_mutation(token: Token) -> Option<Token> {
    match token {
        Token::Equal => Some(Token::NotEqual),
        Token::NotEqual => Some(Token::Equal),
        Token::Greater => Some(Token::LessEqual),
        Token::GreaterEqual => Some(Token::Less),
        Token::Less => Some(Token::GreaterEqual),
        Token::LessEqual => Some(Token::Greater),
        Token::Ampersand => Some(Token::Pipe),
        Token::Pipe => Some(Token::Ampersand),
        Token::Caret => Some(Token::Ampersand),
        Token::ShiftLeft => Some(Token::ShiftRight),
        Token::ShiftRight => Some(Token::ShiftLeft),
        Token::Plus => Some(Token::Minus),
        Token::Minus => Some(Token::Plus),
        Token::Star => Some(Token::Slash),
        Token::Slash => Some(Token::Star),
        Token::Percent => Some(Token::Star),
        Token::Increment => Some(Token::Decrement),
        Token::Decrement => Some(Token::Increment),
        _ => None,
    }
}

pub fn token_as_bytes<'a>(token: &Token) -> Option<&'a [u8]> {
    match token {
        Token::Equal => Some(b"=="),
        Token::NotEqual => Some(b"!="),
        Token::Less => Some(b"<"),
        Token::LessEqual => Some(b"<="),
        Token::Greater => Some(b">"),
        Token::GreaterEqual => Some(b">="),
        Token::Ampersand => Some(b"&"),
        Token::Pipe => Some(b"|"),
        Token::Caret => Some(b"^"),
        Token::ShiftLeft => Some(b"<<"),
        Token::ShiftRight => Some(b">>"),
        Token::Plus => Some(b"+"),
        Token::Minus => Some(b"-"),
        Token::Star => Some(b"*"),
        Token::Slash => Some(b"/"),
        Token::Percent => Some(b"%"),
        Token::Increment => Some(b"++"),
        Token::Decrement => Some(b"--"),
        _ => None,
    }
}

// consider processing a token stream with this function.
pub fn mutant_builder(id: u32, token: Token, span: (u32, u32), src_path: &Path) -> Option<Mutant> {
    let mutation = token_mutation(token.clone()).unwrap();
    match token {
        Token::Equal => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::NotEqual => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Greater => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::GreaterEqual => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Less => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::LessEqual => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Ampersand => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Pipe => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Caret => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::ShiftLeft => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::ShiftRight => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Plus => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Minus => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Star => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Slash => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Percent => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Increment => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span: (span.0, span.1 + 1),
            src_path,
            status: MutationStatus::Pending,
        }),
        Token::Decrement => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span: (span.0, span.1 + 1),
            src_path,
            status: MutationStatus::Pending,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use noirc_frontend::token::Token;
    use std::path::PathBuf;

    #[test]
    fn test_mutant_methods() {
        let path = PathBuf::from("test.noir");
        let token = Token::Equal;
        let span = (0, 1);
        let mutant = Mutant {
            id: 0,
            mutation: token.clone(),
            bytes: token_as_bytes(&token.clone()).unwrap().to_vec(),
            span,
            src_path: &path,
            status: MutationStatus::Pending,
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
        assert_eq!(mutant.span_start(), 0);

        // Test end method
        assert_eq!(mutant.span_end(), 1);

        // Test status method
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_methods_complex() {
        let path = PathBuf::from("complex/path/to/test.noir");
        let token = Token::Plus;
        let span = (10, 20);
        let mutant = Mutant {
            id: 42,
            mutation: token.clone(),
            bytes: token_as_bytes(&token.clone()).unwrap().to_vec(),
            span,
            src_path: &path,
            status: MutationStatus::Pending,
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
        assert_eq!(mutant.span_start(), 10);

        // Test end method
        assert_eq!(mutant.span_end(), 20);

        // Test status method
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
            mutation: token.clone(),
            bytes: token_as_bytes(&token.clone()).unwrap().to_vec(),
            span,
            src_path: &path,
            status: MutationStatus::Pending,
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
        assert_eq!(mutant.span_start(), 1000);

        // Test end method
        assert_eq!(mutant.span_end(), 2000);

        // Test status method
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
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
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), &path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }
}
