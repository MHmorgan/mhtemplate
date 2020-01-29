// Copyright 2020 Magnus Aa. Hirth. All rights reserved.
//
// The internal module `expr` implements the `ExpressionFactory`, defines the
// expression type `Expr`, and the generic trait `Evaluator` implement by
// `Expr`.

// TODO: add precedence to operators in expression
use crate::{Context, Result, TmplError};
use std::collections::VecDeque;
use std::fmt;

/// Any expression that should be evaluated are an evaluator. Used to as an
/// abstraction to hide concrete expression types.
pub trait Evaluator<T> {
    fn evaluate(&self, ctx: &Context) -> Result<T>;
}

// Abstraction of conversion fram a template string value to a rust boolean.
fn str2bool(s: &str) -> bool {
    let num: std::result::Result<f64, _> = s.parse();
    let txt: std::result::Result<bool, _> = s.to_lowercase().parse();
    // Accept number evaluation and true/false evaluation
    match (num, txt) {
        (_, Ok(t)) => t,
        (Ok(n), _) => n != 0.0,
        _ => false,
    }
}

#[inline]
fn feq(lhs: f64, rhs: f64) -> bool {
    let error = 0.0001;
    (lhs - rhs).abs() < error
}

/*******************************************************************************
 *                                                                             *
 *  Expression
 *                                                                             *
 *******************************************************************************/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Const { val: String },
    Variable { ident: String },
    // Unary
    Neg { expr: Box<Expr> },
    Not { expr: Box<Expr> },
    Inc { expr: Box<Expr> },
    Dec { expr: Box<Expr> },
    // Arithmetic
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    Mul { lhs: Box<Expr>, rhs: Box<Expr> },
    Div { lhs: Box<Expr>, rhs: Box<Expr> },
    // Logical
    Lt { lhs: Box<Expr>, rhs: Box<Expr> },
    Gt { lhs: Box<Expr>, rhs: Box<Expr> },
    Leq { lhs: Box<Expr>, rhs: Box<Expr> },
    Geq { lhs: Box<Expr>, rhs: Box<Expr> },
    Eq { lhs: Box<Expr>, rhs: Box<Expr> },
    Neq { lhs: Box<Expr>, rhs: Box<Expr> },
    // String
    Concat { lhs: Box<Expr>, rhs: Box<Expr> },
    Unknown,
}

impl Expr {
    pub fn is_unknown(&self) -> bool {
        match self {
            Expr::Unknown => true,
            _ => false,
        }
    }

    // Print the expression as a tree
    #[allow(unused)]
    pub fn print_expression(&self, depth: u32) {
        let mut indent = String::new();
        for _ in 0..depth {
            indent.push_str("- ");
        }

        match self {
            Expr::Const { val } => {
                println!("{}Const({})", indent, val);
            }

            Expr::Variable { ident } => {
                println!("{}Variable({})", indent, ident);
            }

            Expr::Neg { expr } => {
                println!("{}Neg", indent);
                expr.print_expression(depth + 1);
            }

            Expr::Not { expr } => {
                println!("{}Not", indent);
                expr.print_expression(depth + 1);
            }

            Expr::Inc { expr } => {
                println!("{}Inc", indent);
                expr.print_expression(depth + 1);
            }

            Expr::Dec { expr } => {
                println!("{}Dec", indent);
                expr.print_expression(depth + 1);
            }

            Expr::Add { lhs, rhs } => {
                println!("{}Add", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Sub { lhs, rhs } => {
                println!("{}Sub", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Mul { lhs, rhs } => {
                println!("{}Mul", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Div { lhs, rhs } => {
                println!("{}Div", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Lt { lhs, rhs } => {
                println!("{}Lt", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Gt { lhs, rhs } => {
                println!("{}Gt", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Leq { lhs, rhs } => {
                println!("{}Leq", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Geq { lhs, rhs } => {
                println!("{}Geq", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Eq { lhs, rhs } => {
                println!("{}Eq", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Neq { lhs, rhs } => {
                println!("{}Neq", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Concat { lhs, rhs } => {
                println!("{}Concat", indent);
                lhs.print_expression(depth + 1);
                rhs.print_expression(depth + 1);
            }

            Expr::Unknown => {
                println!("{}Unknown", indent);
            }
        }
    }
}

impl Evaluator<String> for Expr {
    fn evaluate(&self, ctx: &Context) -> Result<String> {
        match self {
            Expr::Const { val } => Ok(val.clone()),
            Expr::Variable { ident } => {
                if let Some(val) = ctx.var(ident) {
                    Ok(val.clone())
                } else {
                    Err(TmplError::new(format!("non existing variable: ${}", ident)))
                }
            }
            Expr::Neg { expr } => {
                let mut val: f64 = expr.evaluate(ctx)?;
                val = -val;
                Ok(val.to_string())
            }
            Expr::Not { expr } => {
                let mut val: bool = expr.evaluate(&ctx)?;
                val = !val;
                Ok(val.to_string())
            }
            Expr::Inc { expr } => {
                let mut val: f64 = expr.evaluate(&ctx)?;
                val += 1.0;
                Ok(val.to_string())
            }
            Expr::Dec { expr } => {
                let mut val: f64 = expr.evaluate(&ctx)?;
                val -= 1.0;
                Ok(val.to_string())
            }
            Expr::Add { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs + rhs;
                Ok(res.to_string())
            }
            Expr::Sub { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs - rhs;
                Ok(res.to_string())
            }
            Expr::Mul { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs * rhs;
                Ok(res.to_string())
            }
            Expr::Div { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs / rhs;
                Ok(res.to_string())
            }
            Expr::Lt { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs < rhs;
                Ok(res.to_string())
            }
            Expr::Gt { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs > rhs;
                Ok(res.to_string())
            }
            Expr::Leq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs <= rhs;
                Ok(res.to_string())
            }
            Expr::Geq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs >= rhs;
                Ok(res.to_string())
            }
            Expr::Eq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = feq(lhs, rhs);
                Ok(res.to_string())
            }
            Expr::Neq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = !feq(lhs, rhs);
                Ok(res.to_string())
            }
            Expr::Concat { lhs, rhs } => {
                let lhs: String = lhs.evaluate(&ctx)?;
                let rhs: String = rhs.evaluate(&ctx)?;
                let res = lhs + &rhs;
                Ok(res.clone())
            }
            Expr::Unknown => Err(TmplError::new("cannot evaluate unknown expression")),
        }
    }
}

impl Evaluator<f64> for Expr {
    fn evaluate(&self, ctx: &Context) -> Result<f64> {
        match self {
            Expr::Const { val } => {
                let val: f64 = val.parse()?;
                Ok(val)
            }
            Expr::Variable { ident } => {
                if let Some(val) = ctx.var(ident) {
                    Ok(val.parse()?)
                } else {
                    Err(TmplError::new(format!("non existing variable: ${}", ident)))
                }
            }
            Expr::Neg { expr } => {
                let mut val: f64 = expr.evaluate(&ctx)?;
                val = -val;
                Ok(val)
            }
            Expr::Not { expr } => {
                let mut val: bool = expr.evaluate(&ctx)?;
                val = !val;
                if val {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            Expr::Inc { expr } => {
                let mut val: f64 = expr.evaluate(&ctx)?;
                val += 1.0;
                Ok(val)
            }
            Expr::Dec { expr } => {
                let mut val: f64 = expr.evaluate(&ctx)?;
                val -= 1.0;
                Ok(val)
            }
            Expr::Add { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs + rhs;
                Ok(res)
            }
            Expr::Sub { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs - rhs;
                Ok(res)
            }
            Expr::Mul { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs * rhs;
                Ok(res)
            }
            Expr::Div { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs / rhs;
                Ok(res)
            }
            Expr::Lt { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs < rhs;
                if res {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            Expr::Gt { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs > rhs;
                if res {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            Expr::Leq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs <= rhs;
                if res {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            Expr::Geq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs >= rhs;
                if res {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            Expr::Eq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = feq(lhs, rhs);
                if res {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            Expr::Neq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = !feq(lhs, rhs);
                if res {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            Expr::Concat { lhs, rhs } => {
                let lhs: String = lhs.evaluate(&ctx)?;
                let rhs: String = rhs.evaluate(&ctx)?;
                let res = lhs + &rhs;
                Ok(res.parse()?)
            }
            Expr::Unknown => Err(TmplError::new("cannot evaluate unknown expression")),
        }
    }
}

impl Evaluator<bool> for Expr {
    fn evaluate(&self, ctx: &Context) -> Result<bool> {
        match self {
            Expr::Const { val } => Ok(str2bool(val)),
            Expr::Variable { ident } => {
                if let Some(val) = ctx.var(ident) {
                    Ok(str2bool(val))
                } else {
                    Err(TmplError::new(format!("non existing variable: ${}", ident)))
                }
            }
            Expr::Neg { expr } => {
                let mut val: f64 = expr.evaluate(&ctx)?;
                val = -val;
                Ok(val != 0.0)
            }
            Expr::Not { expr } => {
                let val: bool = expr.evaluate(&ctx)?;
                Ok(!val)
            }
            Expr::Inc { expr } => {
                let mut val: f64 = expr.evaluate(&ctx)?;
                val += 1.0;
                Ok(val != 0.0)
            }
            Expr::Dec { expr } => {
                let mut val: f64 = expr.evaluate(&ctx)?;
                val -= 1.0;
                Ok(val != 0.0)
            }
            Expr::Add { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs + rhs;
                Ok(res != 0.0)
            }
            Expr::Sub { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs - rhs;
                Ok(res != 0.0)
            }
            Expr::Mul { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs * rhs;
                Ok(res != 0.0)
            }
            Expr::Div { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                let res = lhs / rhs;
                Ok(res != 0.0)
            }
            Expr::Lt { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                Ok(lhs < rhs)
            }
            Expr::Gt { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                Ok(lhs > rhs)
            }
            Expr::Leq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                Ok(lhs <= rhs)
            }
            Expr::Geq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                Ok(lhs >= rhs)
            }
            Expr::Eq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                Ok(feq(lhs, rhs))
            }
            Expr::Neq { lhs, rhs } => {
                let lhs: f64 = lhs.evaluate(&ctx)?;
                let rhs: f64 = rhs.evaluate(&ctx)?;
                Ok(!feq(lhs, rhs))
            }
            Expr::Concat { lhs, rhs } => {
                let lhs: String = lhs.evaluate(&ctx)?;
                let rhs: String = rhs.evaluate(&ctx)?;
                let val = lhs + &rhs;
                Ok(str2bool(&val))
            }
            Expr::Unknown => Err(TmplError::new("cannot evaluate unknown expression")),
        }
    }
}

impl Default for Expr {
    fn default() -> Self {
        Expr::Unknown
    }
}

// -----------------------------------------------------------------------------
// Expression token

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExprToken {
    Const { val: String },
    Variable { ident: String },
    Increase,
    Decrease,
    Pluss,
    Minus,
    Muliply,
    Divide,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    Not,
    Concat,
    OpenBracket,
    CloseBracket,
    Unknown,
}

impl ExprToken {
    #[allow(unused)]
    pub fn is_unknown(&self) -> bool {
        match self {
            ExprToken::Unknown => true,
            _ => false,
        }
    }

    fn is_open_bracket(&self) -> bool {
        match self {
            ExprToken::OpenBracket => true,
            _ => false,
        }
    }
}

impl Default for ExprToken {
    fn default() -> Self {
        ExprToken::Unknown
    }
}

impl fmt::Display for ExprToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/******************************************************************************
 *                                                                             *
 *  Expression factory
 *                                                                             *
 *******************************************************************************/

#[derive(Default)]
pub struct ExpressionFactory {
    buf: VecDeque<char>,
    // Expression tokens
    tokens: VecDeque<ExprToken>,

    // Character lookaheads
    cur_ch: char,
    prev_ch: char,
    next_ch: Option<char>,

    // Token lookaheads and lookbacks
    cur_tok: ExprToken,
    prev_tok: ExprToken,
    next_tok: Option<ExprToken>,
}

impl ExpressionFactory {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<VecDeque<char>>,
    {
        let mut efac = ExpressionFactory {
            buf: text.into(),
            ..Default::default()
        };
        // Update lookaheads
        efac.next_char();
        efac
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.next_ch;
        self.next_ch = self.buf.pop_front();
        if let Some(ch) = ch {
            self.prev_ch = self.cur_ch;
            self.cur_ch = ch;
        }
        ch
    }

    fn next_token(&mut self) -> Option<ExprToken> {
        let tok = self.next_tok.clone();
        self.next_tok = self.tokens.pop_front();
        if let Some(tok) = &tok {
            self.prev_tok = self.cur_tok.clone();
            self.cur_tok = tok.clone();
        }
        tok
    }

    /// Parse the expression, returning the top expression
    pub fn parse(&mut self) -> Result<Expr> {
        // Generate expression tokens

        while let Some(ch) = self.next_char() {
            match ch {
                // Variable
                '$' => {
                    let ident = self.scan_ident()?;
                    self.tokens.push_back(ExprToken::Variable { ident });
                }
                // Increase
                '+' if Some('+') == self.next_ch => {
                    self.tokens.push_back(ExprToken::Increase);
                    self.next_char();
                }
                // Decrease
                '-' if Some('-') == self.next_ch => {
                    self.tokens.push_back(ExprToken::Decrease);
                    self.next_char();
                }
                // Add
                '+' => self.tokens.push_back(ExprToken::Pluss),
                // Subtract
                '-' => self.tokens.push_back(ExprToken::Minus),
                // Multiply
                '*' => self.tokens.push_back(ExprToken::Muliply),
                // Divide
                '/' => self.tokens.push_back(ExprToken::Divide),
                // LessEqual
                '<' if Some('=') == self.next_ch => {
                    self.tokens.push_back(ExprToken::LessEqual);
                    self.next_char();
                }
                // GreaterEqual
                '>' if Some('=') == self.next_ch => {
                    self.tokens.push_back(ExprToken::GreaterEqual);
                    self.next_char();
                }
                // LessThan
                '<' => self.tokens.push_back(ExprToken::LessThan),
                // GreaterThan
                '>' => self.tokens.push_back(ExprToken::GreaterThan),
                // Equal
                '=' if Some('=') == self.next_ch => {
                    self.tokens.push_back(ExprToken::Equal);
                    self.next_char();
                }
                // NotEqual
                '!' if Some('=') == self.next_ch => {
                    self.tokens.push_back(ExprToken::NotEqual);
                    self.next_char();
                }
                // Not
                '!' => self.tokens.push_back(ExprToken::Not),
                // Concat
                '.' => self.tokens.push_back(ExprToken::Concat),
                // OpenBracket
                '(' => self.tokens.push_back(ExprToken::OpenBracket),
                // CloseBracket
                ')' => self.tokens.push_back(ExprToken::CloseBracket),
                // Quoted const
                '"' | '\'' => {
                    let val = self.scan_value(Some(ch))?;
                    self.tokens.push_back(ExprToken::Const { val });
                }
                // Ignore whitespaces in expression
                _ if ch.is_whitespace() => (),
                // Const
                _ => {
                    let val = self.scan_value(None)?;
                    self.tokens.push_back(ExprToken::Const { val });
                }
            }
        }

        // Update expression token lookaheads
        // self.next_token();
        self.next_token();

        // Parse expression tokens
        self.parse_expression()
    }

    fn error(&self, msg: impl ToString) -> TmplError {
        TmplError::new(msg.to_string())
    }

    // -------------------------------------------------------------------------
    // Scanner functions

    // Scan identifier from the buffer.
    fn scan_ident(&mut self) -> Result<String> {
        let mut ident = String::new();

        'scan: while let Some(ch) = self.next_ch {
            match ch {
                '_' => ident.push(ch),
                _ if ch.is_alphanumeric() => ident.push(ch),
                _ => break 'scan,
            }
            self.next_char();
        }

        Ok(ident)
    }

    // Scan a value from the buffer. If `quote` is not none then characters
    // are scanned until the quote character is encountered.
    // A newline results in an error.
    fn scan_value(&mut self, quote: Option<char>) -> Result<String> {
        // The first character of the value was the previous character
        let mut val = String::new();

        if let Some(quote) = quote {
            // Scan quoted characters
            'scan_quoted: while let Some(ch) = self.next_char() {
                match ch {
                    '\n' => return Err(TmplError::new("invalid expression value")),
                    _ if ch == quote => break 'scan_quoted,
                    _ => val.push(ch),
                }
            }
        } else {
            val.push(self.cur_ch);
            // Scan unquoted charaters
            'scan_unquoted: while let Some(ch) = self.next_ch {
                match ch {
                    '.' | '_' => val.push(ch),
                    _ if ch.is_alphanumeric() => val.push(ch),
                    _ if ch.is_whitespace() => val.push(ch),
                    _ => break 'scan_unquoted,
                }
                self.next_char();
            }
        }

        Ok(String::from(val.trim()))
    }

    // -------------------------------------------------------------------------
    // Parser functions

    fn parse_expression(&mut self) -> Result<Expr> {
        // Make error calling prettier
        macro_rules! err {
            ($($arg:tt)*) => {
                return Err(self.error(format!($($arg)*)))
            };
        }

        let in_bracket = self.cur_tok.is_open_bracket();
        let mut expr = Expr::default();

        'parse: while let Some(tok) = self.next_token() {
            match tok {
                ExprToken::Const { val } => {
                    expr = Expr::Const { val };
                    if Some(ExprToken::CloseBracket) == self.next_tok {
                        break 'parse;
                    }
                }

                ExprToken::Variable { ident } => {
                    expr = Expr::Variable { ident };
                    if Some(ExprToken::CloseBracket) == self.next_tok {
                        break 'parse;
                    }
                }

                // misplaced increment
                ExprToken::Increase if expr.is_unknown() => {
                    err!("invalid increment");
                }

                // increment
                ExprToken::Increase => {
                    expr = Expr::Inc {
                        expr: Box::new(expr),
                    };
                }

                // pre decrement
                ExprToken::Decrease if expr.is_unknown() => {
                    // return Err(self.error("invalid decrement"));
                    err!("invalid decrement");
                }

                // post decrement
                ExprToken::Decrease => {
                    expr = Expr::Dec {
                        expr: Box::new(expr),
                    };
                }

                ExprToken::Pluss if expr.is_unknown() => (),

                ExprToken::Pluss => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Add {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::Minus if expr.is_unknown() => {
                    expr = self.parse_neg()?;
                }

                ExprToken::Minus => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Sub {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::Muliply => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Mul {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::Divide => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Div {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::LessThan => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Lt {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::GreaterThan => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Gt {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::LessEqual => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Leq {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::GreaterEqual => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Geq {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::Equal => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Eq {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::NotEqual => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Neq {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::Not => {
                    expr = self.parse_not()?;
                }

                ExprToken::Concat => {
                    let rhs = Box::new(self.parse_expression()?);
                    expr = Expr::Concat {
                        lhs: Box::new(expr),
                        rhs,
                    };
                }

                ExprToken::OpenBracket => {
                    expr = self.parse_expression()?;
                }

                ExprToken::CloseBracket => {
                    break 'parse;
                }

                ExprToken::Unknown => {
                    err!("invalid expression - {} expression token", tok);
                }
            }

            // Check if next token is ( when this function call is not for )
            // which implies that this function instance is not the instance
            // responsible for handling the close bracket.
            if !in_bracket {
                if let Some(ExprToken::CloseBracket) = &self.next_tok {
                    break 'parse;
                }
            }
        }

        Ok(expr)
    }

    fn parse_neg(&mut self) -> Result<Expr> {
        // Make error calling prettier
        macro_rules! err {
            ($($arg:tt)*) => {
                Err(self.error(format!($($arg)*)))
            };
        }

        if let Some(tok) = self.next_token() {
            match tok {
                ExprToken::Variable { ident } => {
                    let val = Expr::Variable { ident };
                    Ok(Expr::Neg {
                        expr: Box::new(val),
                    })
                }
                ExprToken::Const { val } => {
                    let val = Expr::Const { val };
                    Ok(Expr::Neg {
                        expr: Box::new(val),
                    })
                }
                ExprToken::OpenBracket => {
                    let expr = self.parse_expression()?;
                    Ok(Expr::Neg {
                        expr: Box::new(expr),
                    })
                }
                _ => err!("invalid expression, when parsing negative"),
            }
        } else {
            err!("invalid expression, when parsing negative")
        }
    }

    fn parse_not(&mut self) -> Result<Expr> {
        // Make error calling prettier
        macro_rules! err {
            ($($arg:tt)*) => {
                Err(self.error(format!($($arg)*)))
            };
        }

        if let Some(tok) = self.next_token() {
            match tok {
                ExprToken::Variable { ident } => {
                    let val = Expr::Variable { ident };
                    Ok(Expr::Not {
                        expr: Box::new(val),
                    })
                }
                ExprToken::Const { val } => {
                    let val = Expr::Const { val };
                    Ok(Expr::Not {
                        expr: Box::new(val),
                    })
                }
                ExprToken::OpenBracket => {
                    let expr = self.parse_expression()?;
                    Ok(Expr::Not {
                        expr: Box::new(expr),
                    })
                }
                _ => err!("invalid expression, when parsing logical not"),
            }
        } else {
            err!("invalid expression, when parsing logical not")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        // Wanted expression tree
        let cnst = Expr::Const {
            val: String::from("4"),
        };
        let var = Expr::Variable {
            ident: String::from("myvar"),
        };
        let neg = Expr::Neg {
            expr: Box::new(cnst.clone()),
        };
        let not = Expr::Not {
            expr: Box::new(var.clone()),
        };
        let preinc = Expr::Inc {
            expr: Box::new(var.clone()),
        };
        let postinc = Expr::Inc {
            expr: Box::new(var.clone()),
        };
        let predec = Expr::Dec {
            expr: Box::new(var.clone()),
        };
        let postdec = Expr::Dec {
            expr: Box::new(var.clone()),
        };
        let add = Expr::Add {
            lhs: Box::new(cnst.clone()),
            rhs: Box::new(var.clone()),
        };
        let mul = Expr::Mul {
            lhs: Box::new(neg.clone()),
            rhs: Box::new(preinc.clone()),
        };
        let sub = Expr::Sub {
            lhs: Box::new(add.clone()),
            rhs: Box::new(mul.clone()),
        };
        let div = Expr::Div {
            lhs: Box::new(predec.clone()),
            rhs: Box::new(postdec.clone()),
        };
        let mod_ = Expr::Add {
            lhs: Box::new(postinc.clone()),
            rhs: Box::new(div.clone()),
        };
        let concat = Expr::Concat {
            lhs: Box::new(sub.clone()),
            rhs: Box::new(mod_.clone()),
        };
        let neq = Expr::Neq {
            lhs: Box::new(concat.clone()),
            rhs: Box::new(not.clone()),
        };

        // Test expression string
        let expr_str = String::from(
            "(((4 + $myvar) - -4 * $myvar++) . ($myvar+++$myvar--/$myvar--)) != !$myvar",
        );
        let mut efac = ExpressionFactory::new::<VecDeque<char>>(expr_str.chars().collect());

        let expr = efac.parse().unwrap();
        assert_eq!(expr, neq);
    }
}
