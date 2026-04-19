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
        let lexerObj = lexer::Lexer::new(file.as_path());
        let tokens = lexerObj.get_tokens();

        for token in &tokens
        {
            println!("Token: {:?}", token);
        }
    }

    return ExitCode::SUCCESS;
}