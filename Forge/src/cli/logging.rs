use std::path::Path;

// Inspired by:
// warning: unused import: `std::fs`
//  --> src/parser/parser.rs:1:5
//   |
// 1 | use std::fs;
//   |     ^^^^^^^

pub fn warning(source_file: &Path, source: &Vec<String>, line: u32, warning: &str)
{
    println!("warning: {}", warning);
    println!(" --> {}:{}", source_file.as_os_str().display(), line);
    
    let spacing: String = " ".repeat(line.to_string().len());
    let line_str = source.get((line - 1) as usize).unwrap_or_else(|| { panic!("Internal logic error, attempting to print a warning for a line that doesn't exist."); });

    println!("{} |", spacing);
    println!("{} | {}", line, line_str);
    println!("{} |", spacing); // TODO: Arrows to the warning
}

pub fn error(source_file: &Path, source: &Vec<String>, line: u32, error: &str)
{
    println!("error: {}", error);
    println!(" --> {}:{}", source_file.as_os_str().display(), line);
    
    let spacing: String = " ".repeat(line.to_string().len());
    let line_str = source.get((line - 1) as usize).unwrap_or_else(|| { panic!("Internal logic error, attempting to print a error for a line that doesn't exist."); });

    println!("{} |", spacing);
    println!("{} | {}", line, line_str);
    println!("{} |", spacing); // TODO: Arrows to the error
}