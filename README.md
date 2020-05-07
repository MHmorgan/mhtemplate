![MHtemplate](./mhtemplate.png)

[![Travis CI build status](https://img.shields.io/travis/com/MHmorgan/mhtemplate/master?style=flat-square)](https://travis-ci.com/MHmorgan/mhtemplate)
[![Crates.io latest version](https://img.shields.io/crates/v/mhtemplate?style=flat-square)](https://crates.io/crates/mhtemplate)
![Crates.io downloads](https://img.shields.io/crates/d/mhtemplate?style=flat-square)
![GitHub license](https://img.shields.io/github/license/MHmorgan/mhtemplate?style=flat-square)

MHtemplate is an easy-to-use, dynamic text template library.

Usage
-----

```toml
[dependencies]
mhtemplate = "~1.0.0"
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

