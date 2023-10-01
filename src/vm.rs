use std::collections::HashMap;

use crate::{
    bytecode::ByteCode,
    parser::{parser::Prototype, token::GlobalVar},
    value::Value,
};

#[derive(Debug)]
pub struct VM {
    pub globals: HashMap<GlobalVar, Value>,
    pub call_stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            globals: Self::gen_global_vars(),
            call_stack: vec![],
        }
    }

    pub fn run(&mut self, prototype: Prototype) {
        for code in prototype.bytecode {
            match code {
                ByteCode::GetGlobal(_, _) => {}
                ByteCode::LoadK(_, _) => {}
                ByteCode::Call(_, _) => {}
            }
        }
    }

    fn gen_global_vars() -> HashMap<GlobalVar, Value> {
        let globals = HashMap::from([(GlobalVar::Print, Value::Function(lib_print))]);

        globals
    }
}

fn lib_print(ctx: &mut VM) -> i32 {
    println!("{:?}", ctx.call_stack[1]);
    0
}
