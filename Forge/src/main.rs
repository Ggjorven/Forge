use std::env;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub enum CliOption 
{
    Value(String),
    Flag(String),
    Option(String, String)
}

#[derive(Debug)]
pub struct CompilerOptions
{
    pub files: Vec<PathBuf>,
    pub output: PathBuf
}

// Types it can handle
// Value: value
// Flag: -flag
// Option: --option=value
pub fn parse_cli_arguments(args: Vec<String>) -> Vec<CliOption> 
{
    let mut options: Vec<CliOption> = Vec::new();

    for i in 1..args.len()
    {
        let arg: &String = &args[i];

        // Option or Flag
        if arg.starts_with("--") 
        {
            // 2.. removes the creates a view starting after the first 2 characters (which removes the '--')
            let splits: Option<(&str, &str)> = arg[2..].split_once('=');

            if let Some((key, value)) = splits // Option
            {
                options.push(CliOption::Option(key.to_string(), value.to_string()));
            }
            else // Flag
            {
                options.push(CliOption::Flag(arg[2..].to_string()));
            }
        }
        // Flag
        else if arg.starts_with('-')
        {
            options.push(CliOption::Flag(arg[1..].to_string()));
        }
        // Value
        else 
        {
            options.push(CliOption::Value(arg.clone()));
        }
    }

    return options;
}

pub fn parse_cli_options(cli_options: Vec<CliOption>) -> CompilerOptions
{
    let get_os_executable_extension = || -> PathBuf
    { 
        match std::env::consts::OS
        {
            "windows" => PathBuf::from(".exe"),
            "linux" => PathBuf::from(""),
            "macos" => PathBuf::from(""),
            _ => panic!("Unsupported compilation platform.")
        }
    };

    let mut options: CompilerOptions = CompilerOptions { 
        files: Vec::new(),
        output: Path::new("output").with_extension(get_os_executable_extension())
    };

    for option in cli_options
    {
        match option
        {
            CliOption::Value(value) => 
            {
                options.files.push(PathBuf::from(value))
            },
            CliOption::Flag(value) => 
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

fn main() 
{
    let options = parse_cli_arguments(env::args().collect());
    let compiler_options = parse_cli_options(options);

    println!("Options: {:?}", compiler_options);
}