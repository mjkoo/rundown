use std::{ascii::AsciiExt, collections::HashMap};

use anyhow::{bail, Result};

use crate::ast::{Expression, Scope, Statement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Str(String),
    Int(i64),
    Boolean(bool),
}

#[derive(Debug, Default)]
pub struct FunctionContext {
    static_variables: HashMap<String, Value>,
}

#[derive(Debug, Default)]
pub struct Context {
    global_variables: HashMap<String, Value>,
    function_contexts: HashMap<String, FunctionContext>,
}

pub enum StatementResult {
    Continue,
    Goto(String),
    Return(Value),
}

impl Context {
    pub fn eval(&mut self, statements: &[Statement]) -> Result<Option<String>> {
        let mut local_variables = HashMap::new();
        for statement in statements {}
        Ok(Some("foo".to_owned()))
    }

    fn eval_statement(
        &mut self,
        statement: &Statement,
        local_variables: &mut HashMap<String, Value>,
        function_context: &mut Option<FunctionContext>,
    ) -> Result<StatementResult> {
        match statement {
            Statement::Goto(expression) => {
                if let Value::Str(label) = self.eval_expression(&*expression) {
                    return Ok(StatementResult::Goto(label));
                } else {
                    bail!("Attempted to jump to a non-string label");
                }
            }
            Statement::Declare {
                scope: Scope::Global,
                name,
                expression,
            } => {
                if self.global_variables.contains_key(name) {
                    bail!("Attemped to redefine global variable");
                }

                let value = self.eval_expression(expression);
                self.global_variables.insert(name.clone(), value);
            }
            Statement::Declare {
                scope: Scope::Static,
                name,
                expression,
            } => {
                if let Some(function_context) = function_context {
                    if function_context.static_variables.contains_key(name) {
                        bail!("Attemped to redefine static variable");
                    }

                    let value = self.eval_expression(expression);
                    function_context
                        .static_variables
                        .insert(name.clone(), value);
                } else {
                    bail!("Attempted to define a static variable outside of a function context");
                }
            }
            Statement::Declare {
                scope: Scope::Local,
                name,
                expression,
            } => {
                if local_variables.contains_key(name) {
                    bail!("Attemped to redefine local variable");
                }

                let value = self.eval_expression(expression);
                local_variables.insert(name.clone(), value);
            }
            Statement::Assignment { name, expression } => {}
            _ => (),
        }

        Ok(StatementResult::Continue)
    }

    fn eval_expression(&mut self, expression: &Expression) -> Value {
        Value::Str("foo".to_owned())
    }
}
