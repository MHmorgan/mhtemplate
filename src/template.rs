// Copyright 2020 Magnus Aa. Hirth. All rights reserved.
//
// The module `template` implements the `TemplateFactory` and all template
// nodes, which make up a template.

use crate::{Context, Evaluator, Expr, Lexeme, Result, Scanner, TmplError};
use std::collections::VecDeque;
use std::fmt;


// -----------------------------------------------------------------------------
// Template trait

/// Common trait used to interract with templates.
pub trait Template: fmt::Debug {
    fn evaluate(&self, ctx: &mut Context) -> Result<String>;
}

/*******************************************************************************
 *                                                                             *
 *  Template nodes
 *                                                                             *
 *******************************************************************************/

#[derive(Default, Debug)]
struct TextNode(String);

impl Template for TextNode {
    fn evaluate(&self, _ctx: &mut Context) -> Result<String> {
        Ok(self.0.clone())
    }
}

// -----------------------------------------------------------------------------
// Composite node

#[derive(Default, Debug)]
struct CompositeNode(Vec<Box<dyn Template>>);

impl Template for CompositeNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String> {
        let mut txt = String::new();
        for node in &self.0 {
            txt.push_str(&node.evaluate(ctx)?);
        }
        Ok(txt)
    }
}

impl CompositeNode {
    #[inline]
    fn add_node(&mut self, node: Box<dyn Template>) {
        self.0.push(node);
    }
}

// -----------------------------------------------------------------------------

// Set      { ident: String, val: Box<Expr> },
#[derive(Default, Debug)]
struct SetNode(String, Box<Expr>);

impl Template for SetNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String> {
        let val: String = self.1.evaluate(ctx)?;
        ctx.set_var(self.0.clone(), val);
        Ok(String::new())
    }
}

// -----------------------------------------------------------------------------

// For      { ident: String, expr: Box<Expr> },
// ForCnt   { ident: String, ident_i: String, expr: Box<Expr> },
#[derive(Default, Debug)]
struct ForNode {
    ident: String,
    cnt: Option<String>,
    expr: Box<Expr>,
    content: CompositeNode,
}

impl Template for ForNode {
    fn evaluate(&self, context: &mut Context) -> Result<String> {
        let mut ret = String::new();

        // Try to evaluate as number

        let num: Result<f64> = self.expr.evaluate(&context);
        if let Ok(num) = num {
            let num = num.abs() as usize;
            let mut new_ctx = Context::from(&*context);
            for i in (0..num).map(|i| i.to_string()) {
                new_ctx[&self.ident] = i.clone();
                if let Some(ident_i) = &self.cnt {
                    new_ctx[ident_i] = i;
                }
                ret.push_str(&self.content.evaluate(&mut new_ctx)?);
            }
            return Ok(ret);
        }

        // Evaluate as string as default

        let expr: String = self.expr.evaluate(&context)?;
        let mut new_ctx = Context::from(&*context);
        for (i, itm) in expr.split_whitespace().map(String::from).enumerate() {
            new_ctx[&self.ident] = itm;
            if let Some(ident_i) = &self.cnt {
                new_ctx[ident_i] = i.to_string();
            }
            ret.push_str(&self.content.evaluate(&mut new_ctx)?);
        }
        Ok(ret)
    }
}

// -----------------------------------------------------------------------------

// Expr     { expr: Box<Expr> },
#[derive(Default, Debug)]
struct ExprNode(Box<Expr>);

impl Template for ExprNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String> {
        self.0.evaluate(&ctx)
    }
}

// -----------------------------------------------------------------------------

// Repeat   { expr: Box<Expr> },
#[derive(Default, Debug)]
struct RepeatNode(Box<Expr>, CompositeNode);

impl Template for RepeatNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String> {
        let expr_f64: Result<f64> = self.0.evaluate(&ctx);
        let expr_str: Result<String> = self.0.evaluate(&ctx);

        // Get number of repetitions, either from integer value or word count
        let num: usize;
        match (expr_str, expr_f64) {
            (_, Ok(u)) => num = u.abs() as usize,
            (Ok(s), _) => num = s.split_whitespace().count(),
            (Err(e), _) => return Err(e),
        }

        // Evaluate node content
        let mut res = String::new();
        let mut ctx = Context::from(&*ctx);
        for _ in 0..num {
            res.push_str(&self.1.evaluate(&mut ctx)?);
        }
        Ok(res)
    }
}

// -----------------------------------------------------------------------------

// If       { expr: Box<Expr> },
// Elif     { expr: Box<Expr> },
// Else,
#[derive(Default, Debug)]
struct ConditionalNode(Vec<(Box<Expr>, CompositeNode)>, CompositeNode);

impl Template for ConditionalNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String> {
        // Return first matching conditional block
        for (expr, nodes) in &self.0 {
            let cond: bool = expr.evaluate(&ctx)?;
            if cond {
                return nodes.evaluate(ctx);
            }
        }

        // Else block
        self.1.evaluate(ctx)
    }
}

impl ConditionalNode {
    fn add_branch(&mut self, cond: Box<Expr>, block: CompositeNode) {
        self.0.push((cond, block));
    }

    fn set_default_branch(&mut self, block: CompositeNode) {
        self.1 = block;
    }
}

// -----------------------------------------------------------------------------
// Raw node, containing unparsed text

#[derive(Default, Debug)]
struct RawNode(String);

impl Template for RawNode {
    fn evaluate(&self, _ctx: &mut Context) -> Result<String> {
        Ok(self.0.clone())
    }
}

// -----------------------------------------------------------------------------
// With node, creating an inner scope

#[derive(Default, Debug)]
struct WithNode(CompositeNode);

impl Template for WithNode {
    fn evaluate(&self, ctx: &mut Context) -> Result<String> {
        let mut ctx = Context::from(&*ctx);
        self.0.evaluate(&mut ctx)
    }
}

/*******************************************************************************
 *                                                                             *
 *  Template factory
 *                                                                             *
 *******************************************************************************/

/// Parses template text to produce a template object, to be evaluated.
#[derive(Clone, Default)]
pub struct TemplateFactory {
    source: String,
    scanner: Scanner,
    lexemes: VecDeque<Lexeme>,

    // Lookahead & -behind state
    initialized: bool,
    cur_lex: Lexeme,
    prev_lex1: Lexeme,
    prev_lex2: Lexeme,
    next_lex1: Option<Lexeme>,
    next_lex2: Option<Lexeme>,
}

impl TemplateFactory {
    pub fn new(text: &str) -> Self {
        let scn = Scanner::new::<VecDeque<char>>(text.chars().collect());
        TemplateFactory {
            source: String::from(text),
            scanner: scn,
            ..Default::default()
        }
    }

    fn init(&mut self) -> Result<()> {
        if !self.initialized {
            self.initialized = true;
            self.scanner.scan_all()?;
            self.lexemes = self.scanner.clone().collect();
            self.next_lex1 = self.lexemes.pop_front();
            self.next_lex2 = self.lexemes.pop_front();
        }
        Ok(())
    }

    fn next_lexeme(&mut self) -> Option<Lexeme> {
        let lex = self.next_lex1.clone();
        self.next_lex1 = self.next_lex2.clone();
        self.next_lex2 = self.lexemes.pop_front();
        if let Some(lex) = &lex {
            self.prev_lex2 = self.prev_lex1.clone();
            self.prev_lex1 = self.cur_lex.clone();
            self.cur_lex = lex.clone();
        }
        lex
    }

    fn error<S: ToString>(&self, msg: S) -> TmplError {
        TmplError::new(msg.to_string())
    }

    // -------------------------------------------------------------------------
    // Parsing methods

    pub fn parse(&mut self) -> Result<Box<dyn Template>> {
        self.init()?;
        let root = self.parse_block()?;
        Ok(Box::new(root))
    }

    // Parse a block until reaching an end statement, returning the composite
    // node of the block content.
    fn parse_block(&mut self) -> Result<CompositeNode> {
        // Make error calling prettier
        macro_rules! err {
            ($($arg:tt)*) => {
                return Err(self.error(format!($($arg)*)))
            };
        }

        let mut block = CompositeNode {
            ..Default::default()
        };

        'parse_block: while let Some(lex) = self.next_lexeme() {
            match lex {
                Lexeme::Text { text } => {
                    block.add_node(Box::new(TextNode(text)));
                }

                // Ignore comments
                Lexeme::Comment { .. } => (),

                Lexeme::Set { ident, val } => {
                    block.add_node(Box::new(SetNode(ident, val)));
                }

                Lexeme::For { ident, expr } => {
                    let content = self.parse_block()?;
                    block.add_node(Box::new(ForNode {
                        ident,
                        cnt: None,
                        expr,
                        content,
                    }))
                }

                Lexeme::ForCnt {
                    ident,
                    ident_i,
                    expr,
                } => {
                    let content = self.parse_block()?;
                    block.add_node(Box::new(ForNode {
                        ident,
                        cnt: Some(ident_i),
                        expr,
                        content,
                    }))
                }

                Lexeme::Expr { expr } => block.add_node(Box::new(ExprNode(expr))),

                Lexeme::Repeat { expr } => {
                    let content = self.parse_block()?;
                    block.add_node(Box::new(RepeatNode(expr, content)))
                }

                Lexeme::If { expr } => {
                    let conditional = self.parse_conditional_branch(expr)?;
                    block.add_node(Box::new(conditional))
                }

                // Elif should only be found when parsing If
                Lexeme::Elif { .. } => {
                    err!("invalid elif outside if");
                }

                Lexeme::Raw => {
                    err!("raw statement not supported");
                }

                Lexeme::With => {
                    let content = self.parse_block()?;
                    block.add_node(Box::new(WithNode(content)))
                }

                // Reaching an end outside of a block stops template parsing
                Lexeme::End => {
                    break 'parse_block;
                }

                Lexeme::Else => {
                    err!("invalid else outside if");
                }

                Lexeme::Unknown => {
                    err!("found unknown lexeme");
                }
            }
        }

        Ok(block)
    }

    fn parse_conditional_branch(&mut self, first_cond: Box<Expr>) -> Result<ConditionalNode> {
        // Make error calling prettier
        macro_rules! err {
            ($($arg:tt)*) => {
                return Err(self.error(format!($($arg)*)))
            };
        }

        let mut cond_node = ConditionalNode::default();
        let mut condition = first_cond;
        let mut block = CompositeNode::default();
        let mut in_else = false;

        'parse_cond: while let Some(lex) = self.next_lexeme() {
            match lex {
                Lexeme::Text { text } => {
                    block.add_node(Box::new(TextNode(text)));
                }

                // Ignore comments
                Lexeme::Comment { .. } => (),

                Lexeme::Set { ident, val } => {
                    block.add_node(Box::new(SetNode(ident, val)));
                }

                Lexeme::For { ident, expr } => {
                    let content = self.parse_block()?;
                    block.add_node(Box::new(ForNode {
                        ident,
                        cnt: None,
                        expr,
                        content,
                    }))
                }

                Lexeme::ForCnt {
                    ident,
                    ident_i,
                    expr,
                } => {
                    let content = self.parse_block()?;
                    block.add_node(Box::new(ForNode {
                        ident,
                        cnt: Some(ident_i),
                        expr,
                        content,
                    }))
                }

                Lexeme::Expr { expr } => block.add_node(Box::new(ExprNode(expr))),

                Lexeme::Repeat { expr } => {
                    let content = self.parse_block()?;
                    block.add_node(Box::new(RepeatNode(expr, content)))
                }

                Lexeme::If { expr } => {
                    let conditional = self.parse_conditional_branch(expr)?;
                    block.add_node(Box::new(conditional))
                }

                // Elif should only be found when parsing If
                Lexeme::Elif { expr } => {
                    cond_node.add_branch(condition.clone(), block);
                    condition = expr;
                    block = CompositeNode::default();
                }

                Lexeme::Raw => {
                    err!("raw statement not supported");
                }

                Lexeme::With => {
                    let content = self.parse_block()?;
                    block.add_node(Box::new(WithNode(content)))
                }

                // Reaching an end outside of a block stops template parsing
                Lexeme::End => {
                    if in_else {
                        cond_node.set_default_branch(block);
                    }
                    break 'parse_cond;
                }

                Lexeme::Else => {
                    cond_node.add_branch(condition.clone(), block);
                    block = CompositeNode::default();
                    in_else = true;
                }

                Lexeme::Unknown => {
                    err!("found unknown lexeme");
                }
            }
        }

        Ok(cond_node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let text = String::from(
            "
            {%- set MYVAL \"Verden!\" -%}
            Hello {{ $MYVAL }}",
        );
        let tmpl = TemplateFactory::new(&text).parse().unwrap();
        let text = tmpl.evaluate(&mut Context::new()).unwrap();
        assert_eq!(text, String::from("Hello Verden!"));
    }
}
