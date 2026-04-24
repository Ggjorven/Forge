use std::fs;
use std::path::Path;

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
    AAA
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
                    errors.push(error);
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

    fn parse_item(&mut self) -> Result<Item, ParseError> 
    {
        // match self.peek()
        // {
        //     TokenKind::Fn => Item::Function(self.parse_function()),
        //     _ => panic!("Expected top-level item at line {}", self.peek().line),
        // }
        return Err(ParseError::AAA);
    }
}