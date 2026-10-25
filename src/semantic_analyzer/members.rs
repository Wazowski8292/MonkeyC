use super::{Modifiers, SemanticAnalyzer, TableTypes, TokenType};
use crate::variable_types::{PointerType, Value};

impl SemanticAnalyzer {
    pub(super) fn eliminate_instance_placeholder(&mut self, name: &str) {
        match self.active_table().last_mut() {
            Some(TableTypes::FunctionCall(fc)) => {
                if let Some(params) = fc.parameters.as_mut() {
                    if matches!(params.last(), Some(TableTypes::Reasingment(r)) if r.name == name && r.parameters.as_ref().map_or(true, |p| p.is_empty()))
                    {
                        params.pop();
                        return;
                    }
                }
            }
            Some(TableTypes::Reasingment(reasing)) => {
                if let Some(params) = reasing.parameters.as_mut() {
                    if matches!(params.last(), Some(TableTypes::Reasingment(r)) if r.name == name && r.parameters.as_ref().map_or(true, |p| p.is_empty()))
                    {
                        params.pop();
                        return;
                    }
                } else if reasing.name == name && reasing.parameters.as_ref().map_or(true, |p| p.is_empty()) {
                    self.modifiers.push(Modifiers::StandAloneStructCall);
                }
            }
            Some(TableTypes::Variable(v)) => {
                if let Some(vals) = v.value.as_mut() {
                    if matches!(vals.last(), Some(Value::Var(n)) if n == name) {
                        vals.pop();
                    }
                }
            }
            Some(TableTypes::Return(r)) => {
                if let Some(vals) = r.value.as_mut().and_then(|v| v.value.as_mut()) {
                    if matches!(vals.last(), Some(Value::Var(n)) if n == name) {
                        vals.pop();
                    }
                }
            }
            Some(TableTypes::Conditional(c)) => {
                if matches!(c.condition.last(), Some(TableTypes::Reasingment(r)) if r.name == name && r.parameters.as_ref().map_or(true, |p| p.is_empty()))
                {
                    c.condition.pop();
                }
            }
            Some(TableTypes::Loop(l)) => {
                if matches!(l.condition.last(), Some(TableTypes::Reasingment(r)) if r.name == name && r.parameters.as_ref().map_or(true, |p| p.is_empty()))
                {
                    l.condition.pop();
                }
            }
            _ => {}
        }
    }

    pub(super) fn find_preceding_instance_name(&self) -> Option<String> {
        let active = self.active_table_ref();
        match active.last() {
            Some(TableTypes::FunctionCall(fc)) => {
                if let Some(p) = fc.parameters.as_ref().and_then(|p| p.last()) {
                    match p {
                        TableTypes::Reasingment(r) => Some(r.name.clone()),
                        TableTypes::Variable(v) => v.name.clone(),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Some(TableTypes::Reasingment(r)) => {
                if let Some(p) = r.parameters.as_ref().and_then(|p| p.last()) {
                    match p {
                        TableTypes::Reasingment(inner) => Some(inner.name.clone()),
                        TableTypes::Variable(v) => v.name.clone(),
                        _ => None,
                    }
                } else {
                    Some(r.name.clone())
                }
            }
            Some(TableTypes::Variable(v)) => {
                if let Some(vals) = v.value.as_ref().and_then(|v| v.last()) {
                    if let Value::Var(name) = vals { Some(name.clone()) } else { v.name.clone() }
                } else {
                    v.name.clone()
                }
            }
            Some(TableTypes::Return(r)) => {
                let var = r.value.as_ref()?;
                match var.value.as_ref().and_then(|v| v.last()) {
                    Some(Value::Var(name)) => Some(name.clone()),
                    _ => var.name.clone(),
                }
            }
            Some(TableTypes::Conditional(c)) => match c.condition.last() {
                Some(TableTypes::Reasingment(r)) => Some(r.name.clone()),
                Some(TableTypes::Variable(v)) => v.name.clone(),
                _ => None,
            },
            Some(TableTypes::Loop(l)) => match l.condition.last() {
                Some(TableTypes::Reasingment(r)) => Some(r.name.clone()),
                Some(TableTypes::Variable(v)) => v.name.clone(),
                _ => None,
            },
            _ => None,
        }
    }

    pub(super) fn push_member_entry(&mut self, entry: TableTypes) {
        let last_finished = self.last_finished;
        let def_parameters = self.defining_parameters;

        if let Some(pos) = self.modifiers.iter().rposition(|e| matches!(e, Modifiers::StandAloneStructCall)) {
            self.modifiers.remove(pos);
            self.active_table().pop();
            self.active_table().push(entry);
            return;
        }

        let leftover = match self.active_table().last_mut() {
            Some(last) => Self::attach_member(last, entry, last_finished, def_parameters),
            None => Some(entry),
        };

        if let Some(entry) = leftover {
            self.active_table().push(entry);
        }
    }

    fn attach_member(target: &mut TableTypes, entry: TableTypes, last_finished: bool, defining_parameters: bool) -> Option<TableTypes> {
        if let Some(fc) = Self::pending_call_in_mut(target) {
            fc.parameters.get_or_insert_with(Vec::new).push(entry);
            return None;
        }

        match target {
            TableTypes::FunctionCall(call) => {
                call.parameters.get_or_insert_with(Vec::new).push(entry);
                None
            }
            TableTypes::Reasingment(reasign) => {
                if let TokenType::StructDef(_) = reasign.token_type.clone()
                    && let TableTypes::Reasingment(reasing_entry) = entry
                {
                    reasign.token_type = TokenType::Unknow;
                    reasign.array_index = reasing_entry.array_index;
                } else {
                    reasign.parameters.get_or_insert_with(Vec::new).push(entry);
                }
                None
            }
            TableTypes::Variable(var) if !last_finished => match Self::member_to_values(entry) {
                Ok(values) => {
                    var.value.get_or_insert_with(Vec::new).extend(values);
                    None
                }
                Err(entry) => Some(entry),
            },
            TableTypes::Return(ret) => match ret.value.as_mut() {
                Some(var) => match Self::member_to_values(entry) {
                    Ok(values) => {
                        var.value.get_or_insert_with(Vec::new).extend(values);
                        None
                    }
                    Err(entry) => Some(entry),
                },
                None => Some(entry),
            },
            TableTypes::Conditional(c) => match c.condition.last_mut() {
                Some(last) => match Self::attach_member(last, entry, last_finished, defining_parameters) {
                    Some(entry) => {
                        c.condition.push(entry);
                        None
                    }
                    None => None,
                },
                None => {
                    c.condition.push(entry);
                    None
                }
            },
            TableTypes::Loop(l) => match l.condition.last_mut() {
                Some(last) => match Self::attach_member(last, entry, last_finished, defining_parameters) {
                    Some(entry) => {
                        l.condition.push(entry);
                        None
                    }
                    None => None,
                },
                None => {
                    l.condition.push(entry);
                    None
                }
            },
            _ => Some(entry),
        }
    }

    fn member_to_values(entry: TableTypes) -> Result<Vec<Value>, TableTypes> {
        match entry {
            TableTypes::FunctionCall(fc) => Ok(vec![Value::FuncCall(fc)]),
            TableTypes::Reasingment(r) => {
                let name = r.name;
                let value = match (r.array_index, r.ptr) {
                    (Some(idx), _) => Value::Index(name, idx),
                    (None, Some(PointerType::Reference)) => Value::Ref(name),
                    (None, Some(PointerType::Pointer)) => Value::Deref(name),
                    (None, None) => Value::Var(name),
                };
                Ok(vec![value])
            }
            TableTypes::Variable(mut v) => match v.value.take() {
                Some(vals) if !vals.is_empty() => Ok(vals),
                other => {
                    v.value = other;
                    Err(TableTypes::Variable(v))
                }
            },
            other => Err(other),
        }
    }
}
