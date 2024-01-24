use crate::cli::Args;
use crate::config::LanguageConfig;
use crate::handlers::scanner::ScanResult;
use crate::processor::process_mutants;
use crate::reporter::{print_table, surviving_mutants_table};
use crate::token::{random_token, token_as_bytes, token_transformer, MetaToken, Token};
use colored::*;
use std::{
    fmt,
    io::Result,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Mutant {
    id: u32,
    original: Token,
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
    Unbuildable,
}

impl fmt::Display for Mutant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Id: {:?}, Token: {:?}, Mutation: {:?} Bytes: {:?}, Span: {:?}, Source Path: {:?}, Status: {:?}",
            self.id,
            self.original,
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
        self.original.clone()
    }

    pub fn mutation(&self) -> Token {
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

pub fn mutants(meta_tokens: &Vec<MetaToken>, random: bool) -> Vec<Mutant> {
    let mut mutants: Vec<Mutant> = vec![];
    for entry in meta_tokens {
        let path = entry.src().clone();
        let maybe_mutant = mutant_builder(
            entry.id(),
            entry.token().clone(),
            entry.span(),
            path,
            random,
        );
        match maybe_mutant {
            None => continue,
            Some(m) => mutants.push(m),
        }
    }
    mutants
}

pub fn mutant_builder(
    id: u32,
    original: Token,
    span: (u32, u32),
    src_path: PathBuf,
    random: bool,
) -> Option<Mutant> {
    let mutation = match random {
        true => random_token(),
        false => token_transformer(original.clone()).unwrap(),
    };
    match original {
        Token::Equal => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::NotEqual => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Greater => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::GreaterEqual => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Less => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::LessEqual => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Ampersand => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Pipe => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Caret => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::ShiftLeft => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::ShiftRight => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Plus => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Minus => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Star => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Slash => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Percent => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Increment => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Decrement => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::PlusEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::MinusEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::StarEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::SlashEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::PercentEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::AmpersandEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::PipeEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::CaretEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::ShiftLeftEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::ShiftRightEquals => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::DoublePipe => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::DoubleAmpersand => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path),
            status: MutationStatus::Pending,
        }),
        Token::Bang => Some(Mutant {
            id,
            original,
            mutation: mutation.clone(),
            bytes: token_as_bytes(&mutation).unwrap().to_vec(),
            span,
            src_path: Box::new(src_path.clone()),
            status: MutationStatus::Pending,
        }),
        Token::Void => None,
    }
}

pub fn mutate(args: Args, config: Box<dyn LanguageConfig>, results: &mut ScanResult) -> Result<()> {
    let mutants = results.mutants();
    println!("{}", "Running tests...".green());

    process_mutants(mutants, args.clone(), config.clone_box());

    if mutants
        .iter()
        .any(|mutant| mutant.status() == MutationStatus::Survived)
    {
        print_table(args.output_path, surviving_mutants_table(mutants))?;
    }

    Ok(())
}

pub fn calculate_mutation_score(destroyed: f64, unbuildable: f64, total_mutants: f64) -> String {
    let mutation_score = ((destroyed + unbuildable) / total_mutants) * 100.0;
    format!("{:.2}%", mutation_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutant_methods() {
        let path = PathBuf::from("test.noir");
        let token = Token::Equal;
        let mutation = token_transformer(token.clone()).unwrap();
        let span = (0, 1);
        let mutant = Mutant {
            id: 0,
            original: token.clone(),
            mutation: mutation.clone(),
            bytes: token_as_bytes(&token.clone()).unwrap().to_vec(),
            span,
            src_path: Box::new(path.clone()),
            status: MutationStatus::Pending,
        };

        // Test token method
        assert_eq!(mutant.token(), token);

        // Test mutation method
        assert_eq!(mutant.mutation(), mutation);

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
        let mutation = token_transformer(token.clone()).unwrap();
        let span = (10, 20);
        let mutant = Mutant {
            id: 42,
            original: token.clone(),
            mutation: mutation.clone(),
            bytes: token_as_bytes(&token.clone()).unwrap().to_vec(),
            span,
            src_path: Box::new(path.clone()),
            status: MutationStatus::Pending,
        };

        // Test token method
        assert_eq!(mutant.token(), token);

        // Test mutation method
        assert_eq!(mutant.mutation(), mutation);

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
        let mutation = token_transformer(token.clone()).unwrap();
        let span = (1000, 2000);
        let mutant = Mutant {
            id: 42,
            original: token.clone(),
            mutation: mutation.clone(),
            bytes: token_as_bytes(&token.clone()).unwrap().to_vec(),
            span,
            src_path: Box::new(path.clone()),
            status: MutationStatus::Pending,
        };

        // Test token method
        assert_eq!(mutant.token(), token);

        // Test mutation method
        assert_eq!(mutant.mutation(), mutation);

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
