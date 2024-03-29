use super::*;
use pest::Parser;

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

    // quoted string
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "country_code =   'IL'").expect("Parse error"),
            &map
        ),
        true
    );

    // single quoted string
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "country_code =  \"IL\"").expect("Parse error"),
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
    let ast = BoolExprParser::parse(Rule::main, "a>5 and b in (try, 'something more')")
        .expect("Parse error");
    assert_eq!(eval(ast, &map), true);
}

#[test]
fn test_more_comparison_string() {
    let map = HashMap::from([("a", "10"), ("b", "something more")]);
    let ast = BoolExprParser::parse(Rule::main, "a>'demo' and b in (try, 'something more')")
        .expect("Parse error");
    assert_eq!(eval(ast, &map), false);
}

#[test]
fn test_more_or_equal() {
    let map = HashMap::from([("a", "10")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a>=10 or b in (try, 'something more')")
                .expect("Parse error"),
            &map
        ),
        true
    );
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a>=9 or b in (try, 'something more')")
                .expect("Parse error"),
            &map
        ),
        true
    );
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a>=12 or b in (try, 'something more')")
                .expect("Parse error"),
            &map
        ),
        false
    );
}

#[test]
fn test_less() {
    let map = HashMap::from([("a", "10")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a<11 or b in (try, 'something more')")
                .expect("Parse error"),
            &map
        ),
        true
    );
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a<10 or b in (try, 'something more')")
                .expect("Parse error"),
            &map
        ),
        false
    );
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a<9 or b in (try, 'something more')")
                .expect("Parse error"),
            &map
        ),
        false
    );
}

#[test]
fn test_less_or_equal() {
    let map = HashMap::from([("a", "10")]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a<=11 or b in (try, 'something more')")
                .expect("Parse error"),
            &map
        ),
        true
    );
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a<=10 or b in (try, 'something more')")
                .expect("Parse error"),
            &map
        ),
        true
    );
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a<=9 or b in (try, 'something more')")
                .expect("Parse error"),
            &map
        ),
        false
    );
}

#[test]
fn test_ugly_massive_test() {
    let expression = r#"as22 IN (a,v,'c d',213) 
            or (a!=2 and ds='seo ew') 
            OR demo in ("zom", ds, 2323) 
            and a=z 
            AND !(b=3 or b=ds)"#;

    let ast = BoolExprParser::parse(Rule::main, &expression).expect("Failed to parse");
    let map = HashMap::from([("as22", "c d"), ("ds", "seo ew"), ("a", "z"), ("b", "ss")]);
    assert_eq!(eval(ast, &map), true);
}

#[test]
fn test_in_readme_documentation() {
    let expression = r###"(countryCode=NL or countryCode=DE) 
    AND uid in (121321,2312312,231231) 
    and role in (Admin, "Super admin")
    and (
        uid not in ( ca3ed35c-114f-488d-82b7-7c4d1bd5cbcd,  b83f48af-ecb6-4b50-9d5e-b690db2a332b ) 
        or uid <= 0
    ) 
    and !(street_name='Random street 1' and countryCode=NL)"###;

    let map = HashMap::from([
        ("countryCode", "DE"),
        ("uid", "2312312"),
        ("role", "Super admin"),
        ("street_name", "Random street 2"),
    ]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, &expression).expect("Parse error"),
            &map
        ),
        true
    );
}

#[test]
fn test_realworld_demo_values() {
    let rule = r###"accountRole in (Admin,admin,"Admin/Order Manager")
    and account_country_code in (LT , NL, DE, GB, US)
    and account_uuid in (543987b0-e69f-41ec-9a68-cfc5cfb15afe, 6133b8d6-4078-4270-9a68-fa0ac78bf512)
    and accountType in ("Some Corporate & Managament Type", Corporate , Managament)"###;

    let map = HashMap::from([
        ("accountRole", "Admin/Order Manager"),
        ("account_country_code", "LT"),
        ("account_uuid", "543987b0-e69f-41ec-9a68-cfc5cfb15afe"),
        ("accountType", "Some Corporate & Managament Type"),
    ]);
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, &rule).expect("Parse error"),
            &map
        ),
        true
    );
}

#[test]
fn test_number_parsing_handling_error_scenario() {
    assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, "a > \"b\"").expect("Parse error"),
            &HashMap::from([("a", "20"),])
        ),
        false
    );
}
