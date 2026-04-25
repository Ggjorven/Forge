use std::fs;
use std::path::Path;

use super::super::lexer::Token;
use super::super::lexer::TokenType;

use super::nodes::Type;
use super::nodes::Expression;
use super::nodes::BinaryOperator;
use super::nodes::Statement;
use super::nodes::Parameter;
use super::nodes::FunctionDefinition;
use super::nodes::Item;

/////////////////////////////////////////////////////
// Helpers
/////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError
{
    // String = message, u32 = current_line

    ExpectedType(String, u32),
    ExpectedSemicolon(String, u32),
    UnexpectedToken(String, u32), // General error
    NoTokenFound(String, u32)
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
        if let Some(value) = $self.peek(None).cloned()
        {
            if !matches!(value.token_type, $($pattern)|+) 
            {
                $self.consume();
                return Err(ParseError::UnexpectedToken(format!("Expected one of `{}` but found '{:?}'.", stringify!($($pattern),+), &value.token_type), value.line));
            }
        } 
        else
        {
            $self.consume();
            return Err(ParseError::UnexpectedToken(format!("Expected one of `{}` but found None.", stringify!($($pattern),+)), 0)); // TODO: Replace 0 with actual line number
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
        match self.peek(None).cloned()
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
                        self.consume();
                        return Err(ParseError::UnexpectedToken(format!("Unexpected token found ({:?})", token), token.line));
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
        let return_type_result = self.parse_type();
        
        if return_type_result.is_err()
        {
            return Err(return_type_result.unwrap_err());
        }

        let return_type = return_type_result.unwrap();

        expect_token!(self, TokenType::Identifier(_));
        let function_name_token = self.consume();

        let mut function_name: String = String::from("");
        if let TokenType::Identifier(f_name) = function_name_token.token_type
        {
            function_name = f_name;
        }

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

        expect_token!(self, TokenType::LeftBrace);
        let body_result = self.parse_block();

        if body_result.is_err()
        {
            return Err(body_result.unwrap_err());
        }
        
        let body = body_result.unwrap();

        return Ok(FunctionDefinition { name: function_name, parameters, return_type: return_type, body: body });
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
            _ => return Err(ParseError::ExpectedType(format!("Expected a type token."), token.line))
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, ParseError>
    {
        let mut statements: Vec<Statement> = Vec::new();

        expect_token!(self, TokenType::LeftBrace);
        self.consume();
        while !self.check(TokenType::RightBrace) && !self.is_at_end(None) 
        {
            let statement_result = self.parse_statement();

            if statement_result.is_err()
            {
                return Err(statement_result.unwrap_err());
            }
            
            statements.push(statement_result.unwrap());
        }
        expect_token!(self, TokenType::RightBrace);
        self.consume();

        return Ok(statements);
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> 
    {
        match self.peek(None) 
        {
            Some(token) =>
            {
                match token.token_type
                {
                    // TokenType::Let | // FUTURE TODO: Add when there is implicit type conversion
                    TokenType::Void |
                    TokenType::Bool |
                    TokenType::Char |
                    TokenType::Int8 | TokenType::Int16 | TokenType::Int32 | TokenType::Int64 |
                    TokenType::UInt8 | TokenType::UInt16 | TokenType::UInt32 | TokenType::UInt64 |
                    TokenType::Float32 | TokenType::Float64 |
                    TokenType::String
                    => 
                    {
                        let let_result = self.parse_let();

                        if let_result.is_err()
                        {
                            return Err(let_result.unwrap_err());
                        }
                        
                        return Ok(let_result.unwrap());
                    },
                    TokenType::Return => 
                    {
                        let return_result = self.parse_return();

                        if return_result.is_err()
                        {
                            return Err(return_result.unwrap_err());
                        }
                        
                        return Ok(return_result.unwrap());
                    },
                    _ => 
                    { 
                        let expression_result = self.parse_expression();

                        if expression_result.is_err()
                        {
                            return Err(expression_result.unwrap_err());
                        }
                        
                        return Ok(Statement::Expression(expression_result.unwrap()));
                    }
                }
            }
            None => 
            {
                return Err(ParseError::NoTokenFound(String::from("Expected to parse statement, but no token found."), 0)); // TODO: Replace 0 with a line number
            }
        }
    }

    fn parse_let(&mut self) -> Result<Statement, ParseError> 
    {
        // FUTURE TODO: Allow let keyword and implicit type 
        expect_type_token!(self);
        let type_result = self.parse_type();

        if type_result.is_err()
        {
            return Err(type_result.unwrap_err());
        }

        let let_type = type_result.unwrap();

        expect_token!(self, TokenType::Identifier(_));
        let name_token = self.consume();

        let mut variable_name: String = String::from("");
        if let TokenType::Identifier(name) = name_token.token_type
        {
            variable_name = name;
        }

        expect_token!(self, TokenType::Equals);
        self.consume();

        // FUTURE TODO: Allow non-initialized variable
        let initializer_result = self.parse_expression();

        if initializer_result.is_err()
        {
            return Err(initializer_result.unwrap_err());
        }

        expect_token!(self, TokenType::Semicolon);
        self.consume();

        return Ok(Statement::Variable { name: variable_name, variable_type: let_type, initializer: Some(initializer_result.unwrap()) });
    }

    fn parse_return(&mut self) -> Result<Statement, ParseError>  
    {
        expect_token!(self, TokenType::Return);
        self.consume();

        let expression_result = self.parse_expression();

        if expression_result.is_err()
        {
            return Err(expression_result.unwrap_err());
        }

        expect_token!(self, TokenType::Semicolon);
        self.consume();

        return Ok(Statement::Return(expression_result.unwrap()));
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> 
    {
        // TODO: implement Pratt parsing / precedence climbing here
        return self.parse_primary();
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> 
    {
        let token = self.consume();

        match token.token_type 
        {
            // FUTURE TODO: Different sizes and unsigned
            TokenType::Int64Literal(n) => return Ok(Expression::IntegerLiteral(n)),
            TokenType::Float64Literal(f) => return Ok(Expression::FloatLiteral(f)),
            TokenType::StringLiteral(s) => return Ok(Expression::StringLiteral(s)),
            TokenType::True => return Ok(Expression::BooleanLiteral(true)),
            TokenType::False => return Ok(Expression::BooleanLiteral(false)),
            TokenType::Identifier(name) => 
            {
                // Could be a call:  name(args)
                if self.check(TokenType::LeftParenthesis) 
                {
                    self.consume();

                    let args_result = self.parse_call_args();
                    
                    if args_result.is_err()
                    {
                        return Err(args_result.unwrap_err());
                    }

                    expect_token!(self, TokenType::RightParenthesis);
                    self.consume();

                    return Ok(Expression::Call { callee: name, arguments: args_result.unwrap() });
                } 
                else 
                {
                    return Ok(Expression::Identifier(name));
                }
            }
            _ => return Err(ParseError::UnexpectedToken(format!("Unexpected token {:?} at line {}.", token.token_type, token.line), token.line))
        }
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expression>, ParseError> 
    {
        let mut args = Vec::new();

        if self.check(TokenType::RightParenthesis) 
        {
            return Ok(args);
        }

        loop 
        {
            let arg_result = self.parse_expression();
                    
            if arg_result.is_err()
            {
                return Err(arg_result.unwrap_err());
            }

            args.push(arg_result.unwrap());

            if !self.check(TokenType::Comma) { break; }
            self.consume();
        }

        return Ok(args);
    }
}