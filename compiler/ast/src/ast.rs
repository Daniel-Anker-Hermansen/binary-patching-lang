#[derive(Debug, Clone, Copy)]
pub enum Type {
    Unsigned32,
    Unsigned64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Global<'src> {
    pub name: &'src str
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Local<'src> {
    pub name: &'src str
}

#[derive(Debug, Clone, Copy)]
pub struct Argument<'src> {
    pub name: Local<'src>,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct Function<'src> {
    pub name: Global<'src>,
    pub arguments: Vec<Argument<'src>>,
    pub return_type: Type,
    pub body: Vec<Expression<'src>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Literal<'src> {
    pub src: &'src str,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum Expression<'src> {
    Literal {
        literal: Literal<'src>,
    },
    Assignment {
        new_definition: bool,
        r#type: Option<Type>,
        local: Local<'src>,
        value: Box<Expression<'src>>,
    },
    Call {
        function: Global<'src>,
        arguments: Vec<Expression<'src>>,
    },
    BinaryOperator {
        binary_operator: BinaryOperator,
        left: Box<Expression<'src>>,
        right: Box<Expression<'src>>,
    }
}
