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
    IOError(String),
    CharacterParseError(String),
    StringParseError(String)
}

/////////////////////////////////////////////////////
// Lexer
/////////////////////////////////////////////////////
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
                    current_line: 0
                });
            }
            Err(error) =>
            {
                return Err(LexError::IOError(std::fmt::format(format_args!("Failed to open file '{}'. \nError: {:?}", file.to_str().unwrap_or(""), error.kind()))));
            }
        }
    }

    pub fn get_tokens(mut self) -> Vec<Token>
    {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end(None) 
        {
            self.skip_whitespace();
            if self.is_at_end(None) {
                break;
            }

            let next = self.next_token();
            if let Ok(op_token) = next 
            {
                if let Some(token) = op_token
                {
                    tokens.push(token);
                }
            } 
            else if let Err(error) = next {
                eprintln!("Failed to parse token with error: {:?}", error);
            }
        }

        return tokens;
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
            if char == ' '
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
        let c = self.source[self.current_char]; // No need to clone, since char is a Copy type.
        self.current_char += 1;
        return c;
    }

    ////////////////////////////////////////////////////////////// CHANGE BELOW THIS /////////////////////////////////////////////////////////////

    fn next_token(&mut self) -> Result<Option<Token>, LexError>
    {
        let c = self.consume();

        let token_type: TokenType;
        match c 
        {
            // Literals
            c if c.is_ascii_digit() => token_type = self.lex_number(c),
            '"' => token_type = self.lex_string(),
            '\'' => 
            {
                self.consume();

                if let Some(character) = self.peek(None)
                {
                }

                token_type = TokenType::CharLiteral(self.consume());
                self.consume();
            } // token_type = self.lex_string(),

            // Keywords & identifiers
            c if c.is_alphabetic() || c == '_' => token_type = self.lex_ident_or_keyword(c),

            // Punctuation
            '(' => token_type = TokenType::LeftParenthesis,
            ')' => token_type = TokenType::RightParenthesis,
            '{' => token_type = TokenType::LeftBrace,
            '}' => token_type = TokenType::RightBrace,
            ':' => token_type = TokenType::Colon,
            ';' => token_type = TokenType::Semicolon,
            ',' => token_type = TokenType::Comma,
            '.' => token_type = TokenType::Dot,
            
            // Punctuation + Operators mix
            '-' =>
            {
                let next_char = self.peek(None);

                if next_char == Some('>') 
                {
                    self.consume();
                    token_type = TokenType::Arrow;
                }
                else if next_char == Some('-')
                {
                    self.consume();
                    token_type = TokenType::MinusMinus;
                }
                else if next_char == Some('=')
                {
                    self.consume();
                    token_type = TokenType::MinusEquals;
                }
                else {
                    token_type = TokenType::Minus;
                }
            },
            '=' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    token_type = TokenType::EqualsEquals;
                }
                else {
                    token_type = TokenType::Equals;
                }
            },
            
            // Operators
            '+' => 
            {
                let next_char = self.peek(None);

                if next_char == Some('=') 
                {
                    self.consume();
                    token_type = TokenType::PlusEquals;
                }
                else if next_char == Some('+')
                {
                    self.consume();
                    token_type = TokenType::PlusPlus;
                }
                else {
                    token_type = TokenType::Plus;
                }
            },
            '*' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    token_type = TokenType::StarEquals;
                }
                else {
                    token_type = TokenType::Star;
                }
            },
            '/' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    token_type = TokenType::SlashEquals;
                }
                else {
                    token_type = TokenType::Slash;
                }
            },

            '!' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    token_type = TokenType::NotEquals;
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
                    token_type = TokenType::LessThanOrEquals;
                }
                else {
                    token_type = TokenType::LessThan;
                }
            },
            '>' => 
            {
                if self.peek(None) == Some('=') 
                {
                    self.consume();
                    token_type = TokenType::GreaterThanOrEquals;
                }
                else {
                    token_type = TokenType::GreaterThan;
                }
            },

            _ => return Ok(None)
        }

        return Ok(Some(self.make_token(token_type)));
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
            return TokenType::Float64Literal(num.parse().unwrap());
        } 

        return TokenType::Int64Literal(num.parse().unwrap());
    }

    fn lex_string(&mut self) -> TokenType
    {
        let mut string = String::new();

        while let Some(c) = self.peek(None) 
        {
            if c == '"' 
            { 
                self.consume(); 
                break; 
            }

            string.push(c);
            self.consume();
        }

        return TokenType::StringLiteral(string);
    }

    fn lex_char(&mut self) -> TokenType
    {
        let mut c: char;

        let read_char = || -> char 
        {
            if let Some(c) = self.peek(None)
            {

            }
            else 
            {
                eprintln!("");
                return ' ';
            }
        };

        if let Some(c) = self.peek(None)
        {
            if c == '\\'
            {
                self.consume();
                read_char();
            }
            else {
                read_char();
            }
        }

        return TokenType::CharLiteral(c);
    }

    fn lex_ident_or_keyword(&mut self, start_char: char) -> TokenType
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
            "let"    => return TokenType::Let,
            "return" => return TokenType::Return,
            "true"   => return TokenType::True,
            "false"  => return TokenType::False,
            _        => return TokenType::Identifier(identifier),
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token 
    {
        Token { token_type: token_type, line: self.current_line }
    }
}