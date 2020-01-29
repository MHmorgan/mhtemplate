// Copyright 2020 Magnus Aa. Hirth. All rights reserved.

use mhtemplate::{Context, TemplateFactory};

#[test]
fn test_template() {
    let want = include_str!("text.txt");
    let text = include_str!("text.template");
    let tmpl = TemplateFactory::new(&text).parse().unwrap();
    let text = tmpl.evaluate(&mut Context::new()).unwrap();

    // println!("================================");
    // println!("{}", &text);
    // println!("--------------------------------");
    // println!("{}", &want);
    // println!("================================");
    assert_eq!(text, want);
}
