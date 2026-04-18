use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Clone)]
pub enum CliOption 
{
    Value(String),
    Flag(String),
    Option(String, String)
}

#[derive(Debug, Clone)]
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

    for option in &cli_options
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

#[derive(Debug, Clone, PartialEq)] // PartialEq just checks the Kind and not the internal type
pub enum TokenType
{
    // Literals
    Int(i64),       // FUTURE TODO: unsigned and different sizes
    Float(f64),     // FUTURE TODO: different sizes
    String(String),

    // Keywords
    Let,
    Return,
    True,
    False,

    // Identifiers
    Identifier(String),

    // Punctuation
    LeftParenthesis, RightParenthesis,  // ( )
    LeftBrace, RightBrace,              // { }
    Colon,                              // :
    Semicolon,                          // ;
    Comma,                              // ,
    Equals,                             // =
    Arrow,                              // ->
    
    // Operators
    Plus, Minus, Star, Slash,   // +, -, *, /
    PlusPlus, MinusMinus,       // ++, --

    EqualsEquals, NotEquals,                // ==, !=
    LessThan, GreaterThan,                  // <, >
    LessThanOrEquals, GreaterThanOrEquals,  // <=, >=
    MinusEquals, PlusEquals                 // -=, +=
}

#[derive(Debug)]
pub struct Token
{
    token_type: TokenType,
    
    // Debug
    line: u32
}

pub struct Lexer
{
    source: Vec<char>,

    current_char: usize,
    current_line: u32
}

impl Lexer
{
    pub fn new(file: &Path) -> Result<Self, std::io::Error>
    {
        let source = fs::read_to_string(file);

        match source 
        {
            Ok(string) => {
                return Ok(Self {
                    source: string.chars().collect(),

                    current_char: 0,
                    current_line: 0
                });
            }
            Err(error) =>
            {
                return Err(error);
            }
        }
    }

    pub fn get_tokens(mut self) -> Vec<Token>
    {
        let mut tokens = Vec::new();

        //while !self.is_at_end() 
        //{
        //    self.skip_whitespace();
        //    if self.is_at_end() 
        //    {
        //        break;
        //    }
//
        //    if let Some(token) = self.next_token() {
        //        tokens.push(token);
        //    }
        //}

        return tokens;
    }
}

pub fn main() -> ExitCode
{
    let options = parse_cli_arguments(env::args().collect());
    let compiler_options = parse_cli_options(options);

    for file in &compiler_options.files
    {
        let lexer = Lexer::new(file.as_path());

        match lexer
        {
            Ok(lexer) =>
            {
                let tokens = lexer.get_tokens();

                for token in &tokens
                {
                    println!("Token: {:?}", token);
                }
            }
            Err(error) =>
            {
                eprintln!("Failed to lex '{:?}' with IO error: {:?}", &file, error);
                return ExitCode::FAILURE;
            }
        }
    }

    println!("Options: {:?}", compiler_options);

    return ExitCode::SUCCESS;
}