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
pub fn eval(expr: Pairs<Rule>, map: &HashMap<&str, &str>) -> bool {
    let mut output = false;
    let mut logic_or = LogicExpr::Or;
    let mut negate = false;

    for pair in expr {
        match pair.as_rule() {
            Rule::pair => {
                let mut inner_rules = pair.into_inner();
                let var = inner_rules.next().unwrap().as_str();
                let expression = inner_rules.next().unwrap();

                output = match expression.as_rule() {
                    Rule::value_expr => {
                        let mut inner2_rules = expression.into_inner();
                        let op = ComparisonExpr::from_str(inner2_rules.next().unwrap().as_str());
                        let pair_rule = inner2_rules.next().unwrap().into_inner().next().unwrap();
                        let rule = pair_rule.as_rule();
                        let val = pair_rule.as_str();

                        if map.contains_key(var) {
                            let v = *map.get(var).unwrap();
                            match op {
                                ComparisonExpr::Eq => logic_op(&logic_or, output, val == v),
                                ComparisonExpr::NotEq => logic_op(&logic_or, output, val != v),
                                ComparisonExpr::More => comparison_helper(
                                    &logic_or,
                                    output,
                                    v,
                                    val,
                                    rule,
                                    ComparisonExpr::More,
                                ),
                                ComparisonExpr::MoreEq => comparison_helper(
                                    &logic_or,
                                    output,
                                    v,
                                    val,
                                    rule,
                                    ComparisonExpr::MoreEq,
                                ),
                                ComparisonExpr::Less => comparison_helper(
                                    &logic_or,
                                    output,
                                    v,
                                    val,
                                    rule,
                                    ComparisonExpr::Less,
                                ),
                                ComparisonExpr::LessEq => comparison_helper(
                                    &logic_or,
                                    output,
                                    v,
                                    val,
                                    rule,
                                    ComparisonExpr::LessEq,
                                ),
                            }
                        } else {
                            logic_op(&logic_or, output, false)
                        }
                    }
                    Rule::array_expr => {
                        let mut inner2_rules = expression.into_inner();
                        let op = ArrayExpr::from_str(inner2_rules.next().unwrap().as_str());
                        let inner3_rules = inner2_rules.next().unwrap();
                        if inner3_rules.as_rule() == Rule::array {
                            let mut values: Vec<&str> = Vec::new();
                            for p in inner3_rules.into_inner() {
                                let inner3_value = p.as_str();
                                values.push(inner3_value);
                            }
                            if map.contains_key(var) {
                                let v = *map.get(var).unwrap();
                                match op {
                                    ArrayExpr::In => {
                                        logic_op(&logic_or, output, values.contains(&v))
                                    }
                                    ArrayExpr::NotIn => {
                                        logic_op(&logic_or, output, !values.contains(&v))
                                    }
                                }
                            } else {
                                match op {
                                    ArrayExpr::In => logic_op(&logic_or, output, false),
                                    ArrayExpr::NotIn => logic_op(&logic_or, output, true),
                                }
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Rule::logic_op => {
                logic_or = LogicExpr::from_str(pair.as_str());
            }
            Rule::opNegate => {
                negate = true;
            }
            Rule::scope => {
                let out_of_scope = eval(pair.into_inner(), &map);
                output = logic_op(&logic_or, output, out_of_scope)
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    if negate {
        !output
    } else {
        output
    }
}

#[cfg(test)]
mod tests;
