extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

use pest::{iterators::Pairs, Parser};

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
    In,
    NotIn
}
impl Comparison {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "==" | "=" => Comparison::Eq,
            ">" => Comparison::More,
            ">=" => Comparison::MoreEq,
            "<" => Comparison::Less,
            "<=" => Comparison::LessEq,
            "!=" => Comparison::NotEq,
            "in" => Comparison::In,
            "not in" => Comparison::NotIn,
            _ => unreachable!()
        }
    }
}

enum Logic {
    And,
    Or
}

impl Logic {
    fn from_str(logic: &str) -> Self {
        match logic.to_lowercase().as_str() {
            "and" => Logic::And,
            "or" => Logic::Or,
            _ => unreachable!()
        }
    }
}

fn logic_check(logic_or: &Logic, output: bool, val: bool) -> bool {
    match logic_or {
        Logic::Or => output || val,
        Logic::And => output && val,
    }
}

fn eval(expr: Pairs<Rule>, map: &HashMap<&str, &str>) -> bool {
    let mut output = false;
    let mut logic_or = Logic::Or;
    let mut negate = false;

    for pair in expr {
        match pair.as_rule() {
            Rule::pair => {
                let mut inner_rules = pair.into_inner();
                // value_expr|array_expr
                let var = inner_rules.next().unwrap().as_str();
                let expression = inner_rules.next().unwrap();

                output = match expression.as_rule() {
                    Rule::value_expr => {
                        let mut inner2_rules = expression.into_inner();
                        let op = Comparison::from_str(inner2_rules.next().unwrap().as_str());
                        let val = inner2_rules.next().unwrap().as_str();
        
                        if map.contains_key(var) {
                            let v = *map.get(var).unwrap();
                            match op {
                                Comparison::Eq => logic_check(&logic_or, output, val == v),
                                Comparison::NotEq => logic_check(&logic_or, output, val != v),
                                Comparison::More => unimplemented!(),
                                Comparison::MoreEq =>unimplemented!(),
                                Comparison::Less => unimplemented!(),
                                Comparison::LessEq => unimplemented!(),
                                Comparison::In => unimplemented!(),
                                Comparison::NotIn => unimplemented!(),
                            }
                        } else {
                            logic_check(&logic_or, output, false)
                        }
                    },
                    Rule::array_expr => {
                        let mut inner2_rules = expression.into_inner();
                        // in | not in
                        let op = Comparison::from_str(inner2_rules.next().unwrap().as_str());
                        let mut inner3_rules = inner2_rules.next().unwrap();
                        if inner3_rules.as_rule() == Rule::array {
                            let mut values:Vec<&str> = Vec::new();
                            for p in inner3_rules.into_inner() {
                                values.push(p.as_str());
                            }
                            if map.contains_key(var) {
                                let v = *map.get(var).unwrap();
                                println!("Checking if {} contains {} - answer: {:#?}", var, v, values.contains(&v));
                                logic_check(&logic_or, output, values.contains(&v))
                            } else {
                                logic_check(&logic_or, output, false)
                            }
                        } else {
                            unreachable!();
                        }
        
                    },
                    _ => unreachable!()
                }
            }
            Rule::logic_op => {
                logic_or = Logic::from_str(pair.as_str());
            },
            Rule::opNegate => {
                negate = true;
            },
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

    #[test]
    fn test_not_equal_pair() {
        let map = HashMap::from([("a", "b"), ("c", "d")]);
        assert_eq!(eval(BoolExprParser::parse(Rule::main, "a!=c").expect("Parse error"), &map), true);
        assert_eq!(eval(BoolExprParser::parse(Rule::main, "a==b").expect("Parse error"), &map), true);
    }

    #[test]
    fn test_hash_map_does_not_contain() {
        let map = HashMap::from([("a", "b"), ("c", "d")]);
        assert_eq!(eval(BoolExprParser::parse(Rule::main, "a=b AND xxx=ddd").expect("Parse error"), &map), false);
        assert_eq!(eval(BoolExprParser::parse(Rule::main, "a=b or xxx=ddd").expect("Parse error"), &map), true);
    }

    #[test]
    fn test_simple_array() {
        let map = HashMap::from([("a", "d"), ("b", "c")]);
        let ast = BoolExprParser::parse(Rule::main, "b=c or a in (a,b,c,d)").expect("Parse error");
        assert_eq!(eval(ast, &map), true);
    }
}