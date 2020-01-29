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

mod error;
mod template;
mod lex;
mod expr;
mod context;

pub use error::{TmplError, Result};

use expr::{Evaluator,Expr,ExpressionFactory};
use lex::{Lexeme,Scanner};
pub use context::Context;
pub use template::{Template,TemplateFactory};
