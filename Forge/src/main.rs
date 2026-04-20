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

        if let Err(error) = lexer_result
        {
            eprintln!("Failed to create lexer with error: {:?}", error);
            continue;
        }
        else if let Ok(lexer_obj) = lexer_result
        {
            let tokens = lexer_obj.get_tokens();

            for token in &tokens
            {
                println!("Token: {:?}", token);
            }
        }
    }

    return ExitCode::SUCCESS;
}