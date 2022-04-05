use super::*;

#[test]
fn test_negate_test() {
    let map = HashMap::from([("countryCode", "NL"), ("b", "z")]);

    let ast = BoolExprParser::parse(Rule::main, "countryCode = NL and !(a=z or b=g)")
        .expect("Failed to parse");
    assert_eq!(eval(ast, &map), true);

    let ast = BoolExprParser::parse(Rule::main, "countryCode = NL and (a=z or b=g)")
        .expect("Failed to parse");
    assert_eq!(eval(ast, &map), false);
}

#[test]
fn test_scopes() {
    let map = HashMap::from([("b", "a"), ("z", "d")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "(a=b or b=a) AND (z=d or b=d)")
                .expect("Parse error"),
            &map
        ),
        true
    );
}

#[test]
fn test_simple_pair_test() {
    let map = HashMap::from([("countryCode", "DE"), ("b", "z")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "countryCode=DE").expect("Parse error"),
            &map
        ),
        true
    );
}

#[test]
fn test_variable_with_underscore() {
    let map = HashMap::from([("country_code", "IL"), ("b", "z")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "country_code =   IL").expect("Parse error"),
            &map
        ),
        true
    );
}

#[test]
fn test_logic_and() {
    let map = HashMap::from([("a", "b"), ("c", "d")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a=b and c=d").expect("Parse error"),
            &map
        ),
        true
    );
}

#[test]
fn test_logic_or() {
    let map = HashMap::from([("a", "XXX"), ("c", "d")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a=b or c=d").expect("Parse error"),
            &map
        ),
        true
    );
}

#[test]
fn test_not_equal_pair() {
    let map = HashMap::from([("a", "b"), ("c", "d")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a!=c").expect("Parse error"),
            &map
        ),
        true
    );
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a==b").expect("Parse error"),
            &map
        ),
        true
    );
}

#[test]
fn test_hash_map_does_not_contain() {
    let map = HashMap::from([("a", "b"), ("c", "d")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a=b AND xxx=ddd").expect("Parse error"),
            &map
        ),
        false
    );
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a=b or xxx=ddd").expect("Parse error"),
            &map
        ),
        true
    );
}

#[test]
fn test_simple_array() {
    let map = HashMap::from([("a", "d"), ("b", "c")]);
    let ast = BoolExprParser::parse(Rule::main, "b=c AND a in (a,b,c,d)").expect("Parse error");
    assert_eq!(eval(ast, &map), true);
}

#[test]
fn test_simple_array_does_not_contain() {
    let map = HashMap::from([("a", "X"), ("b", "c")]);
    let ast = BoolExprParser::parse(Rule::main, "b=c AND a not in (a,b,c,d)").expect("Parse error");
    assert_eq!(eval(ast, &map), true);
}
#[test]
fn test_complex_array_test() {
    let map = HashMap::from([("a", "b"), ("c", "something more")]);
    let ast = BoolExprParser::parse(Rule::main, "a=b AND (b=c OR c in (d,e,'something more',h))")
        .expect("Parse error");
    assert_eq!(eval(ast, &map), true);
}

#[test]
fn test_complex_string_check() {
    let map = HashMap::from([("a", "b"), ("c", "something more")]);
    let ast = BoolExprParser::parse(Rule::main, "a=b and c='something more'").expect("Parse error");
    assert_eq!(eval(ast, &map), true);
}

#[test]
fn test_more_comparison() {
    let map = HashMap::from([("a", "10"), ("b", "something more")]);
    let ast = BoolExprParser::parse(Rule::main, "a>5 and c='something more'").expect("Parse error");
    assert_eq!(eval(ast, &map), true);
}