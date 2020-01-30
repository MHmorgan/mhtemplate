mhtemplate
==========

[![Latest version](https://img.shields.io/crates/v/mhtemplate.svg)](https://crates.io/crates/mhtemplate)
[![Documentation](https://docs.rs/mhtemplate/badge.svg)](https://docs.rs/mhtemplate/)
[![GitHub license](https://img.shields.io/github/license/MHmorgan/rustmhtemplate)](https://github.com/MHmorgan/rustmhtemplate/blob/master/LICENSE)

mhtemplate is an easy-to-use, dynamic text template library.

Usage
-----

```toml
[dependencies]
mhtemplate = "1"
```

```rust
extern crate mhtemplate;

use mhtemplate::{Context, TemplateFactory};

let text = include_str!("text.template");
let tmpl = TemplateFactory::new(&text).parse().unwrap();
let text = tmpl.evaluate(&mut Context::new()).unwrap();
```


Changelog
---------

