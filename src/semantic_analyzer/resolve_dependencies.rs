use super::SemanticAnalyzer;
use super::TableTypes;
use crate::variable_types::PendingCheck;
use crate::report;
use crate::error_handler::error_codes::ErrorCodes;

impl SemanticAnalyzer {
    pub fn resolve_pending_checks(&mut self, dependencies: Vec<(String, Vec<TableTypes>)>) {
        let file_name = self.file_name.clone();
        for pending_check in self.pending_checks.iter_mut() {
            if !Self::is_defined_in_dependency(dependencies.clone(), pending_check.clone()) {
                report!(file_name.clone(), ErrorCodes::UndefinedSymbol, pending_check.line, pending_check.char_num, "{} is not defined, or it is not public, in the corresponding file: {}.MC", pending_check.symbol, pending_check.module); 
            }
        }
    }

    fn is_defined_in_dependency(dependencies: Vec<(String, Vec<TableTypes>)>, pending_check: PendingCheck) -> bool {
        for (name, dependency) in dependencies {
            if name == pending_check.module {
                let entry = dependency.iter().find(|dep| dep.get_name().unwrap_or(String::new()) == pending_check.symbol);
                if entry.is_some() {
                    return true;
                }
            }
        }
        false
    }
}
