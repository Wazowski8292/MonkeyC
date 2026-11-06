#[derive(Debug, Clone)]
pub enum WarningCodes {
    UnsafeAssembly,
    UnusedVariable,
    UnusedFunction,
    UnreachableCode,
    ImplicitTypeCast,
    ShadowedVariable,
    MissingReturnValue,
    DeprecatedFeature,
    IncludedFileMultipleTimes,
}

impl WarningCodes {
    pub fn to_text(&self) -> String {
        match self {
            WarningCodes::UnsafeAssembly => "Unsafe inline assembly".to_string(),
            WarningCodes::UnusedVariable => "Unused variable".to_string(),
            WarningCodes::UnusedFunction => "Unused function".to_string(),
            WarningCodes::UnreachableCode => "Unreachable code".to_string(),
            WarningCodes::ImplicitTypeCast => "Implicit type cast".to_string(),
            WarningCodes::ShadowedVariable => "Shadowed variable".to_string(),
            WarningCodes::MissingReturnValue => "Missing return value".to_string(),
            WarningCodes::DeprecatedFeature => "Deprecated feature".to_string(),
            WarningCodes::IncludedFileMultipleTimes => "Included file multiple times".to_string(),
        }
    }

    pub fn code(&self) -> String {
        match self {
            WarningCodes::UnsafeAssembly => "W001".to_string(),
            WarningCodes::UnusedVariable => "W002".to_string(),
            WarningCodes::UnusedFunction => "W003".to_string(),
            WarningCodes::UnreachableCode => "W004".to_string(),
            WarningCodes::ImplicitTypeCast => "W005".to_string(),
            WarningCodes::ShadowedVariable => "W006".to_string(),
            WarningCodes::MissingReturnValue => "W007".to_string(),
            WarningCodes::DeprecatedFeature => "W008".to_string(),
            WarningCodes::IncludedFileMultipleTimes => "w009".to_string(),
        }
    }
}
