use std::process::ExitCode;

mod cli;
mod lexer;

/////////////////////////////////////////////////////
// Main function
/////////////////////////////////////////////////////
pub fn main() -> ExitCode
{
    let options = cli::parse_cli_arguments(std::env::args().collect());
    let compiler_options_result = cli::parse_cli_options(options);

    if compiler_options_result.is_err()
    {
        eprintln!("{:?}", compiler_options_result.unwrap_err());
        return ExitCode::FAILURE;
    }

    let compiler_options = compiler_options_result.unwrap();
    println!("{:?}", compiler_options);
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

            for error in &errors
            {
                eprintln!("{:?}", error);
            }
            if !errors.is_empty()
            {
                return ExitCode::FAILURE;
            }

            for token in &tokens
            {
                println!("{:?}", token);
            }
        }
    }

    return ExitCode::SUCCESS;
}