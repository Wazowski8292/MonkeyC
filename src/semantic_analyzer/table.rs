use crate::variable_types::{
    Conditional, Function, FunctionCall, InlineAssembly, Loop, Reasingment, Return, StructLiteral,
    Types, Variable,
};
use super::tokens::TokenType;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scope {
    Root,
    Function,
    Parameter,
    EnbedFunc,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ResolveType {
    Variable,
    Function,
    Struct,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableTypes {
    Variable(Variable),
    Function(Function),
    Reasingment(Reasingment),
    FunctionCall(FunctionCall),
    Conditional(Conditional),
    Loop(Loop),
    Return(Return),
    StructLiteral(StructLiteral),
    InlineAssembly(InlineAssembly),
    Unknown,
}

impl TableTypes {
    pub fn from_token(token: TokenType) -> Self {
        if TokenType::is_operator(token.clone()) {
            return TableTypes::Variable(Variable::new(token));
        }

        match token {
            TokenType::FnLiteral => TableTypes::Function(Function::new(token)),
            TokenType::Return => TableTypes::Return(Return::new(token)),
            TokenType::If | TokenType::Else => TableTypes::Conditional(Conditional::new(token)),
            TokenType::WhileLoop | TokenType::ForLoop | TokenType::DoWhileLoop => TableTypes::Loop(Loop::new(token)),
            TokenType::Unknow => TableTypes::Reasingment(Reasingment::new(TokenType::Unknow)),
            TokenType::Int
            | TokenType::Float
            | TokenType::Double
            | TokenType::Bool
            | TokenType::Char
            | TokenType::String
            | TokenType::StructDef(_) => TableTypes::Variable(Variable::new(token)),
            TokenType::StructLiteral => TableTypes::StructLiteral(StructLiteral::new(token)),
            TokenType::Assembly => TableTypes::InlineAssembly(InlineAssembly::new(token)),
            _ if TokenType::is_value(token.clone()) && token.clone() != TokenType::Unknow => TableTypes::Variable(Variable::new(token)),
            _ => TableTypes::Unknown,
        }
    }

    pub fn add_arguments(&mut self, argument: String) {
        match self {
            TableTypes::Variable(var) => var.add_arguments(argument),
            TableTypes::Function(fun) => fun.add_arguments(argument),
            TableTypes::Reasingment(reasing) => reasing.add_arguments(argument),
            TableTypes::FunctionCall(fc) => fc.add_arguments(argument),
            TableTypes::Conditional(con) => con.add_arguments(argument),
            TableTypes::Loop(while_loop) => while_loop.add_arguments(argument),
            TableTypes::Return(returns) => returns.add_arguments(argument),
            TableTypes::StructLiteral(struct_literal) => struct_literal.add_arguments(argument),
            TableTypes::InlineAssembly(asm) => asm.add_arguments(argument),
            _ => {}
        }
    }
}
