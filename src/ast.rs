use anyhow::Result;
use pest::Parser;

#[derive(Parser)]
#[grammar = "code.pest"]
pub struct LanguageParser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equals,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Negative,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    OperatorExpression {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryExpression {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    Ident(String),
    Str(String),
    Int(i64),
    Boolean(bool),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Pattern {
    Global,
    Static,
    Var,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Goto(Box<Expression>),
    Declare {
        pattern: Box<Pattern>,
        name: String,
        expression: Box<Expression>,
    },
    Assignment {
        name: String,
        expression: Box<Expression>,
    },
    If {
        conditional: Box<Expression>,
        statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    },
    Function {
        name: String,
        parameters: Vec<String>,
        statements: Vec<Statement>,
    },
    Expression(Box<Expression>),
    Return(Box<Expression>),
}

pub fn parse(source: &str) -> Result<Vec<Box<Statement>>> {
    let mut ast = vec![];

    let pairs = LanguageParser::parse(Rule::language, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::language => {
                for statement in pair.into_inner() {
                    ast.push(Box::new(build_statement_from_pair(
                        statement.into_inner().next().unwrap(),
                    )));
                }
            }
            _ => {}
        }
    }
    Ok(ast)
}

fn build_statement_from_pair(pair: pest::iterators::Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::statement => build_statement_from_pair(pair.into_inner().next().unwrap()),
        Rule::goto_statement => {
            let mut pair = pair.into_inner();
            let label_pair = pair.next().unwrap();
            let label = get_expression_from_pair(label_pair);
            Statement::Goto(Box::new(label))
        }
        Rule::function_definition => {
            let mut pair = pair.into_inner();
            let name_pair = pair.next().unwrap();
            let name = get_ident_from_pair(name_pair);
            let parameters_pair = pair.next().unwrap();
            let parameters: Vec<String> = parameters_pair
                .into_inner()
                .map(get_ident_from_pair)
                .collect();
            let statements_pair = pair.next().unwrap();
            let statements: Vec<Statement> = statements_pair
                .into_inner()
                .map(build_statement_from_pair)
                .collect();
            Statement::Function {
                name,
                parameters,
                statements,
            }
        }
        Rule::if_statement => {
            let mut pair = pair.into_inner();
            let conditional_pair = pair.next().unwrap();
            let conditional = Box::new(get_expression_from_pair(conditional_pair));
            let statements_pair = pair.next().unwrap();
            let statements: Vec<Statement> = statements_pair
                .into_inner()
                .map(build_statement_from_pair)
                .collect();
            let else_statements = if let Some(else_pair) = pair.next() {
                match else_pair.as_rule() {
                    Rule::else_statement => Some(
                        else_pair
                            .into_inner()
                            .map(build_statement_from_pair)
                            .collect(),
                    ),
                    _ => None,
                }
            } else {
                None
            };
            Statement::If {
                conditional,
                statements,
                else_statements,
            }
        }
        Rule::return_statement => {
            let mut pair = pair.into_inner();
            let expression_pair = pair.next().unwrap();
            let expression = get_expression_from_pair(expression_pair);
            Statement::Return(Box::new(expression))
        }
        Rule::assignment => {
            let mut pair = pair.into_inner();
            let name_pair = pair.next().unwrap();
            let name = get_ident_from_pair(name_pair);
            let expression_pair = pair.next().unwrap();
            let expression = Box::new(get_expression_from_pair(expression_pair));
            Statement::Assignment { name, expression }
        }
        Rule::declare => {
            let mut pair = pair.into_inner();
            let pattern_pair = pair.next().unwrap();
            let pattern = match pattern_pair.into_inner().next().unwrap().as_rule() {
                Rule::global_var => Pattern::Global,
                Rule::static_var => Pattern::Static,
                Rule::var => Pattern::Var,
                unknown_pattern => panic!("Unkown pattern: {:?}", unknown_pattern),
            };
            let name_pair = pair.next().unwrap();
            let name = get_ident_from_pair(name_pair);
            let expression_pair = pair.next().unwrap();
            let expression = Box::new(get_expression_from_pair(expression_pair));
            Statement::Declare {
                pattern: Box::new(pattern),
                name,
                expression,
            }
        }
        Rule::expression => Statement::Expression(Box::new(get_expression_from_pair(pair))),
        unknown_statement => panic!("Unexpected statement: {:?}", unknown_statement),
    }
}

fn get_expression_from_pair(pair: pest::iterators::Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::expression => get_expression_from_pair(pair.into_inner().next().unwrap()),
        Rule::operator_expression => {
            let mut pair = pair.into_inner();
            let left_pair = pair.next().unwrap();
            let left = get_expression_from_pair(left_pair);
            let operator_pair = pair.next().unwrap();
            let right_pair = pair.next().unwrap();
            let right = get_expression_from_pair(right_pair);
            parse_operator_expression(operator_pair, left, right)
        }
        Rule::unary_expression => {
            let mut pair = pair.into_inner();
            let operator_pair = pair.next().unwrap();
            let expression_pair = pair.next().unwrap();
            let expression = get_expression_from_pair(expression_pair);
            parse_unary_operator_expression(operator_pair, expression)
        }
        Rule::function_call => {
            let mut pair = pair.into_inner();
            let name_pair = pair.next().unwrap();
            let name = get_ident_from_pair(name_pair);
            let arguments_pair = pair.next().unwrap();
            let arguments: Vec<Expression> = arguments_pair
                .into_inner()
                .map(get_expression_from_pair)
                .collect();
            Expression::FunctionCall { name, arguments }
        }
        Rule::term => get_expression_from_pair(pair.into_inner().next().unwrap()),
        Rule::string => {
            let str = &pair.as_str();
            let str = &str[1..str.len() - 1];
            let str = str.replace("''", "'");
            Expression::Str(String::from(str))
        }
        Rule::ident => {
            let str = pair.as_str();
            Expression::Ident(String::from(str))
        }
        Rule::int => {
            let str = pair.as_str();
            Expression::Int(str.parse::<i64>().unwrap())
        }
        Rule::boolean => {
            let str = pair.as_str();
            match str {
                "true" => Expression::Boolean(true),
                _ => Expression::Boolean(false),
            }
        }
        unknown_expression => panic!("Unexpected statement: {:?}", unknown_expression),
    }
}

fn parse_operator_expression(
    operator_pair: pest::iterators::Pair<Rule>,
    left: Expression,
    right: Expression,
) -> Expression {
    Expression::OperatorExpression {
        left: Box::new(left),
        right: Box::new(right),
        operator: match operator_pair.into_inner().next().unwrap().as_rule() {
            Rule::add => Operator::Add,
            Rule::subtract => Operator::Subtract,
            Rule::multiply => Operator::Multiply,
            Rule::divide => Operator::Divide,
            Rule::remainder => Operator::Remainder,
            Rule::and => Operator::And,
            Rule::or => Operator::Or,
            Rule::equals => Operator::Equals,
            unknown_operator => panic!("Unkown operator: {:?}", unknown_operator),
        },
    }
}

fn parse_unary_operator_expression(
    operator_pair: pest::iterators::Pair<Rule>,
    expression: Expression,
) -> Expression {
    Expression::UnaryExpression {
        expression: Box::new(expression),
        operator: match operator_pair.into_inner().next().unwrap().as_rule() {
            Rule::not => UnaryOperator::Not,
            Rule::negative => UnaryOperator::Negative,
            unknown_operator => panic!("Unkown unary operator: {:?}", unknown_operator),
        },
    }
}

fn get_ident_from_pair(pair: pest::iterators::Pair<Rule>) -> String {
    let str = pair.as_str();
    String::from(str)
}
