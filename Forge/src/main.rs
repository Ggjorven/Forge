use core::error;
use std::process::ExitCode;

mod cli;
mod lexer;

/////////////////////////////////////////////////////
// Main function
/////////////////////////////////////////////////////
pub fn main() -> ExitCode
{
    let options = cli::parse_cli_arguments(std::env::args().collect());
    let compiler_options = cli::parse_cli_options(options);

    for file in &compiler_options.files
    {
        let lexer_result = lexer::Lexer::new(file.as_path());

        if lexer_result.is_err()
        {
            let err = lexer_result.unwrap_err();

            eprintln!("Failed to create lexer with error: {:?}", err);
            continue;
        }
        else
        {
            let tokens_and_errors = lexer_result.unwrap().get_tokens();
            let tokens = tokens_and_errors.0;
            let errors = tokens_and_errors.1;

            for error in errors
            {
                eprintln!("{:?}", error);
            }

            for token in &tokens
            {
                println!("Token: {:?}", token);
            }
        }
    }

    return ExitCode::SUCCESS;
}