use noirc_frontend::lexer::Lexer;
use noirc_frontend::token::Token;


fn lex(source: &str) {
    let (tokens, lexing_errors) = Lexer::lex(source);
}

fn mutation_handler() -> Token {
    match token {
        Token::Equal() => Token::NotEqual,
        Token::NotEqual() => Token::Equal,
    }
}

pub fn load_src_files(src: Option<std::path::PathBuf>) {

}


pub fn mutate() {}
pub fn run_tests() {}
pub fn report() {}
