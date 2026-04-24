/////////////////////////////////////////////////////
// Type
/////////////////////////////////////////////////////
#[derive(Debug)]
pub enum Type
{
    Void,
    Bool,
    Int8, Int16, Int32, Int64,
    UInt8, UInt16, UInt32, UInt64,
    Float32, Float64,
    String,

    Named(String)
}

/////////////////////////////////////////////////////
// Expression related
/////////////////////////////////////////////////////
#[derive(Debug)]
pub enum Expression
{
    BooleanLiteral(bool),
    IntLiteral(i64),        // FUTURE TODO: Different integer sizes and unsigned
    FloatLiteral(f64),      // FUTURE TODO: Different integer sizes and unsigned
    
    StringLiteral(String),
    Identifier(String),

    BinaryOp {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Call {
        callee: String,
        arguments: Vec<Expression>,
    },
    // FUTURE TODO: UnaryOp, Index, FieldAccess, etc.
}

#[derive(Debug)]
pub enum BinaryOperator 
{
    Addition, Subtraction, Multiplication, Division,
    
    EqualsEquals, NotEquals,
    
    LessThan, GreaterThan,
    LessThanOrEquals, GreaterThanOrEquals,

    And /*&&*/, Or /*||*/
}

/////////////////////////////////////////////////////
// Statement related
/////////////////////////////////////////////////////
#[derive(Debug)]
pub enum Statement 
{
    Variable {
        name: String,
        variable_type: Type,
        initializer: Option<Expression>,
    },
    Return(Expression),
    Expression(Expression),
    // FUTURE TODO: If, While, For, etc.
}

#[derive(Debug)]
pub struct Block 
{
    pub statements: Vec<Statement>,
}

/////////////////////////////////////////////////////
// Function related
/////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Parameter
{
    pub name: String,
    pub parameter_type: Type,
}

#[derive(Debug)]
pub struct FunctionDefinition
{
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
}

/////////////////////////////////////////////////////
// Item
/////////////////////////////////////////////////////
#[derive(Debug)]
pub enum Item
{
    Function(FunctionDefinition),
    // FUTURE TODO: Enum(EnumDef),
    // FUTURE TODO: Class(ClassDef),
    // FUTURE TODO: Struct(StructDef),
}