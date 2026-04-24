use std::fs;
use std::path::Path;

use super::token::Token;
use super::token::TokenType;

/////////////////////////////////////////////////////
// Helpers
/////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub enum LexError
{
    // String = message, u32 = current_line

    IOError(String),
    CharacterParseError(String, u32),
    StringParseError(String, u32),
    IntegerParseError(String, u32),
    FloatParseError(String, u32)
}

/////////////////////////////////////////////////////
// Lexer
/////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Lexer
{
    source: Vec<char>,

    current_char: usize,
    current_line: u32
}

impl Lexer
{
    /////////////////////////////////////////////////////
    // Public functions
    /////////////////////////////////////////////////////
    pub fn new(file: &Path) -> Result<Self, LexError>
    {
        let source: Result<String, std::io::Error> = fs::read_to_string(file);

        match source 
        {
            Ok(string) => {
                return Ok(Self {
                    source: string.chars().collect(),

                    current_char: 0,
                    current_line: 1
                });
            }
            Err(error) =>
            {
                return Err(LexError::IOError(format!("Failed to open file '{}'. \nError: {:?}", file.to_str().unwrap_or(""), error.kind())));
            }
        }
    }

    pub fn get_tokens(mut self) -> (Vec<Token>, Vec<LexError>)
    {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<LexError> = Vec::new();

        while !self.is_at_end(None) 
        {
            self.skip_whitespace();
            if self.is_at_end(None) {
                break;
            }

            let next = self.next_token();
            match next
            {
                Ok(op_token) => 
                {
                    if let Some(token) = op_token {
                        tokens.push(token);
                    }
                }
                Err(error) =>
                {
                    errors.push(error);
                }
            }
        }

        return (tokens, errors);
    }

    /////////////////////////////////////////////////////
    // Private functions
    /////////////////////////////////////////////////////
    fn is_at_end(&self, char_to_check_or_current_char: Option<usize>) -> bool
    {
        let c: usize = char_to_check_or_current_char.unwrap_or(self.current_char);
        return c >= self.source.len();
    }

    fn skip_whitespace(&mut self)
    {
        let next_character = self.peek(None);

        if let Some(char) = next_character
        {
            if char == ' ' || char == '\t'
            {
                self.consume();
                self.skip_whitespace();
            }
            else if char == '\n' || char == '\r'
            {
                self.current_line += 1;
                self.consume();
                self.skip_whitespace();
            }
            // else return;
        }
    }

    fn peek(&self, offset: Option<usize>) -> Option<char>
    {
        let index: usize = self.current_char + offset.unwrap_or(0);
        // return self.source.get(index).copied();

        if self.is_at_end(Some(index)) {
            return None;
        }

        return Some(self.source[index]); // No need to clone, since char is a Copy type.
    }

    fn consume(&mut self) -> char
    {
        let c = self.source.get(self.current_char).unwrap_or_else(|| { panic!("Internal logic error, consuming a char that doesn't exist."); });
        self.current_char += 1;
        return *c; // char is a Copy type so this is fine.
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexError>
    {
        let c = self.consume();

        // Literals (number)
        if c.is_ascii_digit()
        {
            let result = self.lex_number(c);
            
            match result
            {
                Ok(token_type) => return Ok(Some(self.make_token(token_type))),
                Err(error) => return Err(error)
            }
        }

        // Keywords, Types & identifiers
        if c.is_alphabetic() || c == '_' 
        {
            let result_type = self.lex_keyword_types_and_identifiers(c);
            return Ok(Some(self.make_token(result_type)));
        }

        match c 
        {
            // Literals
            '"' => 
            {
                let result = self.lex_string();
            
                match result
                {
                    Ok(token_type) => return Ok(Some(self.make_token(token_type))),
                    Err(error) => return Err(error)
                }
            }
            '\'' => 
            {
                let result = self.lex_char();
            
                match result
                {
                    Ok(token_type) => return Ok(Some(self.make_token(token_type))),
                    Err(error) => return Err(error)
                }
            }

            // Punctuation
            '(' => return Ok(Some(self.make_token(TokenType::LeftParenthesis))),
            ')' => return Ok(Some(self.make_token(TokenType::RightParenthesis))),
            '{' => return Ok(Some(self.make_token(TokenType::LeftBrace))),
            '}' => return Ok(Some(self.make_token(TokenType::RightBrace))),
            ':' => return Ok(Some(self.make_token(TokenType::Colon))),
            ';' => return Ok(Some(self.make_token(TokenType::Semicolon))),
            ',' => return Ok(Some(self.make_token(TokenType::Comma))),
            '.' => return Ok(Some(self.make_token(TokenType::Dot))),
            
            // Punctuation + Operators mix
            '-' =>
            {
                let next_char = self.peek(None);

                if next_char == Some('>') 
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::Arrow)));
                }
                else if next_char == Some('-')
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::MinusMinus)));
                }
                else if next_char == Some('=')
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::MinusEquals)));
                }
                else {
                    return Ok(Some(self.make_token(TokenType::Minus)));
                }
            },
            '=' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::EqualsEquals)));
                }
                else {
                    return Ok(Some(self.make_token(TokenType::Equals)));
                }
            },
            
            // Operators
            '+' => 
            {
                let next_char = self.peek(None);

                if next_char == Some('=') 
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::PlusEquals)));
                }
                else if next_char == Some('+')
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::PlusPlus)));
                }
                else {
                    return Ok(Some(self.make_token(TokenType::Plus)));
                }
            },
            '*' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::StarEquals)));
                }
                else {
                    return Ok(Some(self.make_token(TokenType::Star)));
                }
            },
            '/' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::SlashEquals)));
                }
                else {
                    return Ok(Some(self.make_token(TokenType::Slash)));
                }
            },

            '!' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::NotEquals)));
                }
                else 
                {
                    eprintln!("Invalid token found on line {}, '!'.", self.current_line);
                    return Ok(None);
                }
            },

            '<' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::LessThanOrEquals)));
                }
                else {
                    return Ok(Some(self.make_token(TokenType::LessThan)));
                }
            },
            '>' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    return Ok(Some(self.make_token(TokenType::GreaterThanOrEquals)));
                }
                else {
                    return Ok(Some(self.make_token(TokenType::GreaterThan)));
                }
            }
            _ => return Ok(None)
        }
    }

    fn lex_number(&mut self, start_char: char) -> Result<TokenType, LexError>
    {
        let mut num = String::from(start_char);
        let mut is_float = false;

        while let Some(c) = self.peek(None) 
        {
            if c.is_ascii_digit() 
            {
                num.push(c);
                self.consume();
            } 
            else if c == '.' && !is_float {
                is_float = true;
                num.push(c);
                self.consume();
            } else {
                break;
            }
        }

        if is_float {
            return Ok(TokenType::Float64Literal(num.parse().unwrap()));
        } 

        return Ok(TokenType::Int64Literal(num.parse().unwrap()));
    }

    fn lex_string(&mut self) -> Result<TokenType, LexError>
    {
        let mut string = String::new();

        loop
        {
            match self.peek(None)
            {
                None => 
                {
                    return Err(LexError::StringParseError(format!("Unterminated string on line {}", self.current_line), self.current_line));
                }
                Some('"') => 
                {
                    self.consume(); // consume closing "
                    break;
                }
                Some('\\') =>
                {
                    self.consume(); // consume backslash
                    let escaped = self.parse_escape_sequence()?;
                    string.push(escaped);
                }
                Some(c) =>
                {
                    string.push(c);
                    self.consume();
                }
            }
        }

        return Ok(TokenType::StringLiteral(string));
    }

    fn lex_char(&mut self) -> Result<TokenType, LexError>
    {
        let c = match self.peek(None)
        {
            None => 
            {
                return Err(LexError::CharacterParseError(format!("Unterminated char literal on line {}", self.current_line), self.current_line));
            }
            Some('\\') =>
            {
                self.consume(); // consume backslash
                self.parse_escape_sequence()?
            }
            Some(_c) =>
            {
                self.consume()
            }
        };

        // Expect closing '
        match self.peek(None)
        {
            Some('\'') => { self.consume(); }
            Some(c) => 
            {
                return Err(LexError::CharacterParseError(format!("Expected closing ' on line {} but got '{}'", self.current_line, c), self.current_line));
            }
            None => 
            {
                return Err(LexError::CharacterParseError(format!("Unterminated char literal on line {}", self.current_line), self.current_line));
            }
        }

        return Ok(TokenType::CharLiteral(c));
    }

    fn parse_escape_sequence(&mut self) -> Result<char, LexError>
    {
        match self.peek(None)
        {
            Some('n')  => { self.consume(); Ok('\n') }
            Some('t')  => { self.consume(); Ok('\t') }
            Some('r')  => { self.consume(); Ok('\r') }
            Some('\\') => { self.consume(); Ok('\\') }
            Some('\'') => { self.consume(); Ok('\'') }
            Some('"')  => { self.consume(); Ok('"')  }
            Some('0')  => { self.consume(); Ok('\0') }
            Some(c) =>
            {
                return Err(LexError::CharacterParseError(format!("Unknown escape sequence '\\{}' on line {}", c, self.current_line), self.current_line));
            }
            None =>
            {
                return Err(LexError::CharacterParseError(format!("Unexpected end of file after '\\' on line {}", self.current_line), self.current_line));
            }
        }
    }

    fn lex_keyword_types_and_identifiers(&mut self, start_char: char) -> TokenType
    {
        let mut identifier = String::from(start_char);

        while let Some(c) = self.peek(None) 
        {
            if c.is_alphanumeric() || c == '_' 
            {
                identifier.push(c);
                self.consume();
            } 
            else {
                break;
            }
        }

        // Keywords — match the string, fall back to Ident
        match identifier.as_str() 
        {
            // Keywords
            "let"    => return TokenType::Let,
            "return" => return TokenType::Return,
            "true"   => return TokenType::True,
            "false"  => return TokenType::False,

            // Types
            "void" => return TokenType::Void,
            "bool" => return TokenType::Bool,
            "char" => return TokenType::Char,
            "int8" => return TokenType::Int8,
            "int16" => return TokenType::Int16,
            "int32" => return TokenType::Int32,
            "int64" => return TokenType::Int64,
            "uint8" => return TokenType::UInt8,
            "uint16" => return TokenType::UInt16,
            "uint32" => return TokenType::UInt32,
            "uint64" => return TokenType::UInt64,
            "float32" => return TokenType::Float32,
            "float64" => return TokenType::Float64,
            "string" => return TokenType::String,

            // Identifier
            _        => return TokenType::Identifier(identifier),
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token 
    {
        return Token { token_type: token_type, line: self.current_line }
    }
}