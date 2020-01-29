//! Text template library.
//! 
//! TODO
//! ====
//! * Call evaluate multiple times on a template, allowing multiple errors to
//!   be recorded by calling evaluate after previous call returned error.

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
