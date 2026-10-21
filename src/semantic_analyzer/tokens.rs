#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    If,
    Else,

    Int,
    Float,
    Double,
    Bool,
    Char,
    String,

    Plus,
    Minus,
    Multiplication,
    Division,
    Equals,
    PlusEquals,
    MinusEquals,
    PlusPlus,
    MinusMinus,

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

    IntegerLiteral,
    FloatLiteral,
    DoubleLiteral,
    CharLiteral,
    StringLiteral,
    BoolLiteral,

    FnLiteral,
    Return,
    ReturnType,

    StructLiteral,
    StructDef(String),

    WhileLoop,

    Assembly,

    ForLoop,
    DoWhileLoop,

    Dots,
    Unknow,
}

impl TokenType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "if" => TokenType::If,
            "else" => TokenType::Else,

            "int" => TokenType::Int,
            "float" => TokenType::Float,
            "double" => TokenType::Double,
            "str" => TokenType::String,
            "bool" => TokenType::Bool,
            "char" => TokenType::Char,

            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "*" => TokenType::Multiplication,
            "/" => TokenType::Division,
            "=" => TokenType::Equals,
            "+=" => TokenType::PlusEquals,
            "-=" => TokenType::MinusEquals,
            "++" => TokenType::PlusPlus,
            "--" => TokenType::MinusMinus,

            "==" => TokenType::LogicalEquals,
            "!=" => TokenType::NotEquals,
            ">" => TokenType::GreaterThan,
            "<" => TokenType::LessThan,
            "&&" => TokenType::LogicalAnd,
            "||" => TokenType::LogicalOr,
            "!" => TokenType::Not,

            ">>" => TokenType::RightBitShift,
            "<<" => TokenType::LeftBitShift,
            "&" => TokenType::And,
            "|" => TokenType::Or,

            "fn" => TokenType::FnLiteral,
            "return" => TokenType::Return,
            "->" => TokenType::ReturnType,

            "struct" => TokenType::StructLiteral,

            "while" => TokenType::WhileLoop,
            "for" => TokenType::ForLoop,
            "do" => TokenType::DoWhileLoop,

            "::" => TokenType::Dots,

            "asm" => TokenType::Assembly,

            "true" | "false" => TokenType::BoolLiteral,

            _ if s.parse::<i64>().is_ok() => TokenType::IntegerLiteral,
            _ if (s.ends_with('f') || s.ends_with('F')) && s[..s.len() - 1].parse::<f32>().is_ok() => TokenType::FloatLiteral,
            _ if s.parse::<f64>().is_ok() => TokenType::DoubleLiteral,
            _ if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') => TokenType::StringLiteral,
            _ if s.len() >= 3 && s.starts_with('\'') && s.ends_with('\'') => TokenType::CharLiteral,

            _ => TokenType::Unknow,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            TokenType::If => "if".to_string(),
            TokenType::Else => "else".to_string(),

            TokenType::Int => "int".to_string(),
            TokenType::Float => "float".to_string(),
            TokenType::Double => "double".to_string(),
            TokenType::String => "str".to_string(),
            TokenType::Bool => "bool".to_string(),
            TokenType::Char => "char".to_string(),

            TokenType::Plus => "+".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Multiplication => "*".to_string(),
            TokenType::Division => "/".to_string(),
            TokenType::Equals => "=".to_string(),
            TokenType::PlusEquals => "+=".to_string(),
            TokenType::MinusEquals => "-=".to_string(),
            TokenType::PlusPlus => "++".to_string(),
            TokenType::MinusMinus => "--".to_string(),

            TokenType::LogicalEquals => "==".to_string(),
            TokenType::NotEquals => "!=".to_string(),
            TokenType::GreaterThan => ">".to_string(),
            TokenType::LessThan => "<".to_string(),
            TokenType::LogicalAnd => "&&".to_string(),
            TokenType::LogicalOr => "||".to_string(),
            TokenType::Not => "!".to_string(),

            TokenType::RightBitShift => ">>".to_string(),
            TokenType::LeftBitShift => "<<".to_string(),
            TokenType::And => "&".to_string(),
            TokenType::Or => "|".to_string(),

            TokenType::FnLiteral => "fn".to_string(),
            TokenType::Return => "return".to_string(),
            TokenType::ReturnType => "->".to_string(),

            TokenType::StructLiteral => "struct".to_string(),
            TokenType::StructDef(name) => format!("struct type of {}", name.clone()),

            TokenType::WhileLoop => "while".to_string(),
            TokenType::ForLoop => "for".to_string(),
            TokenType::DoWhileLoop => "do".to_string(),

            TokenType::IntegerLiteral => "<integer literal>".to_string(),
            TokenType::FloatLiteral => "<float literal>".to_string(),
            TokenType::DoubleLiteral => "<double literal>".to_string(),
            TokenType::StringLiteral => "<string literal>".to_string(),
            TokenType::CharLiteral => "<char literal>".to_string(),
            TokenType::BoolLiteral => "<bool literal>".to_string(),

            TokenType::Dots => "::".to_string(),

            TokenType::Assembly => "asm".to_string(),

            TokenType::Unknow => "<unknown>".to_string(),
        }
    }

    pub fn is_value(token: TokenType) -> bool {
        token == TokenType::Unknow
            || token == TokenType::IntegerLiteral
            || token == TokenType::FloatLiteral
            || token == TokenType::DoubleLiteral
            || token == TokenType::BoolLiteral
            || token == TokenType::StringLiteral
            || token == TokenType::CharLiteral
    }

    pub fn is_operator(token: TokenType) -> bool {
        TokenType::is_aritmetic_operator(token.clone())
            || TokenType::is_binary_operator(token.clone())
            || TokenType::is_logical_operator(token.clone())
    }

    pub fn is_logical_operator(token: TokenType) -> bool {
        token == TokenType::LogicalEquals
            || token == TokenType::NotEquals
            || token == TokenType::GreaterThan
            || token == TokenType::LessThan
            || token == TokenType::LogicalAnd
            || token == TokenType::LogicalOr
            || token == TokenType::Not
    }

    pub fn is_aritmetic_operator(token: TokenType) -> bool {
        token == TokenType::Plus
            || token == TokenType::Minus
            || token == TokenType::Multiplication
            || token == TokenType::Division
            || token == TokenType::Equals
            || token == TokenType::PlusEquals
            || token == TokenType::MinusEquals
            || token == TokenType::PlusPlus
            || token == TokenType::MinusMinus
    }

    pub fn is_binary_operator(token: TokenType) -> bool {
        token == TokenType::RightBitShift || token == TokenType::LeftBitShift || token == TokenType::And || token == TokenType::Or
    }

    pub fn literal_type(&self) -> Option<TokenType> {
        match self {
            TokenType::IntegerLiteral => Some(TokenType::Int),
            TokenType::FloatLiteral => Some(TokenType::Float),
            TokenType::DoubleLiteral => Some(TokenType::Double),
            TokenType::StringLiteral => Some(TokenType::String),
            TokenType::CharLiteral => Some(TokenType::Char),
            TokenType::BoolLiteral => Some(TokenType::Bool),
            TokenType::Int | TokenType::Float | TokenType::Double | TokenType::String | TokenType::Bool | TokenType::Char => {
                Some(self.clone())
            }
            _ => None,
        }
    }
}

