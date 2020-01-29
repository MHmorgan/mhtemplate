// Copyright 2020 Magnus Aa. Hirth. All rights reserved.
//
// The internal module `lex` implements the scanner of template strings, and
// defines all template lexemes. Any expression encountered are handled by the
// ExpressionFactory from the internal module `expr`.

use regex;

use crate::{Expr, ExpressionFactory, Result, TmplError};
use std::collections::VecDeque;
use std::iter::Iterator;

use regex::Regex;

/*******************************************************************************
 *                                                                             *
 *  Lexemes
 *                                                                             *
 *******************************************************************************/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lexeme {
    Text {
        text: String,
    },
    Comment {
        text: String,
    },
    Set {
        ident: String,
        val: Box<Expr>,
    },
    For {
        ident: String,
        expr: Box<Expr>,
    },
    ForCnt {
        ident: String,
        ident_i: String,
        expr: Box<Expr>,
    },
    Expr {
        expr: Box<Expr>,
    },
    Repeat {
        expr: Box<Expr>,
    },
    If {
        expr: Box<Expr>,
    },
    Elif {
        expr: Box<Expr>,
    },
    Raw,
    With,
    End,
    Else,
    Unknown,
}

impl Default for Lexeme {
    fn default() -> Self {
        Lexeme::Unknown
    }
}

/*******************************************************************************
 *                                                                             *
 *  Scanner
 *                                                                             *
 *******************************************************************************/

#[derive(Default, Clone)]
pub struct Scanner {
    current: String, // Current text buffer
    bytes: VecDeque<char>,
    lexemes: VecDeque<Lexeme>,

    trim_ws_start: bool, // Indicate if a text lexeme should strip whitespaces in front
    trim_ws_end: bool,   // Indicate if a text lexeme should strip whitespaces in back

    // Two-character lookback
    cur_ch: char,
    prev1: char,
    prev2: char,

    // Error info
    line: usize,
    col: usize,
}

impl Scanner {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<VecDeque<char>>,
    {
        Scanner {
            bytes: text.into(),
            line: 1,
            col: 0,
            ..Default::default()
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.bytes.pop_front();
        if let Some(ch) = ch {
            // Update lookbacks
            self.prev2 = self.prev1;
            self.prev1 = self.cur_ch;
            self.cur_ch = ch;
            // Update line & col counts
            match self.prev1 {
                '\n' => {
                    self.line += 1;
                    self.col = 0;
                }
                _ => self.col += 1,
            }
        }
        ch
    }

    // Add the self.current String as text lexeme to lexeme queue
    fn push_text(&mut self) {
        // Trim the text first if necessary
        let txt = match (self.trim_ws_start, self.trim_ws_end) {
            (true, true) => self.current.trim(),
            (false, true) => self.current.trim_end(),
            (true, false) => self.current.trim_start(),
            _ => self.current.as_ref(),
        };

        // If text is empty this is unnecessary
        if !txt.is_empty() {
            self.lexemes.push_back(Lexeme::Text {
                text: String::from(txt),
            });
        }

        self.current.clear();
        self.trim_ws_start = false;
        self.trim_ws_end = false;
    }

    /// Scan the entire source.
    pub fn scan_all(&mut self) -> Result<()> {
        // Iterate through all characters
        while let Some(ch) = self.next_char() {
            match ch {
                // Expression opening tag
                '{' if self.prev1 == '{' => {
                    self.current.pop();
                    self.scan_expression()?;
                }
                // Statement opening tag
                '%' if self.prev1 == '{' => {
                    self.current.pop();
                    self.scan_statement()?;
                }
                // Comment opening tag
                '#' if self.prev1 == '{' => {
                    self.current.pop();
                    self.scan_comment()?;
                }
                _ => self.current.push(ch),
            }
        }

        // Before returning make sure the last text lexeme is added
        if !self.current.is_empty() {
            let s = if self.trim_ws_start {
                self.current.trim_start()
            } else {
                self.current.as_ref()
            };
            self.lexemes.push_back(Lexeme::Text {
                text: String::from(s),
            });
        }

        Ok(())
    }

    fn handle_statement(&mut self) -> Result<()> {
        lazy_static! {
            static ref STMT_SET_RE: Regex =
                Regex::new(r"^set\s(?P<ident>\S+)\s(?P<expr>.+)").unwrap();
            static ref STMT_FOR_RE: Regex =
                Regex::new(r"^for\s(?P<ident>\S+)\sin\s(?P<expr>.+)").unwrap();
            static ref STMT_FORCNT_RE: Regex =
                Regex::new(r"^for\s(?P<ident_i>\S+),\s(?P<ident>\S+)\sin\s(?P<expr>.+)$").unwrap();
            static ref STMT_REPEAT_RE: Regex = Regex::new(r"^repeat\s(?P<expr>.+)$").unwrap();
            static ref STMT_IF_RE: Regex = Regex::new(r"^if\s(?P<expr>.+)$").unwrap();
            static ref STMT_ELIF_RE: Regex = Regex::new(r"^elif\s(?P<expr>.+)$").unwrap();
            static ref STMT_DEFINE_RE: Regex = Regex::new(r"^define\s(?P<ident>\S+)$").unwrap();
            static ref STMT_TEMPLATE_RE: Regex = Regex::new(r"^template\s(?P<ident>\S+)$").unwrap();
        }

        let txt = String::from(self.current.trim());
        self.current.clear();

        // With
        if &txt == "with" {
            self.lexemes.push_back(Lexeme::With);
            return Ok(());
        }

        // Set
        if let Some(m) = STMT_SET_RE.captures(&txt) {
            // Extract text from statement
            let expr = m.name("expr").unwrap().as_str();
            let ident = m.name("ident").unwrap().as_str();
            // Parse expression
            let expr = ExpressionFactory::new::<VecDeque<char>>(expr.chars().collect()).parse()?;
            // Add lexeme
            self.lexemes.push_back(Lexeme::Set {
                ident: String::from(ident),
                val: Box::new(expr),
            });
            return Ok(());
        }

        // For
        if let Some(m) = STMT_FOR_RE.captures(&txt) {
            // Extract text from statement
            let expr = m.name("expr").unwrap().as_str();
            let ident = m.name("ident").unwrap().as_str();
            // Parse expression
            let expr = ExpressionFactory::new::<VecDeque<char>>(expr.chars().collect()).parse()?;
            // Add lexeme
            self.lexemes.push_back(Lexeme::For {
                ident: String::from(ident),
                expr: Box::new(expr),
            });
            return Ok(());
        }

        // For count
        if let Some(m) = STMT_FORCNT_RE.captures(&txt) {
            // Extract text from statement
            let expr = m.name("expr").unwrap().as_str();
            let ident = m.name("ident").unwrap().as_str();
            let ident_i = m.name("ident_i").unwrap().as_str();
            // Parse expression
            let expr = ExpressionFactory::new::<VecDeque<char>>(expr.chars().collect()).parse()?;
            // Add lexeme
            self.lexemes.push_back(Lexeme::ForCnt {
                ident: String::from(ident),
                ident_i: String::from(ident_i),
                expr: Box::new(expr),
            });
            return Ok(());
        }

        // End
        if &txt == "end" {
            self.lexemes.push_back(Lexeme::End);
            return Ok(());
        }

        // Repeat
        if let Some(m) = STMT_REPEAT_RE.captures(&txt) {
            // Extract text from statement
            let expr = m.name("expr").unwrap().as_str();
            // Parse expression
            let expr = ExpressionFactory::new::<VecDeque<char>>(expr.chars().collect()).parse()?;
            // Add lexeme
            self.lexemes.push_back(Lexeme::Repeat {
                expr: Box::new(expr),
            });
            return Ok(());
        }

        // If
        if let Some(m) = STMT_IF_RE.captures(&txt) {
            // Extract text from statement
            let expr = m.name("expr").unwrap().as_str();
            // Parse expression
            let expr = ExpressionFactory::new::<VecDeque<char>>(expr.chars().collect()).parse()?;
            // Add lexeme
            self.lexemes.push_back(Lexeme::If {
                expr: Box::new(expr),
            });
            return Ok(());
        }

        // Elif
        if let Some(m) = STMT_ELIF_RE.captures(&txt) {
            // Extract text from statement
            let expr = m.name("expr").unwrap().as_str();
            // Parse expression
            let expr = ExpressionFactory::new::<VecDeque<char>>(expr.chars().collect()).parse()?;
            // Add lexeme
            self.lexemes.push_back(Lexeme::Elif {
                expr: Box::new(expr),
            });
            return Ok(());
        }

        // Else
        if &txt == "else" {
            self.lexemes.push_back(Lexeme::Else);
            return Ok(());
        }

        // Raw
        if &txt == "raw" {
            self.lexemes.push_back(Lexeme::Raw);
            return Ok(());
        }

        Err(self.error(format!("unknown statement: \"{}\"", txt)))
    }

    fn scan_statement(&mut self) -> Result<()> {
        // TODO handle raw statements in Scanner::scan_statement

        // Check if end of the preceding text should be trimmed
        if let Some('-') = self.bytes.front() {
            self.trim_ws_end = true;
            self.bytes.pop_front();
        }
        self.push_text();

        'scanning: while let Some(ch) = self.next_char() {
            match ch {
                // Statements are single-line
                '\n' => self.handle_statement()?,

                // Closing statement tag
                '}' if self.prev1 == '%' => {
                    self.current.pop();
                    // Check trimming of following text
                    if self.prev2 == '-' {
                        self.current.pop();
                        self.trim_ws_start = true;
                    }
                    self.handle_statement()?;
                    break 'scanning;
                }

                _ => self.current.push(ch),
            }
        }

        Ok(())
    }

    fn handle_expression(&mut self) -> Result<()> {
        // Get expression string
        let txt = String::from(self.current.trim());
        self.current.clear();

        // Parse & add expression
        let expr = ExpressionFactory::new::<VecDeque<char>>(txt.chars().collect()).parse()?;
        self.lexemes.push_back(Lexeme::Expr {
            expr: Box::new(expr),
        });
        Ok(())
    }

    fn scan_expression(&mut self) -> Result<()> {
        // Check if end of the preceding text should be trimmed
        if let Some('-') = self.bytes.front() {
            self.trim_ws_end = true;
            self.bytes.pop_front();
        }
        self.push_text();

        'scanning: while let Some(ch) = self.next_char() {
            match ch {
                // Ignore newlines in expressions
                '\n' => (),

                // Closing statement tag
                '}' if self.prev1 == '}' => {
                    self.current.pop();
                    // Check trimming of following text
                    if self.prev2 == '-' {
                        self.current.pop();
                        self.trim_ws_start = true;
                    }
                    self.handle_expression()?;
                    break 'scanning;
                }

                _ => self.current.push(ch),
            }
        }

        Ok(())
    }

    fn handle_comment(&mut self) -> Result<()> {
        // Get comment str
        let txt = String::from(self.current.trim());
        self.current.clear();

        self.lexemes.push_back(Lexeme::Comment { text: txt });
        Ok(())
    }

    fn scan_comment(&mut self) -> Result<()> {
        // Check if end of the preceding text should be trimmed
        if let Some('-') = self.bytes.front() {
            self.trim_ws_end = true;
            self.bytes.pop_front();
        }
        self.push_text();

        'scanning: while let Some(ch) = self.next_char() {
            match ch {
                // Closing statement tag
                '}' if self.prev1 == '#' => {
                    self.current.pop();
                    // Check trimming of following text
                    if self.prev2 == '-' {
                        self.current.pop();
                        self.trim_ws_start = true;
                    }
                    self.handle_comment()?;
                    break 'scanning;
                }

                // Ignore comment texts
                _ => self.current.push(ch),
            }
        }

        Ok(())
    }

    fn error<S: ToString>(&self, txt: S) -> TmplError {
        TmplError::new(format!(
            "at line {}, col {}: {}",
            self.line,
            self.col,
            txt.to_string()
        ))
    }
}

impl Iterator for Scanner {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexemes.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() {
        let text = String::from("{% set num 4
   set str Hello World -%}

{#- Hello comment -#}

{% with
   set num 6
   set str UGH %}
Ut enim ad minim {{ $str + ($myvar-- % 2) }}, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. 


{% if $myvar < ($num++) -%}
A conditional text.
{% end %}


Duis aute irure dolor in reprehenderit in voluptate velit {{ $num }} cillum dolore eu fugiat nulla pariatur.
{% end -%}

Excepteur sint occaecat cupidatat non {{ $str }}, sunt in culpa qui officia deserunt mollit anim id est {{ $num }}.
");

        let mut scn = Scanner::new::<VecDeque<char>>(text.chars().collect());
        scn.scan_all().unwrap();
    }
}
