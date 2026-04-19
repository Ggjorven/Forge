use std::path::Path;
use std::path::PathBuf;

use super::arguments::CliOption;

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
pub fn parse_cli_options(cli_options: Vec<CliOption>) -> CompilerOptions
{
    let get_os_executable_extension = || -> PathBuf
    { 
        match std::env::consts::OS
        {
            // "windows" => PathBuf::from(".exe"),
            "linux" => PathBuf::from(""),
            // "macos" => PathBuf::from(""),
            _ => panic!("Unsupported compilation platform.")
        }
    };

    let mut options: CompilerOptions = CompilerOptions { 
        files: Vec::new(),
        output: Path::new("output").with_extension(get_os_executable_extension())
    };

    for option in &cli_options
    {
        match option
        {
            CliOption::Value(value) => 
            {
                options.files.push(PathBuf::from(value))
            },
            CliOption::Flag(_value) => 
            {
                // TODO: ...
            },
            CliOption::Option(key, value) =>
            {
                match key.as_str()
                {
                    "output" => { options.output = PathBuf::from(value) },
                    _ => 
                    { 
                        println!("Unknown option: '--{}=...'", key);
                    }
                }
            }
        }
    }

    return options;
}