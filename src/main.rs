extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

use pest::{iterators::Pairs, Parser};

#[derive(Parser)]
#[grammar = "bool_expr.pest"]
pub struct BoolExprParser;

fn eval(expr: Pairs<Rule>, map: &HashMap<&str, &str>) -> bool {
    let mut output = false;
    let mut logic_or = true;
    let mut negate = false;

    for pair in expr {
        match pair.as_rule() {
            Rule::pair => {
                let mut inner_rules = pair.into_inner();
                let var = inner_rules.next().unwrap().as_str();
                let mut inner2_rules = inner_rules.next().unwrap().into_inner();
                let op = inner2_rules.next().unwrap().as_str();
                let val = inner2_rules.next().unwrap().as_str();
                // println!("var {} {} {}", var, op, val);
                if map.contains_key(var) {
                    let v = *map.get(var).unwrap();
                    if val == v {
                        output = match logic_or {
                            true => {
                                logic_or = false;
                                output || true
                            }
                            false => output && true,
                        }
                    } else {
                        output = match logic_or {
                            true => {
                                logic_or = false;
                                output || false
                            }
                            false => output && false,
                        }
                    }
                }
            }
            Rule::logic_op => {
                logic_or = match pair.as_str() {
                    "and" | "AND" => false,
                    "or" | "OR" => true,
                    _ => false,
                }
            },
            Rule::opNegate => {
                negate = true;
            }
            Rule::scope => {
                let out_of_scope = eval(pair.into_inner(), &map);
                output = match logic_or {
                    true => {
                        logic_or = false;
                        output || out_of_scope
                    }
                    false => output && out_of_scope,
                }
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

fn main() {
    let _expression = r#"as22 IN (a,v,'c d',213) 
    or (a!=2 and ds='seo ew') 
    OR demo in ("zom", ds, 2323) 
    and a=z 
    AND !(b=3 or b=ds)"#;
    let expression = "countryCode = NL and !(a=z or b=g)";

    let ast = BoolExprParser::parse(Rule::main, &expression).expect("Failed to parse");
    println!("Tree: {:#?}", ast);

    let map = HashMap::from([("countryCode", "NL"), ("b", "z")]);

    println!("Evaluated answer: {:#?}", eval(ast, &map));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_negate_test() {
        let map = HashMap::from([("countryCode", "NL"), ("b", "z")]);

        let ast = BoolExprParser::parse(Rule::main, "countryCode = NL and !(a=z or b=g)").expect("Failed to parse");
        assert_eq!(eval(ast, &map), true);

        let ast = BoolExprParser::parse(Rule::main, "countryCode = NL and (a=z or b=g)").expect("Failed to parse");
        assert_eq!(eval(ast, &map), false);
    }

    #[test]
    fn test_scopes() {
        let map = HashMap::from([("b", "a"), ("z", "d")]);
        assert_eq!(eval(BoolExprParser::parse(Rule::main, "(a=b or b=a) AND (z=d or b=d)").expect("Parse error"), &map), true);
    }

    #[test]
    fn test_simple_pair_test() {
        let map = HashMap::from([("countryCode", "DE"), ("b", "z")]);
        assert_eq!(eval(BoolExprParser::parse(Rule::main, "countryCode=DE").expect("Parse error"), &map), true);
    }

    #[test]
    fn test_variable_with_underscore() {
        let map = HashMap::from([("country_code", "IL"), ("b", "z")]);
        assert_eq!(eval(BoolExprParser::parse(Rule::main, "country_code =   IL").expect("Parse error"), &map), true);
    }

    #[test]
    fn test_logic_and() {
        let map = HashMap::from([("a", "b"), ("c", "d")]);
        assert_eq!(eval(BoolExprParser::parse(Rule::main, "a=b and c=d").expect("Parse error"), &map), true);
    }

    #[test]
    fn test_logic_or() {
        let map = HashMap::from([("a", "XXX"), ("c", "d")]);
        assert_eq!(eval(BoolExprParser::parse(Rule::main, "a=b or c=d").expect("Parse error"), &map), true);
    }
}