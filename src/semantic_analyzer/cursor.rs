use super::{SemanticAnalyzer, TableTypes};
use crate::variable_types::Function;

impl SemanticAnalyzer {
    pub(super) fn active_table(&mut self) -> &mut Vec<TableTypes> {
        Self::desend_table(self.defining_parameters, self.defining_fn, &mut self.table, 1, self.max_nesting)
    }

    fn desend_table(
        defining_parameters: bool, defining_fn: bool, last_table: &mut Vec<TableTypes>, current_nest_level: usize, max_nesting: usize,
    ) -> &mut Vec<TableTypes> {
        if let Some(entry) = last_table.last() {
            if Self::pending_call_in(entry).is_some() {
                return last_table;
            }
        }

        let has_child = matches!(
            last_table.last(),
            Some(TableTypes::Function(_))
                | Some(TableTypes::Conditional(_))
                | Some(TableTypes::Loop(_))
                | Some(TableTypes::StructLiteral(_))
        );

        if !has_child || (current_nest_level == max_nesting) && !defining_parameters {
            return last_table;
        }

        if defining_parameters && max_nesting == current_nest_level {
            if let Some(TableTypes::Conditional(_)) = last_table.last() {
                return last_table;
            } else if let Some(TableTypes::Loop(_)) = last_table.last() {
                return last_table;
            }
        }

        match last_table.last_mut().unwrap() {
            TableTypes::Function(func) => {
                if defining_parameters && func.table.is_empty() {
                    func.parameters.get_or_insert_with(Vec::new)
                } else {
                    Self::desend_table(defining_parameters, defining_fn, &mut func.table, current_nest_level + 1, max_nesting)
                }
            }
            TableTypes::Conditional(con) => {
                Self::desend_table(defining_parameters, defining_fn, &mut con.table, current_nest_level + 1, max_nesting)
            }
            TableTypes::Loop(while_loop) => {
                Self::desend_table(defining_parameters, defining_fn, &mut while_loop.table, current_nest_level + 1, max_nesting)
            }
            TableTypes::StructLiteral(struct_literal) => {
                if defining_fn {
                    Self::desend_table(defining_parameters, defining_fn, &mut struct_literal.functions, current_nest_level + 1, max_nesting)
                } else {
                    Self::desend_table(defining_parameters, defining_fn, &mut struct_literal.arguments, current_nest_level + 1, max_nesting)
                }
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn active_table_ref(&self) -> &Vec<TableTypes> {
        Self::descend_table_ref(self.defining_parameters, self.defining_fn, &self.table, 1, self.max_nesting)
    }

    fn descend_table_ref<'a>(
        defining_parameters: bool, defining_fn: bool, last_table: &'a Vec<TableTypes>, current_nest_level: usize, max_nesting: usize,
    ) -> &'a Vec<TableTypes> {
        let has_child = matches!(
            last_table.last(),
            Some(TableTypes::Function(_))
                | Some(TableTypes::Conditional(_))
                | Some(TableTypes::Loop(_))
                | Some(TableTypes::StructLiteral(_))
        );

        if !has_child || (current_nest_level == max_nesting) && !defining_parameters {
            return last_table;
        }

        if defining_parameters && max_nesting == current_nest_level {
            if let Some(TableTypes::Conditional(_)) = last_table.last() {
                return last_table;
            } else if let Some(TableTypes::Loop(_)) = last_table.last() {
                return last_table;
            }
        }

        match last_table.last().unwrap() {
            TableTypes::Function(func) => {
                if defining_parameters && func.table.is_empty() {
                    func.parameters.as_ref().map(|p| p as &Vec<TableTypes>).unwrap_or(last_table)
                } else {
                    Self::descend_table_ref(defining_parameters, defining_fn, &func.table, current_nest_level + 1, max_nesting)
                }
            }
            TableTypes::Conditional(con) => {
                Self::descend_table_ref(defining_parameters, defining_fn, &con.table, current_nest_level + 1, max_nesting)
            }
            TableTypes::Loop(while_loop) => {
                Self::descend_table_ref(defining_parameters, defining_fn, &while_loop.table, current_nest_level + 1, max_nesting)
            }
            TableTypes::StructLiteral(struct_literal) => {
                if defining_fn {
                    Self::descend_table_ref(
                        defining_parameters,
                        defining_fn,
                        &struct_literal.functions,
                        current_nest_level + 1,
                        max_nesting,
                    )
                } else {
                    Self::descend_table_ref(
                        defining_parameters,
                        defining_fn,
                        &struct_literal.arguments,
                        current_nest_level + 1,
                        max_nesting,
                    )
                }
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn current_function(&self) -> Option<&Function> {
        match self.table.last() {
            Some(TableTypes::Function(f)) => Some(f),
            Some(TableTypes::StructLiteral(s)) if self.defining_fn => match s.functions.last() {
                Some(TableTypes::Function(f)) => Some(f),
                _ => None,
            },
            _ => None,
        }
    }

    pub(super) fn callee_table(&self, name: &str) -> (&Vec<TableTypes>, bool) {
        if let Some((struct_name, _)) = name.split_once('.') {
            for t in &self.table {
                if let TableTypes::StructLiteral(s) = t
                    && s.name == struct_name
                {
                    return (&s.functions, true);
                }
            }
        }

        (&self.table, false)
    }
}
