extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "bool_expr.pest"]
pub struct BoolExprParser;

fn main() {

    let expression = r#"as22 IN (a,v,'c d',213) 
    or (a!=2 and ds='seo ew') 
    OR demo in ("zom", ds, 2323) 
    and a=z 
    AND !(b=3 or b=ds)"#;

    let ast = BoolExprParser::parse(Rule::main, &expression).expect("Failed to parse");
    println!("Tree: {:#?}", ast);
}
