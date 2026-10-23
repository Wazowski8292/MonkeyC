use super::{TableTypes, TokenType};
use crate::variable_types::{FunctionCall, PointerType, Value};

pub fn print(table: &[TableTypes]) {
    println!("\n---------------------- \n");
    println!("{}", format_table(table, 0));
    println!("\n---------------------- \n");
}

fn format_table(table: &[TableTypes], level: usize) -> String {
    table.iter().map(|item| format_item(item, level)).collect::<Vec<_>>().join("\n")
}

fn format_item(item: &TableTypes, level: usize) -> String {
    let p = "    ".repeat(level);
    match item {
        TableTypes::Variable(v) => {
            let array_note = if v.is_array { format!(" [array; size={:?}]", v.array_size) } else { String::new() };
            format!(
                "{p}var {}{}: {:?} = {}{}",
                format_ptr(&v.ptr),
                v.name.clone().unwrap_or_else(|| "_".to_string()),
                v.token_type,
                format_values(&v.value),
                array_note,
            )
        }

        TableTypes::Function(f) => {
            let ret = f.return_type.as_ref().map(|t| t.to_str()).unwrap_or_else(|| "void".to_string());
            let header = format!("{p}fn {}() -> {} {{", f.name.clone().unwrap_or_else(|| "_".to_string()), ret);
            let body = format_table(&f.table, level + 1);
            format!("{header}\n{body}\n{p}}}")
        }

        TableTypes::Reasingment(r) => {
            let params = r.parameters.as_ref().map(|p| p.iter().map(format_item_inline).collect::<Vec<_>>().join(", ")).unwrap_or_default();
            let idx = r.array_index.as_ref().map(|i| format!("[{}]", i)).unwrap_or_default();
            format!(
                "{p}{}{}{} = {}  ({:?}, scope={:?}, target={})",
                format_ptr(&r.ptr),
                r.name,
                idx,
                params,
                r.token_type,
                r.target_scope,
                r.target,
            )
        }

        TableTypes::FunctionCall(fc) => {
            format!("{p}{}  (scope={:?}, target={})", format_func_call_inline(fc), fc.scope, fc.target)
        }

        TableTypes::Conditional(c) => {
            let kw = if c.token_type == TokenType::Else { "else" } else { "if" };
            let is_plain = match c.condition.first() {
                Some(TableTypes::Variable(v)) => v.value.as_ref().map_or(true, |vals| vals.is_empty()),
                _ => false,
            };
            let header = if is_plain {
                format!("{p}{kw} {{")
            } else {
                let cond = c.condition.iter().map(format_item_inline).collect::<Vec<_>>().join(" ");
                format!("{p}{kw} {} {{", cond)
            };
            let body = format_table(&c.table, level + 1);
            format!("{header}\n{body}\n{p}}}")
        }

        TableTypes::Loop(l) => {
            let cond = l.condition.iter().map(format_item_inline).collect::<Vec<_>>().join(" ");
            let header = format!("{p}while {} {{", cond);
            let body = format_table(&l.table, level + 1);
            format!("{header}\n{body}\n{p}}}")
        }

        TableTypes::Return(r) => {
            let val = r.value.as_ref().map(|v| format_values(&v.value)).unwrap_or_else(|| "<none>".to_string());
            format!("{p}return {}", val)
        }

        TableTypes::StructLiteral(s) => {
            let header = format!("{p}struct {} {{", s.name);
            let mut body = format_table(&s.arguments, level + 1);
            if !s.functions.is_empty() {
                body.push_str(&format!("\n{p}    -- methods --\n"));
                body.push_str(&format_table(&s.functions, level + 1));
            }
            format!("{header}\n{body}\n{p}}}")
        }

        TableTypes::InlineAssembly(asm) => {
            format!("{p}asm: {:?}", asm.table)
        }

        TableTypes::Unknown => {
            format!("{p}<unknown>")
        }
    }
}

fn format_item_inline(item: &TableTypes) -> String {
    match item {
        TableTypes::Variable(v) => {
            if let Some(name) = &v.name
                && name != "_"
            {
                name.clone()
            } else {
                format_values(&v.value)
            }
        }
        TableTypes::FunctionCall(fc) => format_func_call_inline(fc),
        TableTypes::Reasingment(r) => r.name.clone(),
        _ => "<expr>".to_string(),
    }
}

fn format_func_call_inline(fc: &FunctionCall) -> String {
    let params = fc.parameters.as_ref().map(|p| p.iter().map(format_item_inline).collect::<Vec<_>>().join(", ")).unwrap_or_default();
    format!("{}({})", fc.name, params)
}

fn format_ptr(ptr: &Option<PointerType>) -> &'static str {
    match ptr {
        Some(PointerType::Pointer) => "*",
        Some(PointerType::Reference) => "&",
        None => "",
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Var(name) => name.clone(),
        Value::Deref(name) => format!("*{}", name),
        Value::Ref(name) => format!("&{}", name),
        Value::Index(name, idx) => format!("{}[{}]", name, idx),
        Value::FuncCall(fc) => format_func_call_inline(fc),
    }
}

fn format_values(values: &Option<Vec<Value>>) -> String {
    match values {
        Some(v) if !v.is_empty() => v.iter().map(format_value).collect::<Vec<_>>().join(" "),
        _ => "<none>".to_string(),
    }
}
