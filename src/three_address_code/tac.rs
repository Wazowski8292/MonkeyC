use crate::semantic_analyzer::TokenType;

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
    Equals,

    LogicalEquals,
    NotEquals,
    GreaterThan,
    LessThan,
    LogicalAnd,
    LogicalOr,
    Not,

    RightBitShift,
    LeftBitShift,
    And,
    Or,

    Unknow,
}

impl Operator {
    pub fn from_str(operator: &str) -> Operator {
        match operator {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiplication,
            "/" => Operator::Division,
            "=" => Operator::Equals,

            "==" => Operator::LogicalEquals,
            "!=" => Operator::NotEquals,
            ">" => Operator::GreaterThan,
            "<" => Operator::LessThan,
            "&&" => Operator::LogicalAnd,
            "||" => Operator::LogicalOr,
            "!" => Operator::Not,

            ">>" => Operator::RightBitShift,
            "<<" => Operator::LeftBitShift,
            "&" => Operator::And,
            "|" => Operator::Or,

            _ => Operator::Unknow,
        }
    }

    pub fn _as_str(&self) -> &'static str {
        match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiplication => "*",
            Operator::Division => "/",
            Operator::Equals => "=",
            Operator::LogicalEquals => "==",
            Operator::NotEquals => "!=",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::LogicalAnd => "&&",
            Operator::LogicalOr => "||",
            Operator::Not => "!",
            Operator::RightBitShift => ">>",
            Operator::LeftBitShift => "<<",
            Operator::And => "&",
            Operator::Or => "|",
            Operator::Unknow => "?",
        }
    }

    pub fn to_asm_op(&self) -> Option<&'static str> {
        match self {
            Operator::Plus => Some("add"),
            Operator::Minus => Some("sub"),
            Operator::Multiplication => Some("imul"),
            Operator::RightBitShift => Some("sar"),
            Operator::LeftBitShift => Some("shl"),
            Operator::And => Some("and"),
            Operator::Or => Some("or"),
            _ => None,
        }
    }

    pub fn to_asm_op_f32(&self) -> Option<&'static str> {
        match self {
            Operator::Plus => Some("addss"),
            Operator::Minus => Some("subss"),
            Operator::Multiplication => Some("mulss"),
            Operator::Division => Some("divss"),
            _ => None,
        }
    }

    pub fn to_asm_op_f64(&self) -> Option<&'static str> {
        match self {
            Operator::Plus => Some("addsd"),
            Operator::Minus => Some("subsd"),
            Operator::Multiplication => Some("mulsd"),
            Operator::Division => Some("divsd"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Function,
    Param,
    Variable,
    ArrayDecl,
    ArrayIndex,
    ArrayAssign,
    AddressOf,
    Deref,
    DerefAssign,
    Call,
    Reasingment,
    Conditional,
    ConditionalEnd,
    Loop,
    LoopEnd,
    Label,
    GetReturn,
    Return,
    InlineAssembly,
    Jump,
}

#[derive(Debug, Clone)]
pub struct Tac {
    pub tac_type: Type,
    pub arguments: Vec<String>,
    pub operator: Option<Operator>,
    pub result: Option<String>,
    pub value_type: Option<TokenType>,
    pub is_ptr: bool,
}

