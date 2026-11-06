#[derive(Debug, Clone)]
pub enum ErrorCodes {
    TypeMismatch,
    ParameterMismatch,
    ParameterLengthMismatch,
    UndefinedSymbol,
    NoMainFunction,
    VariableRedeclaration,
    FunctionRedeclaration,
    InvalidMemberAccess,
    MissingEntry,
    FileNotFound,
}

impl ErrorCodes {
    pub fn to_text(&self) -> String {
        match self {
            ErrorCodes::TypeMismatch => "Type mismatch".to_string(),
            ErrorCodes::ParameterMismatch => "Parameter type mismatch".to_string(),
            ErrorCodes::ParameterLengthMismatch => "Parameter length mismatch".to_string(),
            ErrorCodes::UndefinedSymbol => "Undefined symbol".to_string(),
            ErrorCodes::NoMainFunction => "No main function".to_string(),
            ErrorCodes::VariableRedeclaration => "Variable redeclaration".to_string(),
            ErrorCodes::FunctionRedeclaration => "Function redeclaration".to_string(),
            ErrorCodes::InvalidMemberAccess => "Invalid member access".to_string(),
            ErrorCodes::MissingEntry => "Missing entry".to_string(),
            ErrorCodes::FileNotFound => "File not found".to_string(),
        }
    }

    pub fn code(&self) -> String {
        match self {
            ErrorCodes::TypeMismatch => "E001".to_string(),
            ErrorCodes::ParameterMismatch => "E002".to_string(),
            ErrorCodes::ParameterLengthMismatch => "E003".to_string(),
            ErrorCodes::UndefinedSymbol => "E004".to_string(),
            ErrorCodes::NoMainFunction => "E005".to_string(),
            ErrorCodes::VariableRedeclaration => "E006".to_string(),
            ErrorCodes::FunctionRedeclaration => "E007".to_string(),
            ErrorCodes::InvalidMemberAccess => "E008".to_string(),
            ErrorCodes::MissingEntry => "E009".to_string(),
            ErrorCodes::FileNotFound => "E010".to_string(),
        }
    }

    pub fn code_to_text(code: ErrorCodes) -> String {
        code.to_text()
    }
}
