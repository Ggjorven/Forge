/////////////////////////////////////////////////////
// TokenType
/////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)] // PartialEq just checks the Kind and not the internal type
pub enum TokenType
{
    // Literals
    Int64Literal(i64),       // FUTURE TODO: unsigned and different sizes
    Float64Literal(f64),     // FUTURE TODO: different sizes
    CharLiteral(char),
    StringLiteral(String),

    // Keywords
    Let,
    Return,
    True,
    False,

    // Types & Identifier
    Void,
    Bool,
    Char,
    Int8, Int16, Int32, Int64,
    UInt8, UInt16, UInt32, UInt64,
    Float32, Float64,
    String,
    Identifier(String),

    // Punctuation
    LeftParenthesis, RightParenthesis,  // ( )
    LeftBrace, RightBrace,              // { }
    Colon,                              // :
    Semicolon,                          // ;
    Comma,                              // ,
    Dot,                                // .
    Equals,                             // =
    Arrow,                              // ->
    
    // Operators
    Plus, Minus, Star, Slash,                           // +, -, *, /
    PlusEquals, MinusEquals, StarEquals, SlashEquals,   // +=, -=, *=, /=
    PlusPlus, MinusMinus,                               // ++, --

    EqualsEquals, NotEquals,                // ==, !=

    LessThan, GreaterThan,                  // <, >
    LessThanOrEquals, GreaterThanOrEquals,  // <=, >=
}

/////////////////////////////////////////////////////
// Token
/////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Token
{
    pub token_type: TokenType,
    
    // Debug
    pub line: u32
}