extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

use pest::{iterators::Pairs};

#[derive(Parser)]
#[grammar = "bool_expr.pest"]
pub struct BoolExprParser;

enum Comparison {
    Eq,
    More,
    Less,
    MoreEq,
    LessEq,
    NotEq,
}

impl Comparison {
    fn from_str(s: &str) -> Self {
        match s {
            "==" | "=" => Comparison::Eq,
            ">" => Comparison::More,
            ">=" => Comparison::MoreEq,
            "<" => Comparison::Less,
            "<=" => Comparison::LessEq,
            "!=" => Comparison::NotEq,
            _ => unreachable!(),
        }
    }
}
enum ArrayComparison {
    In,
    NotIn,
}

impl ArrayComparison {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "in" => ArrayComparison::In,
            "not in" => ArrayComparison::NotIn,
            _ => unreachable!(),
        }
    }
}

enum Logic {
    And,
    Or,
}

impl Logic {
    fn from_str(logic: &str) -> Self {
        match logic.to_lowercase().as_str() {
            "and" => Logic::And,
            "or" => Logic::Or,
            _ => unreachable!(),
        }
    }
}

fn logic_check(logic_or: &Logic, output: bool, val: bool) -> bool {
    match logic_or {
        Logic::Or => output || val,
        Logic::And => output && val,
    }
}

fn comparison_helper(
    logic_or: &Logic,
    output: bool,
    v: &str,
    val: &str,
    rule: Rule,
    comp: Comparison,
) -> bool {
    let out = match rule {
        Rule::number => {
            let v1: i64 = v.to_string().parse().expect("cannot parse string to int");
            let v2: i64 = val.to_string().parse().unwrap();
            match comp {
                Comparison::More => v1 > v2,
                Comparison::MoreEq => v1 >= v2,
                Comparison::Less => v1 < v2,
                Comparison::LessEq => v1 <= v2,
                _ => false,
            }
        }
        _ => false,
    };
    logic_check(&logic_or, output, out)
}

pub fn eval(expr: Pairs<Rule>, map: &HashMap<&str, &str>) -> bool {
    let mut output = false;
    let mut logic_or = Logic::Or;
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
                        let op = Comparison::from_str(inner2_rules.next().unwrap().as_str());
                        let pair_rule = inner2_rules.next().unwrap().into_inner().next().unwrap();
                        let rule = pair_rule.as_rule();
                        let val = pair_rule.as_str();

                        if map.contains_key(var) {
                            let v = *map.get(var).unwrap();
                            match op {
                                Comparison::Eq => logic_check(&logic_or, output, val == v),
                                Comparison::NotEq => logic_check(&logic_or, output, val != v),
                                Comparison::More => comparison_helper(
                                    &logic_or,
                                    output,
                                    v,
                                    val,
                                    rule,
                                    Comparison::More,
                                ),
                                Comparison::MoreEq => comparison_helper(
                                    &logic_or,
                                    output,
                                    v,
                                    val,
                                    rule,
                                    Comparison::MoreEq,
                                ),
                                Comparison::Less => comparison_helper(
                                    &logic_or,
                                    output,
                                    v,
                                    val,
                                    rule,
                                    Comparison::Less,
                                ),
                                Comparison::LessEq => comparison_helper(
                                    &logic_or,
                                    output,
                                    v,
                                    val,
                                    rule,
                                    Comparison::LessEq,
                                ),
                            }
                        } else {
                            logic_check(&logic_or, output, false)
                        }
                    }
                    Rule::array_expr => {
                        let mut inner2_rules = expression.into_inner();
                        let op = ArrayComparison::from_str(inner2_rules.next().unwrap().as_str());
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
                                    ArrayComparison::In => {
                                        logic_check(&logic_or, output, values.contains(&v))
                                    }
                                    ArrayComparison::NotIn => {
                                        logic_check(&logic_or, output, !values.contains(&v))
                                    }
                                }
                            } else {
                                match op {
                                    ArrayComparison::In => logic_check(&logic_or, output, false),
                                    ArrayComparison::NotIn => logic_check(&logic_or, output, true),
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
                logic_or = Logic::from_str(pair.as_str());
            }
            Rule::opNegate => {
                negate = true;
            }
            Rule::scope => {
                let out_of_scope = eval(pair.into_inner(), &map);
                output = logic_check(&logic_or, output, out_of_scope)
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
