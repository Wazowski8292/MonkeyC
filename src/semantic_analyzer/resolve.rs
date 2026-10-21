use super::{ResolveType, Scope, SemanticAnalyzer, TableTypes};
use crate::enbeded_funcs::FUNCTIONS;
use crate::variable_types::parse_array_syntax;

pub(super) type Resolution = (usize, Scope, ResolveType);

fn embedded_func_index(name: &str) -> Option<usize> {
    FUNCTIONS.iter().position(|func| func.name == name)
}

fn scope_for_level(nest_level: usize) -> Scope {
    if nest_level == 0 { Scope::Root } else { Scope::Function }
}

fn find_in_level(name: &str, table: &[TableTypes], nest_level: usize) -> Option<Resolution> {
    let embedded = embedded_func_index(name);

    table.iter().enumerate().find_map(|(idx, entry)| match entry {
        TableTypes::Variable(v) if v.name.as_deref() == Some(name) => Some((idx, scope_for_level(nest_level), ResolveType::Variable)),
        TableTypes::Function(f) if f.name.as_deref() == Some(name) => Some((idx, scope_for_level(nest_level), ResolveType::Function)),
        TableTypes::StructLiteral(s) if s.name == name => Some((idx, scope_for_level(nest_level), ResolveType::Struct)),
        _ => embedded.map(|index| (index, Scope::EnbedFunc, ResolveType::Function)),
    })
}

fn should_descend(table: &[TableTypes], current_nest_level: usize, max_nesting: usize, defining_parameters: bool) -> bool {
    let has_child = matches!(
        table.last(),
        Some(TableTypes::Function(_) | TableTypes::Conditional(_) | TableTypes::Loop(_) | TableTypes::StructLiteral(_))
    );
    if !has_child {
        return false;
    }

    let nesting_exhausted = current_nest_level + 1 > max_nesting && !defining_parameters;
    let blocked_by_params = defining_parameters && matches!(table.last(), Some(TableTypes::Conditional(_)));

    !nesting_exhausted && !blocked_by_params
}

fn loop_variable(condition: &[TableTypes], name: &str) -> Option<Resolution> {
    if condition.len() > 1
        && let Some(TableTypes::Variable(variable)) = condition.first()
        && variable.name.as_deref() == Some(name)
    {
        return Some((0, Scope::Parameter, ResolveType::Variable));
    }

    None
}

fn find_loop_var(table: &[TableTypes], name: &str) -> Option<Resolution> {
    match table.last() {
        Some(TableTypes::Conditional(c)) => find_loop_var(&c.table, name),
        Some(TableTypes::Loop(l)) => find_loop_var(&l.table, name).or_else(|| loop_variable(&l.condition, name)),
        _ => None,
    }
}

impl SemanticAnalyzer {
    pub(super) fn resolve(&mut self, name: String) -> Option<Resolution> {
        if name.starts_with("self") {
            return Some((0, Scope::Root, ResolveType::Variable));
        }

        let lookup_name = parse_array_syntax(&name).map(|(array_name, _)| array_name).unwrap_or(name);

        Self::resolve_in_chain(&lookup_name, &mut self.table, 0, self.max_nesting, self.defining_parameters)
            .or_else(|| self.resolve_in_parameters(&lookup_name))
    }

    pub(super) fn resolve_in_chain(
        name: &str, table: &mut Vec<TableTypes>, current_nest_level: usize, max_nesting: usize, defining_parameters: bool,
    ) -> Option<Resolution> {
        if should_descend(table, current_nest_level, max_nesting, defining_parameters) {
            if let Some(found) = Self::descend_and_resolve(name, table, current_nest_level, max_nesting, defining_parameters) {
                return Some(found);
            }
        }

        find_in_level(name, table, current_nest_level)
    }

    pub(super) fn resolve_in_parameters(&self, name: &str) -> Option<Resolution> {
        if !self.defining_fn {
            return None;
        }

        match self.table.last() {
            Some(TableTypes::Function(f)) => {
                if let Some(found) = find_loop_var(&f.table, name) {
                    return Some(found);
                }

                let params = f.parameters.as_ref()?;
                params.iter().enumerate().find_map(|(idx, entry)| match entry {
                    TableTypes::Variable(v) if v.name.as_deref() == Some(name) => Some((idx, Scope::Parameter, ResolveType::Variable)),
                    _ => None,
                })
            }
            Some(TableTypes::Loop(looping)) => loop_variable(&looping.condition, name),
            _ => None,
        }
    }

    fn descend_and_resolve(
        name: &str, table: &mut Vec<TableTypes>, current_nest_level: usize, max_nesting: usize, defining_parameters: bool,
    ) -> Option<Resolution> {
        let (child, skip_when_empty) = match table.last_mut().unwrap() {
            TableTypes::Function(func) => (&mut func.table, true),
            TableTypes::Loop(looping) => (&mut looping.table, true),
            TableTypes::Conditional(con) => (&mut con.table, false),
            TableTypes::StructLiteral(struct_def) => (&mut struct_def.functions, false),
            _ => unreachable!(),
        };

        if skip_when_empty && defining_parameters && child.is_empty() {
            return None;
        }

        Self::resolve_in_chain(name, child, current_nest_level + 1, max_nesting, defining_parameters)
    }
}
