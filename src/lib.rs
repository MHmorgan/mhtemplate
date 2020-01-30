//! Text template library.
//!
//! A template is a text which may contain special delimiters:
//! * Statements: `{% ... %}`
//! * Expression: `{{ ... }}`
//! * Comments:   `{# ... #}`
//! 
//! A hyphen may be added to any delimiter, `{%-` or `-%}`, to remove leading 
//! or trailing whitespace, respectively.
//! 
//!
//! ## Expressions
//!
//! Template expressions may consist of constants, variables and operators.
//! There are no concept of operator precedence, and evaluation are done right to left.
//! However, this may change in future releases.
//! 
//! Expressions occur inside expression delimiters: `{{ ... }}` or some statements,
//! and may be evaluated as string, number or boolean, depending on context.
//! 
//! Supported expressions:
//! 
//! | Expression         | Comment                                                              |
//! |--------------------|----------------------------------------------------------------------|
//! | `<var>`            | A variable, ex: `$foo`                                               |
//! | `<const>`          | A constant, ex: `6` or `"Hello World"`                               |
//! | `<expr>++`         | Increment. `<expr>` is evaluated as number                           |
//! | `<expr>--`         | Decrement. `<expr>` is evaluated as number                           |
//! | `<expr> + <expr>`  | Addition. `<expr>` is evaluated as numbers.                          |
//! | `<expr> - <expr>`  | Subtraction. `<expr>` is evaluated as numbers.                       |
//! | `<expr> * <expr>`  | Multiplication. `<expr>` is evaluated as numbers.                    |
//! | `<expr> / <expr>`  | Division. `<expr>` is evaluated as numbers.                          |
//! | `<expr> < <expr>`  | Less than. `<expr>` is evaluated as numbers.                         |
//! | `<expr> > <expr>`  | Greater than. `<expr>` is evaluated as numbers.                      |
//! | `<expr> <= <expr>` | Less or equal to. `<expr>` is evaluated as numbers.                  |
//! | `<expr> >= <expr>` | Greater or equal to. `<expr>` is evaluated as numbers.               |
//! | `<expr> == <expr>` | Equal. `<expr>` are both either evaluated as numbers or strings.     |
//! | `<expr> != <expr>` | Not equal. `<expr>` are both either evaluated as numbers or strings. |
//! | `<expr> . <expr>`  | Concatenation. `<expr>` are evaluated as strings.                    |
//! 
//! Expressions evaluated as booleans will evaluated as: 
//! * Number: "0" is false, anything else is true.
//! * "true"/"false" strings (case insensitive) is true/false. 
//! * A comparison expression which returns true/false.
//! * Anything else is false.
//! 
//!
//! ## Statements
//!
//! Statements occur inside statement delimiters: `{% ... %}`.
//! Supported statements are loops, conditionals, `set` and `with`.
//!
//! ### Loops
//! 
//! A repeat loop:
//! ```text
//! {% repeat <expr> %}
//! Hello
//! {% end %}
//! ```
//!
//! A for loop:
//! ```text
//! {% for <ident> in <expr> %}
//! Hello {{ $<ident> }}
//! {% end %}
//! ```
//!
//! A enumerated for loop:
//! ```text
//! {% for <ident_i>, <ident> in <expr> %}
//! Hello {{ $<ident> }} (element nr. {{ $<ident_i> + 1 }})
//! {% end %}
//! ```
//! 
//! Expressions in loops are evaluated as numbers or strings. As number the loop
//! will iterate over [0,n). As a string the loop will iterate over the words in 
//! the string, split at whitespaces.
//! 
//! ### Conditionals
//! 
//! if/elif/else conditional:
//! ```text
//! {% if <expr> -%}
//!     Hey if
//! {% elif <expr> -%}
//!     Hey elif
//! {% else -%}
//!     Hey else
//! {% end %}
//! ```
//! 
//! Any expression in a conditional statement are evaluated as boolean.
//! 
//! 
//! ## Context
//! 
//! A context is passed to a template during evaluation, which may contain
//! values of variables available during evaluation.
//! 
//! A context may be created from a `HashMap` or environment variables.
//! 
//! Simple context:
//! ```
//! # use mhtemplate::Context;
//! let mut ctx = Context::new();
//! ctx["NUMBER"] = String::from("42");
//! ```
//! 
//! Context from environment:
//! ```
//! # use mhtemplate::Context;
//! let vars = std::env::vars();
//! let ctx  = Context::from(vars);
//! ```
//! 
//!
//! ## Example
//!
//! Template text:
//! ```text
//! {% set num 4
//!    set str Hello World %}
//! 
//! {%- repeat $num -%}
//! Brrapp {% end %}
//! 
//! {%- for VAL in $num -%}
//! {{ $VAL }}
//! {% end %}
//! 
//! {% for i, s in $str -%}
//! {{ $i }} - {{ $s }}
//! {%end%}
//! 
//! {%- if $num > 6 %}
//! NOT PRINTED
//! {%- elif $num == 4 %}
//! PRINTED
//! {%- else %}
//! NOT PRINTED
//! {%- end %}
//! 
//! 
//! {%- with
//!    set num (($num*2) - 1)--
//!    set str UGH %}
//! Ut enim ad minim {{ $str }}, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. 
//! 
//! Duis aute irure dolor in reprehenderit in voluptate velit {{ $num }} cillum dolore eu fugiat nulla pariatur.
//! {% end -%}
//! 
//! Excepteur sint occaecat cupidatat non {{ $str }}, sunt in culpa qui officia deserunt mollit anim id est {{ $num }}.
//! ```
//! 
//! Rust unsage:
//! ```
//! # use mhtemplate::{Context, TemplateFactory};
//! let text = String::from("{% repeat 3 %} Hello {% end %}");
//! let tmpl = TemplateFactory::new(&text).parse().unwrap();
//! let text = tmpl.evaluate(&mut Context::new()).unwrap();
//! ```
//! 
//! The evaluated template returns: 
//! ```text
//! Brrapp Brrapp Brrapp Brrapp 0
//! 1
//! 2
//! 3
//! 
//! 
//! 0 - Hello
//! 1 - World
//! 
//! PRINTED
//! Ut enim ad minim UGH, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
//! 
//! Duis aute irure dolor in reprehenderit in voluptate velit 6 cillum dolore eu fugiat nulla pariatur.
//! Excepteur sint occaecat cupidatat non Hello World, sunt in culpa qui officia deserunt mollit anim id est 4.
//! ```

// Copyright 2020 Magnus Aa. Hirth. All rights reserved.

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
