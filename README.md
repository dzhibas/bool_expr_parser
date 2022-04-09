# Bool Expr Parser lib

Boolean expresion parser and evaluation function for feature flagging - flipper

It parses expression and given input hashmap evaluates and returns true/false

Example of complex expression you can parse and evaluate:

```
(countryCode=NL or countryCode=DE) 
    AND uid in (121321,2312312,231231) 
    and role in (Admin, "Super admin")
    and (
        uid not in ( ca3ed35c-114f-488d-82b7-7c4d1bd5cbcd,  b83f48af-ecb6-4b50-9d5e-b690db2a332b ) 
        or uid <= 0
    ) 
    and !(street_name='Random street 1' and countryCode=NL)
```

given a input hashmap of
```
    let map = HashMap::from([
        ("countryCode", "DE"),
        ("uid", "2312312"),
        ("role", "Super admin"),
        ("street_name", "Random street 2"),
    ]);
```

result would be `true`

```
assert_eq!(
        eval(
            BoolExprParser::parse(Rule::main, &expression)
                .expect("Parse error"),
            &map
        ),
        true
    );
```

Please see test in src/test.rs test_in_readme_documentation()