use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub struct MetaToken {
    token: Token,
    span: (u32, u32),
    src: Box<PathBuf>,
    id: u32,
}

impl MetaToken {
    pub fn new(token: Token, span: (u32, u32), src: Box<PathBuf>, id: u32) -> Self {
        Self {
            token,
            span,
            src,
            id,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn span(&self) -> (u32, u32) {
        self.span
    }

    pub fn src(&self) -> &PathBuf {
        &self.src
    }

    pub fn id(&self) -> u32 {
        self.id
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
    /// +=
    PlusEquals,
    /// -=
    MinusEquals,
    /// *=
    StarEquals,
    /// /=
    SlashEquals,
    /// %=
    PercentEquals,
    /// &=
    AmpersandEquals,
    /// |=
    PipeEquals,
    /// ^=
    CaretEquals,
    /// <<=
    ShiftLeftEquals,
    /// >>=
    ShiftRightEquals,
    /// ||
    DoublePipe,
    /// &&
    DoubleAmpersand,
    /// !
    // @todo finish this ! need to work on replacing bytes in utils
    Bang,
    /// empty
    Void,
}

pub fn token_patterns() -> Vec<&'static str> {
    vec![
        r" (==) ",
        r" (!=) ",
        r" (<) ",
        r" (<=) ",
        r" (>) ",
        r" (>=) ",
        r" (&) ",
        r" (\|) ",
        r" (^) ",
        r" (<<) ",
        r" (>>) ",
        r" (\+) ",
        r" (-) ",
        r" (\*) ",
        r" (/) ",
        r" (%) ",
        r" (\+\+) ",
        r" (--) ",
        r" (\+=) ",
        r" (-=) ",
        r" (\*=) ",
        r" (/=) ",
        r" (%=) ",
        r" (&=) ",
        r" (\|=) ",
        r" (^=) ",
        r" (<<=) ",
        r" (>>=) ",
        r" (\|\|) ",
        r" (&&) ",
        r" (!)",
    ]
}

pub fn raw_string_as_token(raw: &str) -> Option<Token> {
    match raw {
        r"==" => Some(Token::Equal),
        r"!=" => Some(Token::NotEqual),
        r"<" => Some(Token::Less),
        r"<=" => Some(Token::LessEqual),
        r">" => Some(Token::Greater),
        r">=" => Some(Token::GreaterEqual),
        r"&" => Some(Token::Ampersand),
        r"|" => Some(Token::Pipe),
        r"^" => Some(Token::Caret),
        r"<<" => Some(Token::ShiftLeft),
        r">>" => Some(Token::ShiftRight),
        r"+" => Some(Token::Plus),
        r"-" => Some(Token::Minus),
        r"*" => Some(Token::Star),
        r"/" => Some(Token::Slash),
        r"%" => Some(Token::Percent),
        r"++" => Some(Token::Increment),
        r"--" => Some(Token::Decrement),
        r"+=" => Some(Token::PlusEquals),
        r"-=" => Some(Token::MinusEquals),
        r"*=" => Some(Token::StarEquals),
        r"/=" => Some(Token::SlashEquals),
        r"%=" => Some(Token::PercentEquals),
        r"&=" => Some(Token::AmpersandEquals),
        r"|=" => Some(Token::PipeEquals),
        r"^=" => Some(Token::CaretEquals),
        r"<<=" => Some(Token::ShiftLeftEquals),
        r">>=" => Some(Token::ShiftRightEquals),
        r"||" => Some(Token::DoublePipe),
        r"&&" => Some(Token::DoubleAmpersand),
        r"!" => Some(Token::Bang),
        _ => None,
    }
}

pub fn token_transformer(token: Token) -> Option<Token> {
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
        Token::PlusEquals => Some(Token::MinusEquals),
        Token::MinusEquals => Some(Token::PlusEquals),
        Token::StarEquals => Some(Token::SlashEquals),
        Token::SlashEquals => Some(Token::StarEquals),
        Token::PercentEquals => Some(Token::StarEquals),
        Token::AmpersandEquals => Some(Token::PipeEquals),
        Token::PipeEquals => Some(Token::AmpersandEquals),
        Token::CaretEquals => Some(Token::AmpersandEquals),
        Token::ShiftLeftEquals => Some(Token::ShiftRightEquals),
        Token::ShiftRightEquals => Some(Token::ShiftLeftEquals),
        Token::DoublePipe => Some(Token::DoubleAmpersand),
        Token::DoubleAmpersand => Some(Token::DoublePipe),
        Token::Bang => Some(Token::Void),
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
        Token::PlusEquals => Some(b"+="),
        Token::MinusEquals => Some(b"-="),
        Token::StarEquals => Some(b"*="),
        Token::SlashEquals => Some(b"/="),
        Token::PercentEquals => Some(b"%="),
        Token::AmpersandEquals => Some(b"&="),
        Token::PipeEquals => Some(b"|="),
        Token::CaretEquals => Some(b"^="),
        Token::ShiftLeftEquals => Some(b"<<="),
        Token::ShiftRightEquals => Some(b">>="),
        Token::DoublePipe => Some(b"||"),
        Token::DoubleAmpersand => Some(b"&&"),
        Token::Bang => Some(b"!"),
        Token::Void => Some(b""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::mutator::{mutant_builder, MutationStatus};
    use std::path::PathBuf;

    #[test]
    fn test_raw_string_as_token_equal() {
        let bytes = "==";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Equal);
    }

    #[test]
    fn test_raw_string_as_token_not_equal() {
        let bytes = "!=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::NotEqual);
    }

    #[test]
    fn test_raw_string_as_token_less() {
        let bytes = "<";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Less);
    }

    #[test]
    fn test_raw_string_as_token_less_equal() {
        let bytes = "<=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::LessEqual);
    }

    #[test]
    fn test_raw_string_as_token_greater() {
        let bytes = ">";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Greater);
    }

    #[test]
    fn test_raw_string_as_token_greater_equal() {
        let bytes = ">=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::GreaterEqual);
    }

    #[test]
    fn test_raw_string_as_token_ampersand() {
        let bytes = "&";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Ampersand);
    }

    #[test]
    fn test_raw_string_as_token_pipe() {
        let bytes = "|";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Pipe);
    }

    #[test]
    fn test_raw_string_as_token_caret() {
        let bytes = "^";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Caret);
    }

    #[test]
    fn test_raw_string_as_token_shift_left() {
        let bytes = "<<";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::ShiftLeft);
    }

    #[test]
    fn test_raw_string_as_token_shift_right() {
        let bytes = ">>";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::ShiftRight);
    }

    #[test]
    fn test_raw_string_as_token_plus() {
        let bytes = r"+";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Plus);
    }

    #[test]
    fn test_raw_string_as_token_minus() {
        let bytes = "-";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Minus);
    }

    #[test]
    fn test_raw_string_as_token_star() {
        let bytes = r"*";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Star);
    }

    #[test]
    fn test_raw_string_as_token_slash() {
        let bytes = "/";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Slash);
    }

    #[test]
    fn test_raw_string_as_token_percent() {
        let bytes = "%";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Percent);
    }

    #[test]
    fn test_raw_string_as_token_increment() {
        let bytes = r"++";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Increment);
    }

    #[test]
    fn test_raw_string_as_token_decrement() {
        let bytes = "--";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Decrement);
    }

    #[test]
    fn test_raw_string_as_token_plus_equals() {
        let bytes = r"+=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::PlusEquals);
    }

    #[test]
    fn test_raw_string_as_token_minus_equals() {
        let bytes = "-=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::MinusEquals);
    }

    #[test]
    fn test_raw_string_as_token_star_equals() {
        let bytes = r"*=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::StarEquals);
    }

    #[test]
    fn test_raw_string_as_token_slash_equals() {
        let bytes = "/=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::SlashEquals);
    }

    #[test]
    fn test_raw_string_as_token_percent_equals() {
        let bytes = "%=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::PercentEquals);
    }

    #[test]
    fn test_raw_string_as_token_ampersand_equals() {
        let bytes = "&=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::AmpersandEquals);
    }

    #[test]
    fn test_raw_string_as_token_pipe_equals() {
        let bytes = "|=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::PipeEquals);
    }

    #[test]
    fn test_raw_string_as_token_caret_equals() {
        let bytes = "^=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::CaretEquals);
    }

    #[test]
    fn test_raw_string_as_token_shift_left_equals() {
        let bytes = "<<=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::ShiftLeftEquals);
    }

    #[test]
    fn test_raw_string_as_token_shift_right_equals() {
        let bytes = ">>=";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::ShiftRightEquals);
    }

    #[test]
    fn test_raw_string_as_token_double_pipe() {
        let bytes = r"||";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::DoublePipe);
    }

    #[test]
    fn test_raw_string_as_token_double_ampersand() {
        let bytes = "&&";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::DoubleAmpersand);
    }

    #[test]
    fn test_raw_string_as_token_bang() {
        let bytes = "!";
        let token = raw_string_as_token(bytes).unwrap();
        assert_eq!(token, Token::Bang);
    }

    #[test]
    fn test_token_transformer_equal() {
        let token = Token::Equal;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::NotEqual));
    }

    #[test]
    fn test_token_transformer_not_equal() {
        let token = Token::NotEqual;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Equal));
    }

    #[test]
    fn test_token_transformer_less_than() {
        let token = Token::Less;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::GreaterEqual));
    }

    #[test]
    fn test_token_transformer_less_than_or_equal() {
        let token = Token::LessEqual;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Greater));
    }

    #[test]
    fn test_token_transformer_greater_than() {
        let token = Token::Greater;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::LessEqual));
    }

    #[test]
    fn test_token_transformer_greater_than_or_equal() {
        let token = Token::GreaterEqual;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Less));
    }

    #[test]
    fn test_token_transformer_and() {
        let token = Token::Ampersand;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Pipe));
    }

    #[test]
    fn test_token_transformer_or() {
        let token = Token::Pipe;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Ampersand));
    }

    #[test]
    fn test_token_transformer_xor() {
        let token = Token::Caret;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Ampersand));
    }

    #[test]
    fn test_token_transformer_ampersand() {
        let token = Token::Ampersand;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Pipe));
    }

    #[test]
    fn test_token_transformer_pipe() {
        let token = Token::Pipe;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Ampersand));
    }

    #[test]
    fn test_token_transformer_caret() {
        let token = Token::Caret;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Ampersand));
    }

    #[test]
    fn test_token_transformer_left_shift() {
        let token = Token::ShiftLeft;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::ShiftRight));
    }

    #[test]
    fn test_token_transformer_right_shift() {
        let token = Token::ShiftRight;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::ShiftLeft));
    }

    #[test]
    fn test_token_transformer_plus() {
        let token = Token::Plus;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Minus));
    }

    #[test]
    fn test_token_transformer_minus() {
        let token = Token::Minus;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Plus));
    }

    #[test]
    fn test_token_transformer_multiply() {
        let token = Token::Star;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Slash));
    }

    #[test]
    fn test_token_transformer_divide() {
        let token = Token::Slash;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Star));
    }

    #[test]
    fn test_token_transformer_modulo() {
        let token = Token::Percent;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Star));
    }

    #[test]
    fn test_token_transformer_increment() {
        let token = Token::Increment;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Decrement));
    }

    #[test]
    fn test_token_transformer_decrement() {
        let token = Token::Decrement;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Increment));
    }

    #[test]
    fn test_token_transformer_plus_equals() {
        let token = Token::PlusEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::MinusEquals));
    }

    #[test]
    fn test_token_transformer_minus_equals() {
        let token = Token::MinusEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::PlusEquals));
    }

    #[test]
    fn test_token_transformer_multiply_equals() {
        let token = Token::StarEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::SlashEquals));
    }

    #[test]
    fn test_token_transformer_divide_equals() {
        let token = Token::SlashEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::StarEquals));
    }

    #[test]
    fn test_token_transformer_modulo_equals() {
        let token = Token::PercentEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::StarEquals));
    }

    #[test]
    fn test_token_transformer_ampersand_equals() {
        let token = Token::AmpersandEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::PipeEquals));
    }

    #[test]
    fn test_token_transformer_pipe_equals() {
        let token = Token::PipeEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::AmpersandEquals));
    }

    #[test]
    fn test_token_transformer_caret_equals() {
        let token = Token::CaretEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::AmpersandEquals));
    }

    #[test]
    fn test_token_transformer_left_shift_equals() {
        let token = Token::ShiftLeftEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::ShiftRightEquals));
    }

    #[test]
    fn test_token_transformer_right_shift_equals() {
        let token = Token::ShiftRightEquals;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::ShiftLeftEquals));
    }

    #[test]
    fn test_token_transformer_double_pipe() {
        let token = Token::DoublePipe;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::DoubleAmpersand));
    }

    #[test]
    fn test_token_transformer_double_ampersand() {
        let token = Token::DoubleAmpersand;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::DoublePipe));
    }

    #[test]
    fn test_token_transformer_bang() {
        let token = Token::Bang;
        let mutation = token_transformer(token);
        assert_eq!(mutation, Some(Token::Void));
    }

    #[test]
    fn test_token_as_bytes_equal() {
        let token = Token::Equal;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"==");
    }

    #[test]
    fn test_token_as_bytes_not_equal() {
        let token = Token::NotEqual;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"!=");
    }

    #[test]
    fn test_token_as_bytes_less_than() {
        let token = Token::Less;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"<");
    }

    #[test]
    fn test_token_as_bytes_less_than_or_equal() {
        let token = Token::LessEqual;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"<=");
    }

    #[test]
    fn test_token_as_bytes_greater_than() {
        let token = Token::Greater;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b">");
    }

    #[test]
    fn test_token_as_bytes_greater_than_or_equal() {
        let token = Token::GreaterEqual;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b">=");
    }

    #[test]
    fn test_token_as_bytes_and() {
        let token = Token::Ampersand;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"&");
    }

    #[test]
    fn test_token_as_bytes_or() {
        let token = Token::Pipe;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"|");
    }

    #[test]
    fn test_token_as_bytes_xor() {
        let token = Token::Caret;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"^");
    }

    #[test]
    fn test_token_as_bytes_left_shift() {
        let token = Token::ShiftLeft;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"<<");
    }

    #[test]
    fn test_token_as_bytes_right_shift() {
        let token = Token::ShiftRight;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b">>");
    }

    #[test]
    fn test_token_as_bytes_plus() {
        let token = Token::Plus;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"+");
    }

    #[test]
    fn test_token_as_bytes_minus() {
        let token = Token::Minus;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"-");
    }

    #[test]
    fn test_token_as_bytes_multiply() {
        let token = Token::Star;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"*");
    }

    #[test]
    fn test_token_as_bytes_divide() {
        let token = Token::Slash;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"/");
    }

    #[test]
    fn test_token_as_bytes_modulo() {
        let token = Token::Percent;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"%");
    }

    #[test]
    fn test_token_as_bytes_increment() {
        let token = Token::Increment;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"++");
    }

    #[test]
    fn test_token_as_bytes_decrement() {
        let token = Token::Decrement;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"--");
    }

    #[test]
    fn test_token_as_bytes_plus_equals() {
        let token = Token::PlusEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"+=");
    }

    #[test]
    fn test_token_as_bytes_minus_equals() {
        let token = Token::MinusEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"-=");
    }

    #[test]
    fn test_token_as_bytes_multiply_equals() {
        let token = Token::StarEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"*=");
    }

    #[test]
    fn test_token_as_bytes_divide_equals() {
        let token = Token::SlashEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"/=");
    }

    #[test]
    fn test_token_as_bytes_modulo_equals() {
        let token = Token::PercentEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"%=");
    }

    #[test]
    fn test_token_as_bytes_ampersand_equals() {
        let token = Token::AmpersandEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"&=");
    }

    #[test]
    fn test_token_as_bytes_pipe_equals() {
        let token = Token::PipeEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"|=");
    }

    #[test]
    fn test_token_as_bytes_caret_equals() {
        let token = Token::CaretEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"^=");
    }

    #[test]
    fn test_token_as_bytes_left_shift_equals() {
        let token = Token::ShiftLeftEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"<<=");
    }

    #[test]
    fn test_token_as_bytes_right_shift_equals() {
        let token = Token::ShiftRightEquals;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b">>=");
    }

    #[test]
    fn test_token_as_bytes_double_pipe() {
        let token = Token::DoublePipe;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"||");
    }

    #[test]
    fn test_token_as_bytes_double_ampersand() {
        let token = Token::DoubleAmpersand;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"&&");
    }

    #[test]
    fn test_token_as_bytes_bang() {
        let token = Token::Bang;
        let bytes = token_as_bytes(&token).unwrap();
        assert_eq!(bytes, b"!");
    }

    #[test]
    fn test_mutant_builder_equal() {
        let path = PathBuf::from("test.noir");
        let token = Token::Equal;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::NotEqual);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "!=");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_not_equal() {
        let path = PathBuf::from("test.noir");
        let token = Token::NotEqual;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Equal);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "==");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_greater() {
        let path = PathBuf::from("test.noir");
        let token = Token::Greater;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::LessEqual);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "<=");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_greater_equal() {
        let path = PathBuf::from("test.noir");
        let token = Token::GreaterEqual;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Less);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "<");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_less() {
        let path = PathBuf::from("test.noir");
        let token = Token::Less;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::GreaterEqual);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, ">=");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_less_equal() {
        let path = PathBuf::from("test.noir");
        let token = Token::LessEqual;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Greater);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, ">");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_ampersand() {
        let path = PathBuf::from("test.noir");
        let token = Token::Ampersand;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Pipe);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "|");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_pipe() {
        let path = PathBuf::from("test.noir");
        let token = Token::Pipe;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Ampersand);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "&");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_caret() {
        let path = PathBuf::from("test.noir");
        let token = Token::Caret;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Ampersand);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "&");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_plus() {
        let path = PathBuf::from("test.noir");
        let token = Token::Plus;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Minus);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "-");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_minus() {
        let path = PathBuf::from("test.noir");
        let token = Token::Minus;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Plus);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "+");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_star() {
        let path = PathBuf::from("test.noir");
        let token = Token::Star;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Slash);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "/");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_slash() {
        let path = PathBuf::from("test.noir");
        let token = Token::Slash;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Star);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "*");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
    }

    #[test]
    fn test_mutant_builder_percent() {
        let path = PathBuf::from("test.noir");
        let token = Token::Percent;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.token(), Token::Star);
        let bytes_str =
            String::from_utf8(mutant.bytes().clone()).expect("Failed to convert bytes to string");
        assert_eq!(bytes_str, "*");
        assert_eq!(mutant.span_start(), 0);
        assert_eq!(mutant.span_end(), 1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_increment() {
        let path = PathBuf::from("test.noir");
        let token = Token::Increment;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_decrement() {
        let path = PathBuf::from("test.noir");
        let token = Token::Decrement;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_plus_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::PlusEquals;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_minus_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::MinusEquals;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_star_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::StarEquals;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_slash_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::SlashEquals;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_percent_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::PercentEquals;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_ampersand_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::AmpersandEquals;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_vmutant_builder_pipe_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::PipeEquals;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_caret_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::CaretEquals;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();

        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_shift_left_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::ShiftLeftEquals;
        let span = (0, 3);
        let id = 42;
        let mutant = mutant_builder(id, token.clone(), span, path.clone()).unwrap();

        assert_eq!(mutant.id(), id);
        assert_eq!(mutant.token(), token_transformer(token.clone()).unwrap());
        assert_eq!(
            &mutant.bytes(),
            token_as_bytes(&token_transformer(token).unwrap()).unwrap()
        );
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_shift_right_equals() {
        let path = PathBuf::from("test.noir");
        let token = Token::ShiftRightEquals;
        let span = (0, 3);
        let id = 42;
        let mutant = mutant_builder(id, token, span, path.clone()).unwrap();

        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_double_pipe() {
        let path = PathBuf::from("test.noir");
        let token = Token::DoublePipe;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token.clone(), span, path.clone()).unwrap();

        assert_eq!(mutant.id(), id);
        assert_eq!(mutant.token(), token_transformer(token.clone()).unwrap());
        assert_eq!(
            &mutant.bytes(),
            token_as_bytes(&token_transformer(token).unwrap()).unwrap()
        );
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_double_ampersand() {
        let path = PathBuf::from("test.noir");
        let token = Token::DoubleAmpersand;
        let span = (0, 2);
        let id = 42;
        let mutant = mutant_builder(id, token.clone(), span, path.clone()).unwrap();

        assert_eq!(mutant.id(), id);
        assert_eq!(mutant.token(), token_transformer(token.clone()).unwrap());
        assert_eq!(
            &mutant.bytes(),
            token_as_bytes(&token_transformer(token).unwrap()).unwrap()
        );
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }

    #[test]
    fn test_mutant_builder_bang() {
        let path = PathBuf::from("test.noir");
        let token = Token::Bang;
        let span = (0, 1);
        let id = 42;
        let mutant = mutant_builder(id, token.clone(), span, path.clone()).unwrap();

        assert_eq!(mutant.id(), id);
        assert_eq!(mutant.token(), token_transformer(token.clone()).unwrap());
        assert_eq!(
            &mutant.bytes(),
            token_as_bytes(&token_transformer(token).unwrap()).unwrap()
        );
        assert_eq!(mutant.span_start(), span.0);
        assert_eq!(mutant.span_end(), span.1);
        assert_eq!(mutant.path(), path);
    }
}
