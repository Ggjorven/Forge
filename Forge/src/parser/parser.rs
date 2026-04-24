use std::fmt::format;
use std::fs;
use std::path::Path;

use crate::lexer::token;

use super::super::lexer::Token;
use super::super::lexer::TokenType;

use super::nodes::Type;
use super::nodes::Expression;
use super::nodes::BinaryOperator;
use super::nodes::Statement;
use super::nodes::Block;
use super::nodes::Parameter;
use super::nodes::FunctionDefinition;
use super::nodes::Item;

/////////////////////////////////////////////////////
// Helpers
/////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError
{
    ExpectedType(String),
    UnexpectedToken(String) // General error
}

/////////////////////////////////////////////////////
// Parser
/////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Parser
{
    pub tokens: Vec<Token>,
    pub current: usize
}

macro_rules! expect_token
{
    ($self:ident, $($pattern:pat),+) => 
    {
        if let Some(value) = $self.peek(None) 
        {
            if !matches!(value.token_type, $($pattern)|+) 
            {
                return Err(ParseError::UnexpectedToken(format!("Expected one of `{}` but found '{:?}'.", stringify!($($pattern),+), &value.token_type)));
            }
        } 
        else
        {
            return Err(ParseError::UnexpectedToken(format!("Expected one of `{}` but found None.", stringify!($($pattern),+))));
        }
    };
}

macro_rules! expect_type_token 
{
    ($self:ident) => 
    {
        expect_token!($self, 
            TokenType::Void, 
            TokenType::Bool, 
            TokenType::Char,
            TokenType::Int8, TokenType::Int16, TokenType::Int32, TokenType::Int64,
            TokenType::UInt8, TokenType::UInt16, TokenType::UInt32, TokenType::UInt64,
            TokenType::Float32, TokenType::Float64,
            TokenType::String 
            // TokenType::Identifier(_) // FUTURE TODO: Custom types
        );
    };
}

impl Parser
{
    /////////////////////////////////////////////////////
    // Public functions
    /////////////////////////////////////////////////////
    pub fn new(tokens: Vec<Token>) -> Self
    {
        return Self {
            tokens: tokens,
            current: 0
        };
    }

    pub fn get_items(mut self) -> (Vec<Item>, Vec<ParseError>)
    {
        let mut items: Vec<Item> = Vec::new();
        let mut errors: Vec<ParseError> = Vec::new();

        while !self.is_at_end(None) 
        {
            let next = self.parse_item();
            match next
            {
                Ok(item) => 
                {
                    items.push(item);
                }
                Err(error) =>
                {
                    eprintln!("{:?}", error); // TODO: Remove
                    errors.push(error);
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }

        return (items, errors);
    }

    /////////////////////////////////////////////////////
    // Private functions
    /////////////////////////////////////////////////////
    fn is_at_end(&self, token_to_check_or_current_char: Option<usize>) -> bool
    {
        let token: usize = token_to_check_or_current_char.unwrap_or(self.current);
        return token >= self.tokens.len();
    }

    fn peek(&self, offset: Option<usize>) -> Option<&Token>
    {
        let index: usize = self.current + offset.unwrap_or(0);
        // return self.source.get(index).copied();

        if self.is_at_end(Some(index)) {
            return None;
        }

        return Some(self.tokens.get(index).unwrap()); // No need to clone, since char is a Copy type.
    }

    fn consume(&mut self) -> Token 
    {
        let t = self.tokens.get(self.current).unwrap_or_else(|| { panic!("Internal logic error, consuming a token that doesn't exist."); });
        self.current += 1;
        return t.clone();
    }

    fn check(&self, token_type: TokenType) -> bool
    {
        let peeked = self.peek(None);
        
        if let Some(value) = peeked
        {
            if value.token_type == token_type
            {
                return true;
            }

            return false;
        }

        return false;
    }

    // fn check_type(&self) -> bool
    // {
    //     let types = [
    //         TokenType::Void, 
    //         TokenType::Bool, 
    //         TokenType::Char,
    //         TokenType::Int8, TokenType::Int16, TokenType::Int32, TokenType::Int64,
    //         TokenType::UInt8, TokenType::UInt16, TokenType::UInt32, TokenType::UInt64,
    //         TokenType::Float32, TokenType::Float64,
    //         TokenType::String
    //         // TokenType::Identifier(_) // FUTURE TODO: Custom types
    //     ];
// 
    //     let mut result: bool = false;
    //     for token_type in types
    //     {
    //         result |= self.check(token_type);
    //     }
// 
    //     return result;
    // }

    fn parse_item(&mut self) -> Result<Item, ParseError> 
    {
        match self.peek(None)
        {
            Some(token) => 
            {
                match &token.token_type  
                {
                    TokenType::Void |
                    TokenType::Bool |
                    TokenType::Char |
                    TokenType::Int8 | TokenType::Int16 | TokenType::Int32 | TokenType::Int64 |
                    TokenType::UInt8 | TokenType::UInt16 | TokenType::UInt32 | TokenType::UInt64 |
                    TokenType::Float32 | TokenType::Float64 |
                    TokenType::String // | 
                    // TokenType::Identifier(_) // FUTURE TODO: Custom types
                    => // Return type of function
                    {
                        let function_result = self.parse_function();
                        
                        if function_result.is_err()
                        {
                            return Err(function_result.unwrap_err());
                        }
                        
                        return Ok(Item::Function(function_result.unwrap()));
                    }
                    _ => 
                    {
                        return Err(ParseError::UnexpectedToken(format!("Unexpected token found ({:?})", token)));
                    }
                }
            }
            None => 
            {
                panic!("Internal logic error, parsing an item while there are no more tokens left.");
            }
        }
    }

    fn parse_function(&mut self) -> Result<FunctionDefinition, ParseError>
    {
        expect_type_token!(self);
        let return_type = self.consume();
        
        expect_token!(self, TokenType::Identifier(_));
        let function_name = self.consume();

        expect_token!(self, TokenType::LeftParenthesis); 
        self.consume();
        let parameters: Vec<Parameter> = 
        {
            let mut params = Vec::new();

            if !self.check(TokenType::RightParenthesis) 
            {
                loop 
                {
                    expect_type_token!(self);
                    let parameter_type_result = self.parse_type();
                    
                    if parameter_type_result.is_err()
                    {
                        return Err(parameter_type_result.unwrap_err());
                    }
                    
                    expect_token!(self, TokenType::Identifier(_));
                    let parameter_name_result = self.consume();
                    
                    if let TokenType::Identifier(parameter_name) = &parameter_name_result.token_type
                    {
                        params.push(Parameter { name: parameter_name.clone(), parameter_type: parameter_type_result.unwrap() });
                    }
                    
                    if !self.check(TokenType::Comma) { break; }
                    self.consume(); // Consume the ','
                }
            }

            params
        };
        expect_token!(self, TokenType::RightParenthesis); 
        self.consume();

        println!("------");
        println!("{:?}", &return_type);
        println!("{:?}", &function_name);

        for parameter in &parameters
        {
            println!("{:?}", parameter);
        }

        //let body = self.parse_block();
        
        //FunctionDef { name, params, return_type, body }

        return Err(ParseError::UnexpectedToken(String::from("AAAA")));
    }

    fn parse_type(&mut self) -> Result<Type, ParseError>
    {
        let token = self.consume();

        match token.token_type
        {
            TokenType::Void                           => return Ok(Type::Void),
            TokenType::Bool                           => return Ok(Type::Bool),
            TokenType::Char                           => return Ok(Type::Char),
            TokenType::Int8                           => return Ok(Type::Int8),
            TokenType::Int16                          => return Ok(Type::Int16),
            TokenType::Int32                          => return Ok(Type::Int32),
            TokenType::Int64                          => return Ok(Type::Int64),
            TokenType::UInt8                          => return Ok(Type::UInt8),
            TokenType::UInt16                         => return Ok(Type::UInt16),
            TokenType::UInt32                         => return Ok(Type::UInt32),
            TokenType::UInt64                         => return Ok(Type::UInt64),
            TokenType::Float32                        => return Ok(Type::Float32),
            TokenType::Float64                        => return Ok(Type::Float64),
            TokenType::String                         => return Ok(Type::String),
            TokenType::Identifier(identifier) => return Ok(Type::Named(identifier)), 
            _ => return Err(ParseError::ExpectedType(format!("Expected a type token.")))
        }
    }

}