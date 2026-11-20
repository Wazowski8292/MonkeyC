pub mod conditional;
pub mod function_calls;
pub mod functions;
pub mod in_line_assembly;
pub mod include;
pub mod loops;
pub mod pending_checks;
pub mod reasignment;
pub mod r#return;
pub mod struct_literals;
pub mod variable;

// Module aliases for backwards compatibility or alternative naming
pub use conditional as condition;
pub use functions as function;
pub use r#return as return_statement;

pub use conditional::*;
pub use function_calls::*;
pub use functions::*;
pub use in_line_assembly::*;
pub use include::*;
pub use loops::*;
pub use pending_checks::*;
pub use reasignment::*;
pub use r#return::*;
pub use struct_literals::*;
pub use variable::*;
