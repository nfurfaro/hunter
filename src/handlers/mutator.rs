use crate::cli::Args;
use crate::config::Config;
use crate::handlers::scanner::ScanResult;
use crate::processor::process_mutants;
use crate::reporter::{print_table, surviving_mutants_table};
use crate::token::{token_as_bytes, token_transformer, Token};
use colored::*;
use std::{
    fmt,
    io::Result,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Mutant {
    id: u32,
    mutation: Token,
    bytes: Vec<u8>,
    span: (u32, u32),
    src_path: Box<PathBuf>,
    status: MutationStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MutationStatus {
    Pending,
    Survived,
    Killed,
}

impl fmt::Display for Mutant {
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

impl Mutant {
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
        &self.src_path
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

pub fn mutant_builder(
    id: u32,
    token: Token,
    span: (u32, u32),
    src_path: PathBuf,
) -> Option<Mutant> {
    let mutation = token_transformer(token.clone()).unwrap();
    match token {
        Token::Equal => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::NotEqual => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Greater => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::GreaterEqual => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Less => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::LessEqual => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Ampersand => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Pipe => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Caret => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::ShiftLeft => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::ShiftRight => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Plus => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Minus => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Star => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Slash => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Percent => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Increment => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Decrement => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::PlusEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::MinusEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::StarEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::SlashEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::PercentEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::AmpersandEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::PipeEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::CaretEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::ShiftLeftEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::ShiftRightEquals => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::DoublePipe => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::DoubleAmpersand => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Bang => Some(Mutant {
            id,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path.clone()),
            status: MutationStatus::Pending,
        }),
        // this is to catch Token::Void, for which there is no mutation.
        _ => None,
    }
}

pub fn mutate(args: Args, config: Config, results: &mut ScanResult) -> Result<()> {
    let mutants = results.mutants();
    println!("{}", "Running tests...".green());

    process_mutants(mutants, args.clone(), config.clone());

    if mutants
        .iter()
        .any(|mutant| mutant.status() == MutationStatus::Survived)
    {
        print_table(args.output_path, surviving_mutants_table(mutants))?;
    }

    Ok(())
}

pub fn calculate_mutation_score(destroyed: f64, total_mutants: f64) -> String {
    let mutation_score = (destroyed / total_mutants) * 100.0;
    format!("{:.2}%", mutation_score)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            src_path: Box::new(path.clone()),
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
        assert_eq!(mutant.path(), path);

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
            src_path: Box::new(path.clone()),
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
        assert_eq!(mutant.path(), path);

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
            src_path: Box::new(path.clone()),
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
        assert_eq!(mutant.path(), path);

        // Test start method
        assert_eq!(mutant.span_start(), 1000);

        // Test end method
        assert_eq!(mutant.span_end(), 2000);

        // Test status method
        assert_eq!(mutant.status(), MutationStatus::Pending);
    }
}
