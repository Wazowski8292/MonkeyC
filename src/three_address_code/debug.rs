use super::{Tac, ThreeAddressCodeGenerator, Type};

impl ThreeAddressCodeGenerator {
    pub fn _print(&self) {
        let mut indent: usize = 0;
        println!("\n---------------------- \n");
        for tac in &self.tac_table {
            if matches!(tac.tac_type, Type::LoopEnd | Type::ConditionalEnd | Type::Function) {
                indent = indent.saturating_sub(1);
            }

            let pad = "    ".repeat(indent);
            println!("{}", Self::_format_tac(tac, &pad));

            if matches!(tac.tac_type, Type::Loop | Type::Conditional) {
                indent += 1;
            } else if matches!(tac.tac_type, Type::Function) {
                indent = 1;
            }
        }
        println!("\n---------------------- \n");
    }

    fn _format_tac(tac: &Tac, pad: &str) -> String {
        match &tac.tac_type {
            Type::Function => {
                let name = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}function {name}")
            }
            Type::Param => {
                let name = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let ty = tac.value_type.clone().map_or("unknown".to_string(), |t| t.to_str());
                format!("{pad}param {ty} {name}")
            }
            Type::LoopEnd => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}: end while")
            }
            Type::Conditional => match (&tac.operator, tac.arguments.get(1..).unwrap_or(&[])) {
                (Some(op), [left, right]) => {
                    format!("{pad} if ({left} {} {right})", op._as_str())
                }
                (_, rest) => format!("{pad} if ({})", rest.join(", ")),
            },
            Type::Loop => match (&tac.operator, tac.arguments.get(1..).unwrap_or(&[])) {
                (Some(op), [left, right]) => {
                    format!("{pad} while ({left} {} {right})", op._as_str())
                }
                (_, rest) => format!("{pad} while ({})", rest.join(", ")),
            },
            Type::ConditionalEnd => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}: end if")
            }
            Type::Call => {
                let target = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let args = tac.arguments.get(1..).unwrap_or(&[]).join(", ");
                format!("{pad}call {target}({args})")
            }
            Type::Variable | Type::Reasingment | Type::Return => {
                let result = tac.result.as_deref().unwrap_or("?");
                match (&tac.operator, tac.arguments.as_slice()) {
                    (Some(op), [left, right]) => {
                        format!("{pad}{result} = {left} {} {right}", op._as_str())
                    }
                    (None, [value]) => format!("{pad}{result} = {value}"),
                    _ => format!("{pad}{result} = {:?}", tac.arguments),
                }
            }
            Type::AddressOf => {
                let result = tac.result.as_deref().unwrap_or("?");
                let src = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{result} = &{src}")
            }
            Type::Deref => {
                let result = tac.result.as_deref().unwrap_or("?");
                let src = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{result} = *{src}")
            }
            Type::DerefAssign => {
                let ptr = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let val = tac.arguments.get(1).map(String::as_str).unwrap_or("?");
                format!("{pad}*{ptr} = {val}")
            }
            Type::ArrayDecl => {
                let result = tac.result.as_deref().unwrap_or("?");
                let size = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}array {result}[{size}]")
            }
            Type::ArrayIndex => {
                let result = tac.result.as_deref().unwrap_or("?");
                let arr = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let idx = tac.arguments.get(1).map(String::as_str).unwrap_or("?");
                format!("{pad}{result} = {arr}[{idx}]")
            }
            Type::ArrayAssign => {
                let arr = tac.result.as_deref().unwrap_or("?");
                let idx = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                let val = tac.arguments.get(1).map(String::as_str).unwrap_or("?");
                format!("{pad}{arr}[{idx}] = {val}")
            }
            Type::Label => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}{label}:")
            }
            Type::GetReturn => {
                format!("{pad}get return value")
            }
            Type::InlineAssembly => {
                format!("{pad}asm {:?}", tac.arguments)
            }
            Type::Jump => {
                let label = tac.arguments.get(0).map(String::as_str).unwrap_or("?");
                format!("{pad}jump {label}")
            }
        }
    }
}
