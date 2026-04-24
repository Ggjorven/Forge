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
    UnexpectedToken(String)
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

macro_rules! expect_token {
    ($self:ident, $pattern:pat) => // _ wildcards are valid in the pattern
    {
        if let Some(value) = $self.peek(None)
        {
            if !matches!(value.token_type, $pattern)
            {
                return Err(ParseError::UnexpectedToken(format!("Expected `{}` token, but found '{:?}'.", stringify!($pattern), &value.token_type)));
            }
        }
        else
        {
            return Err(ParseError::UnexpectedToken(format!("Expected `{}` token, but found None.", stringify!($pattern))));
        }
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

    fn consume(&mut self) -> &Token 
    {
        let t = self.tokens.get(self.current).unwrap_or_else(|| { panic!("Internal logic error, consuming a token that doesn't exist."); });
        self.current += 1;
        return t;
    }

    fn expect(&mut self, token_type: TokenType) -> Option<ParseError> // Does not consume token
    {
        let peeked = self.peek(None);
        
        if let Some(value) = peeked
        {
            if value.token_type == token_type
            {
                return None;
            }

            return Some(ParseError::UnexpectedToken(format!("Expected `{:?}` token, but found '{:?}'.", token_type, &value.token_type)));
        }

        return Some(ParseError::UnexpectedToken(format!("Expected `{:?}` token, but found None.", token_type)));
    }

    fn parse_item(&mut self) -> Result<Item, ParseError> 
    {
        match self.peek(None)
        {
            Some(token) => 
            {
                match &token.token_type  
                {
                    TokenType::Identifier(_identifier) => // Return type of function
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
        // Identifier
        // let parse_error: Option<ParseError>;
        // parse_error = self.expect(TokenType::Identifier(String::from("")));
        // if let Some(error) = parse_error
        // {
        //     return Err(error);
        // }

        // expect_token!(self, TokenType::Identifier(String::from("")));
        expect_token!(self, TokenType::Identifier(_));
        self.consume();
        
        expect_token!(self, TokenType::Identifier(_));

        //self.expect(TokenKind::Fn);
//
        //let name = self.expect_ident();
//
        //self.expect(TokenKind::LParen);
        //let params = self.parse_params();
        //self.expect(TokenKind::RParen);
//
        //let return_type = if self.check(TokenKind::Arrow) {
        //    self.advance();
        //    Some(self.parse_type())
        //} else {
        //    None
        //};
//
        //let body = self.parse_block();
//
        //FunctionDef { name, params, return_type, body }

        return Err(ParseError::UnexpectedToken(String::from("AAAA")));
    }

}