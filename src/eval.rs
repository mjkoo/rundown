use std::collections::HashMap;
use std::fmt;

use anyhow::{anyhow, bail, Result};

use crate::ast::{Expression, Operator, ScopeSpecifier, Statement};

pub type Scope = HashMap<String, Value>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Str(String),
    Int(i64),
    Bool(bool),
}

impl Value {
    fn as_bool(&self) -> bool {
        match self {
            Value::Str(s) => !s.is_empty(),
            Value::Int(i) => *i != 0,
            Value::Bool(b) => *b,
        }
    }

    fn add(&self, other: Value) -> Result<Value> {
        match self {
            Value::Str(s) => Ok(Value::Str(format!("{}{}", s, other.to_string()))),
            Value::Int(i) => {
                if let Value::Int(other) = other {
                    Ok(Value::Int(i + other))
                } else {
                    Err(anyhow!("Type error"))
                }
            }
            _ => Err(anyhow!("Type error")),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Default)]
pub struct FunctionContext {
    parameters: Vec<String>,
    statements: Vec<Statement>,
    static_variables: Scope,
}

#[derive(Debug, Default)]
pub struct Context {
    global_variables: Scope,
    function_contexts: HashMap<String, FunctionContext>,
}

pub enum StatementResult {
    Continue,
    Goto(String),
    Return(Value),
}

pub enum ExpressionResult {
    Value(Value),
    Goto(String),
}

impl Context {
    pub fn eval(&mut self, statements: &[Statement]) -> Result<StatementResult> {
        let mut local_variables: Scope = Default::default();
        self.eval_statements(statements, &mut local_variables, &mut None)
    }

    fn eval_statements(
        &mut self,
        statements: &[Statement],
        local_variables: &mut Scope,
        function_context: &mut Option<FunctionContext>,
    ) -> Result<StatementResult> {
        for statement in statements {
            match self.eval_statement(statement, local_variables, function_context) {
                Ok(StatementResult::Continue) => (),
                exit => {
                    return exit;
                }
            }
        }

        Ok(StatementResult::Continue)
    }

    fn eval_statement(
        &mut self,
        statement: &Statement,
        local_variables: &mut Scope,
        function_context: &mut Option<FunctionContext>,
    ) -> Result<StatementResult> {
        match statement {
            Statement::Goto(expression) => {
                match self.eval_expression(&*expression, local_variables, function_context)? {
                    ExpressionResult::Value(Value::Str(s)) => return Ok(StatementResult::Goto(s)),
                    ExpressionResult::Goto(s) => {
                        return Ok(StatementResult::Goto(s));
                    }
                    _ => {
                        bail!("Attempted to jump to a non-string label");
                    }
                }
            }
            Statement::Declare {
                scope: ScopeSpecifier::Global,
                name,
                expression,
            } => {
                if self.global_variables.contains_key(name) {
                    return Ok(StatementResult::Continue);
                }

                match self.eval_expression(expression, local_variables, function_context)? {
                    ExpressionResult::Value(v) => {
                        self.global_variables.insert(name.clone(), v);
                    }
                    ExpressionResult::Goto(s) => {
                        return Ok(StatementResult::Goto(s));
                    }
                }
            }
            Statement::Declare {
                scope: ScopeSpecifier::Static,
                name,
                expression,
            } => {
                if function_context.is_none() {
                    bail!("Attempted to define a static variable outside of a function context");
                }

                if function_context
                    .as_ref()
                    .map_or(false, |ctx| ctx.static_variables.contains_key(name))
                {
                    return Ok(StatementResult::Continue);
                }

                match self.eval_expression(expression, local_variables, function_context)? {
                    ExpressionResult::Value(v) => {
                        function_context
                            .as_mut()
                            .and_then(|ctx| ctx.static_variables.insert(name.clone(), v));
                    }
                    ExpressionResult::Goto(s) => {
                        return Ok(StatementResult::Goto(s));
                    }
                }
            }
            Statement::Declare {
                scope: ScopeSpecifier::Local,
                name,
                expression,
            } => {
                if local_variables.contains_key(name) {
                    bail!("Attemped to redefine local variable");
                }

                match self.eval_expression(expression, local_variables, function_context)? {
                    ExpressionResult::Value(v) => {
                        local_variables.insert(name.clone(), v);
                    }
                    ExpressionResult::Goto(s) => {
                        return Ok(StatementResult::Goto(s));
                    }
                }
            }
            Statement::Assignment { name, expression } => {
                // TODO: If this expression contains a function call which has side effects,
                // but the assignment fails, this will still perform the side-effects.
                let value =
                    match self.eval_expression(expression, local_variables, function_context)? {
                        ExpressionResult::Value(v) => v,
                        ExpressionResult::Goto(s) => {
                            return Ok(StatementResult::Goto(s));
                        }
                    };
                if let Some(v) = local_variables.get_mut(name) {
                    *v = value;
                } else if let Some(v) = function_context
                    .as_mut()
                    .and_then(|ctx| ctx.static_variables.get_mut(name))
                {
                    *v = value;
                } else if let Some(v) = self.global_variables.get_mut(name) {
                    *v = value;
                } else {
                    bail!("Attempted to assign to an undeclared variable");
                }
            }
            Statement::If {
                conditional,
                statements,
                else_statements,
            } => {
                let conditional =
                    match self.eval_expression(conditional, local_variables, function_context)? {
                        ExpressionResult::Value(v) => v.as_bool(),
                        ExpressionResult::Goto(s) => {
                            return Ok(StatementResult::Goto(s));
                        }
                    };

                if conditional {
                    return self.eval_statements(statements, local_variables, function_context);
                } else if let Some(else_statements) = else_statements {
                    return self.eval_statements(
                        else_statements,
                        local_variables,
                        function_context,
                    );
                }
            }
            Statement::FunctionDefinition {
                name,
                parameters,
                statements,
            } => {
                if function_context.is_some() {
                    bail!("Attempted to define a function within a function context");
                }

                self.function_contexts.insert(
                    name.clone(),
                    FunctionContext {
                        parameters: parameters.clone(),
                        statements: statements.clone(),
                        ..Default::default()
                    },
                );
            }
            Statement::Expression(expression) => {
                if let ExpressionResult::Goto(s) =
                    self.eval_expression(expression, local_variables, function_context)?
                {
                    return Ok(StatementResult::Goto(s));
                }
            }
            Statement::Return(expression) => {
                match self.eval_expression(expression, local_variables, function_context)? {
                    ExpressionResult::Value(v) => return Ok(StatementResult::Return(v)),
                    ExpressionResult::Goto(s) => {
                        return Ok(StatementResult::Goto(s));
                    }
                }
            }
            _ => (),
        }

        Ok(StatementResult::Continue)
    }

    fn eval_expression(
        &mut self,
        expression: &Expression,
        local_variables: &mut Scope,
        function_context: &mut Option<FunctionContext>,
    ) -> Result<ExpressionResult> {
        match expression {
            Expression::Str(s) => Ok(ExpressionResult::Value(Value::Str(s.clone()))),
            Expression::Int(i) => Ok(ExpressionResult::Value(Value::Int(*i))),
            Expression::Bool(b) => Ok(ExpressionResult::Value(Value::Bool(*b))),
            Expression::Ident(name) => {
                if let Some(v) = local_variables.get(name) {
                    Ok(ExpressionResult::Value(v.clone()))
                } else if let Some(v) = function_context
                    .as_ref()
                    .and_then(|ctx| ctx.static_variables.get(name))
                {
                    Ok(ExpressionResult::Value(v.clone()))
                } else if let Some(v) = self.global_variables.get(name) {
                    Ok(ExpressionResult::Value(v.clone()))
                } else {
                    Err(anyhow!("Attempted to access an undeclared variable"))
                }
            }
            Expression::OperatorExpression {
                operator,
                left,
                right,
            } => {
                let lhs = match self.eval_expression(left, local_variables, function_context)? {
                    ExpressionResult::Value(v) => v,
                    goto => {
                        return Ok(goto);
                    }
                };
                let rhs = match self.eval_expression(right, local_variables, function_context)? {
                    ExpressionResult::Value(v) => v,
                    goto => {
                        return Ok(goto);
                    }
                };

                match operator {
                    Operator::Add => Ok(ExpressionResult::Value(lhs.add(rhs)?)),
                    _ => unimplemented!(),
                }
            }
            _ => Err(anyhow!("foo")),
        }
    }
}
