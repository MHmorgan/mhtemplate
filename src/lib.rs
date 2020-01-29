//! Text template library.
//!
//! A template is a text which may contain special tokens:
//! * Statements: `{% ... %}`
//! * Expression: `{{ ... }}`
//! * Comments:   `{# ... #}`
//!
//! ## Expressions
//!
//! TODO
//!
//! ## Statements
//!
//! Supported statements are loops, conditionals, `set` and `with`.
//!
//! ### Loops
//!
//! A repeat loop:
//! ```Text
//! {% repeat <expr> %}
//! Hello
//! {% end %}
//! ```
//!
//! ## Example
//!
//! ```
//! ```

// Copyright 2019 Magnus Aa. Hirth. All rights reserved.

#[macro_use]
extern crate lazy_static;

mod context;
mod error;
mod expr;
mod lex;
mod template;

pub use error::{Result, TmplError};

pub use context::Context;
use expr::{Evaluator, Expr, ExpressionFactory};
use lex::{Lexeme, Scanner};
pub use template::{Template, TemplateFactory};
