use std::collections::HashMap;

use crate::{
    bytecode::ByteCode,
    parser::{parser::Prototype, token::Vars},
    value::Value,
};

#[derive(Debug)]
pub struct VM {
    pub globals: HashMap<Vars, Value>,
    pub _locals: HashMap<Vars, Value>,
    pub call_stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            globals: Self::gen_global_vars(),
            _locals: HashMap::new(),
            call_stack: vec![],
        }
    }

    pub fn run(&mut self, prototype: Prototype) {
        for code in prototype.bytecode {
            match code {
                ByteCode::GetGlobal(stack_index, const_index) => self.set_stack(
                    stack_index as usize,
                    self.globals
                        .get(&prototype.constants[const_index as usize])
                        .unwrap_or(&Value::Nil)
                        .clone(),
                ),
                ByteCode::LoadK(stack_index, const_index) => {
                    match prototype.constants[const_index as usize] {
                        Vars::General(ref val) => {
                            self.set_stack(stack_index as usize, Value::String(val.clone()));
                        }
                        _ => {}
                    }
                }
                ByteCode::Call(stack_index, param_index) => {
                    if let Value::Function(func) = &self.call_stack[stack_index as usize] {
                        func(self);
                    } else {
                        panic!("not implement!");
                    }
                }
            }
        }
    }

    fn set_stack(&mut self, idx: usize, value: Value) {
        if self.call_stack.len() <= idx {
            self.call_stack.reserve(idx);
        }

        self.call_stack.push(value);
        // self.call_stack[idx] = value;
    }

    fn gen_global_vars() -> HashMap<Vars, Value> {
        let globals = HashMap::from([(Vars::Print, Value::Function(lib_print))]);

        globals
    }
}

fn lib_print(ctx: &mut VM) -> i32 {
    println!("{:?}", ctx.call_stack[1]);
    0
}
