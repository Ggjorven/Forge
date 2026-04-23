use std::path::Path;
use std::path::PathBuf;

use crate::cli;

use super::arguments::CliOption;

/////////////////////////////////////////////////////
// CliError
/////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub enum CliError
{
    UnknownFlag(String),
    UnknownOption(String),
    UnsupportedPlatform(String)
}

/////////////////////////////////////////////////////
// CompilerOptions
/////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct CompilerOptions
{
    pub files: Vec<PathBuf>,
    pub output: PathBuf
}

/////////////////////////////////////////////////////
// Parse functions
/////////////////////////////////////////////////////
pub fn parse_cli_options(cli_options: Vec<CliOption>) -> Result<CompilerOptions, CliError>
{
    let os_extension =
    { 
        match std::env::consts::OS
        {
            // "windows" => PathBuf::from(".exe"),
            "linux" => PathBuf::from(""),
            // "macos" => PathBuf::from(""),
            _ => return Err(CliError::UnsupportedPlatform(String::from("Unsupported compilation platform.")))
        }
    };

    let mut options: CompilerOptions = CompilerOptions { 
        files: Vec::new(),
        output: Path::new("output").with_extension(os_extension)
    };

    let mut i: usize = 0;
    while i < cli_options.len()
    {
        match cli_options.get(i).unwrap()
        {
            CliOption::Value(value) => 
            {
                options.files.push(PathBuf::from(value))
            },
            CliOption::Flag(value) => 
            {
                match value.as_str()
                {
                    "o" => 
                    {
                        if let Some(next) = cli_options.get(i + 1) && let CliOption::Value(flag_value) = next
                        {
                            options.output = PathBuf::from(flag_value);
                            
                            // Skip the next value
                            i = i + 1;
                        }
                    }
                    _ => 
                    { 
                        return Err(CliError::UnknownFlag(format!("Unknown flag: '-{}'", value)));
                    }
                }
            },
            CliOption::Option(key, value) =>
            {
                match key.as_str()
                {
                    "output" => { options.output = PathBuf::from(value) },
                    _ => 
                    { 
                        return Err(CliError::UnknownOption(format!("Unknown option: '--{}=...'", key)));
                    }
                }
            }
        }

        i = i + 1;
    }

    return Ok(options);
}