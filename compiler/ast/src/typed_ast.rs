use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Unsigned32,
    Unsigned64,
    Error,
    Temporary {
        index: usize,
    },
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Global {
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Global,
    pub arguments: Vec<Type>,
    pub return_type: Type,
    pub body: Vec<Expression>,
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    Unsigned32(u32),
    Unsigned64(u64),
    UnsignedUnknown(u64),
}

#[derive(Debug, Clone, Copy)]
pub struct Local {
    pub index: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal {
        literal: Literal,
    },
    Assignment {
        local: Local,
        value: Box<Expression>,
    },
    Call {
        function: Global,
        arguments: Vec<Expression>,
        return_type: Type,
    },
    BinaryOperator {
        binary_operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
        result_type: Type,
    }
}

pub struct FunctionMapping {

}

impl FunctionMapping {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn add(&mut self, function: &Function) {
        unimplemented!()
    }

    pub fn find(&self, function: &Function) -> Option<(Global, HashMap<Global, Global>)> {
        unimplemented!()
    }
}
