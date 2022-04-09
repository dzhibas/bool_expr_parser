extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "bool_expr.pest"]
pub struct BoolExprParser;

enum ComparisonExpr {
    Eq,
    More,
    Less,
    MoreEq,
    LessEq,
    NotEq,
}

impl ComparisonExpr {
    fn from_str(expr: &str) -> Self {
        match expr {
            "==" | "=" => ComparisonExpr::Eq,
            ">" => ComparisonExpr::More,
            ">=" => ComparisonExpr::MoreEq,
            "<" => ComparisonExpr::Less,
            "<=" => ComparisonExpr::LessEq,
            "!=" => ComparisonExpr::NotEq,
            _ => unreachable!(),
        }
    }
}
enum ArrayExpr {
    In,
    NotIn,
}

impl ArrayExpr {
    fn from_str(expr: &str) -> Self {
        match expr.to_lowercase().as_str() {
            "in" => ArrayExpr::In,
            "not in" => ArrayExpr::NotIn,
            _ => unreachable!(),
        }
    }
}

enum LogicExpr {
    And,
    Or,
}

impl LogicExpr {
    fn from_str(expr: &str) -> Self {
        match expr.to_lowercase().as_str() {
            "and" => LogicExpr::And,
            "or" => LogicExpr::Or,
            _ => unreachable!(),
        }
    }
}

/// helper function to do a logic operation
fn logic_op(op: &LogicExpr, value_a: bool, value_b: bool) -> bool {
    match op {
        LogicExpr::Or => value_a || value_b,
        LogicExpr::And => value_a && value_b,
    }
}

/// helper function for comparisons >, <, >=, <=
fn comparison_helper(
    op: &LogicExpr,
    bool_left: bool,
    value_a: &str,
    value_b: &str,
    pair_rule: Rule,
    comparsion_expr: ComparisonExpr,
) -> bool {
    let bool_right = match pair_rule {
        Rule::number => {
            // @TODO - handle int parse exceptions gracefully so it return false in case cannot be parsed
            let v_left: i64 = value_a
                .to_string()
                .parse()
                .expect("cannot parse string to int");
            let v_right: i64 = value_b.to_string().parse().unwrap();
            match comparsion_expr {
                ComparisonExpr::More => v_left > v_right,
                ComparisonExpr::MoreEq => v_left >= v_right,
                ComparisonExpr::Less => v_left < v_right,
                ComparisonExpr::LessEq => v_left <= v_right,
                _ => false,
            }
        }
        _ => false,
    };
    logic_op(&op, bool_left, bool_right)
}

/// bool expression evaluation function
/// given parsed expression and incoming variable HashMap
/// bool expression evaluated and returns either true / false
///
/// ```
/// use std::collections::HashMap;
/// use pest::Parser;
/// use bool_expr_parser::{eval, BoolExprParser, Rule};
///
/// let parsed = BoolExprParser::parse(Rule::main, "a=b and (c=d or e=f)").expect("Parse error");
/// let map = HashMap::from([("a", "b"), ("e", "f")]);
/// assert_eq!(eval(parsed, &map), true);
/// ```
pub fn eval(pairs: Pairs<Rule>, variable_map: &HashMap<&str, &str>) -> bool {
    let mut output = false;
    let mut logic_expr = LogicExpr::Or;
    let mut negate = false;

    for pair in pairs {
        match pair.as_rule() {
            Rule::pair => {
                let mut inner_pairs = pair.into_inner();
                let var_name = inner_pairs.next().unwrap().as_str();
                let value_expression = inner_pairs.next().unwrap();

                output = match value_expression.as_rule() {
                    Rule::value_expr => {
                        let mut inner_2_pairs = value_expression.into_inner();
                        let op = ComparisonExpr::from_str(inner_2_pairs.next().unwrap().as_str());
                        let inner_2_pair =
                            inner_2_pairs.next().unwrap().into_inner().next().unwrap();
                        let rule = inner_2_pair.as_rule();
                        let rule_value = inner_2_pair.as_str();

                        if variable_map.contains_key(var_name) {
                            let incoming_value = *variable_map.get(var_name).unwrap();
                            match op {
                                ComparisonExpr::Eq | ComparisonExpr::NotEq => logic_op(
                                    &logic_expr,
                                    output,
                                    match op {
                                        ComparisonExpr::Eq => rule_value == incoming_value,
                                        ComparisonExpr::NotEq => rule_value != incoming_value,
                                        _ => unreachable!(),
                                    },
                                ),
                                ComparisonExpr::More
                                | ComparisonExpr::MoreEq
                                | ComparisonExpr::Less
                                | ComparisonExpr::LessEq => comparison_helper(
                                    &logic_expr,
                                    output,
                                    incoming_value,
                                    rule_value,
                                    rule,
                                    op,
                                ),
                            }
                        } else {
                            logic_op(&logic_expr, output, false)
                        }
                    }
                    Rule::array_expr => {
                        let mut inner_2_pairs = value_expression.into_inner();
                        let array_expr =
                            ArrayExpr::from_str(inner_2_pairs.next().unwrap().as_str());
                        let inner_3_pairs = inner_2_pairs.next().unwrap();
                        if inner_3_pairs.as_rule() == Rule::array {
                            let mut array_values: Vec<&str> = Vec::new();
                            for p in inner_3_pairs.into_inner() {
                                let inner3_value = p.as_str();
                                array_values.push(inner3_value);
                            }
                            if variable_map.contains_key(var_name) {
                                let incomming_value = *variable_map.get(var_name).unwrap();
                                match array_expr {
                                    ArrayExpr::In => logic_op(
                                        &logic_expr,
                                        output,
                                        array_values.contains(&incomming_value),
                                    ),
                                    ArrayExpr::NotIn => logic_op(
                                        &logic_expr,
                                        output,
                                        !array_values.contains(&incomming_value),
                                    ),
                                }
                            } else {
                                match array_expr {
                                    ArrayExpr::In => logic_op(&logic_expr, output, false),
                                    ArrayExpr::NotIn => logic_op(&logic_expr, output, true),
                                }
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Rule::logic_op => logic_expr = LogicExpr::from_str(pair.as_str()),
            Rule::negate_op => negate = true,
            Rule::scope => {
                let scope_output = eval(pair.into_inner(), &variable_map);
                output = logic_op(&logic_expr, output, scope_output)
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    match negate {
        true => !output,
        false => output,
    }
}

#[cfg(test)]
mod tests;
