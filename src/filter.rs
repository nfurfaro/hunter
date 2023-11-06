use noirc_frontend::token::{SpannedToken, Token};

// Given a SpannedToken, filter for mutable tokens. If found, return a tuple of the opposite Token and the original span
pub fn token_mutator(input: SpannedToken) -> Option<SpannedToken> {
    match input.token() {
        Token::NotEqual => return Some(SpannedToken::new(Token::Equal, input.to_span())),
        Token::Equal => return Some(SpannedToken::new(Token::NotEqual, input.to_span())),
        _ => None,
    }
}

// fn token_filter(token: Token) -> Option<Token> {
//     match token {
//         Token::Equal
//         | Token::NotEqual
//         | Token::Greater
//         | Token::GreaterEqual
//         | Token::Less
//         | Token::LessEqual
//         | Token::Ampersand
//         | Token::Pipe
//         | Token::Caret
//         | Token::ShiftLeft
//         | Token::ShiftRight
//         | Token::Plus
//         | Token::Minus
//         | Token::Star
//         | Token::Slash
//         | Token::Percent => Some(token),
//         _ => None,
//     }
// }
