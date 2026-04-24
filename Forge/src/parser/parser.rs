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
}

/////////////////////////////////////////////////////
// Parser
/////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Parser
{
}

impl Parser
{
    /////////////////////////////////////////////////////
    // Public functions
    /////////////////////////////////////////////////////
    pub fn new(tokens: Vec<Token>) -> Result<Self, ParseError>
    {
        return Ok(Self{});
    }

    /////////////////////////////////////////////////////
    // Private functions
    /////////////////////////////////////////////////////
    
}