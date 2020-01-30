// Copyright 2020 Magnus Aa. Hirth. All rights reserved.
//
// The module `context` implements the `Context` type used to create scopes 
// during evaluation of templates.

use std::collections::{hash_map, HashMap};
use std::ops::{Index, IndexMut};

/// Evaluation context containing variable values.
///
/// A context can be created from the programs enviroment:
/// ```Rust
/// # use mhtemplate::Context;
/// let vars = std::env::vars();
/// let ctx  = Context::from(vars);
/// ```
///
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Context(HashMap<String, String>);

impl Context {
    pub fn new() -> Self {
        Context {
            ..Default::default()
        }
    }

    pub fn var(&self, name: &str) -> Option<&String> {
        self.0.get(name)
    }

    pub fn var_mut(&mut self, name: &str) -> Option<&mut String> {
        self.0.get_mut(name)
    }

    pub fn set_var(&mut self, name: String, val: String) -> Option<String> {
        self.0.insert(name, val)
    }

    pub fn iter_vars(&self) -> hash_map::Iter<'_, String, String> {
        self.0.iter()
    }

    pub fn iter_vars_mut(&mut self) -> hash_map::IterMut<'_, String, String> {
        self.0.iter_mut()
    }
}

impl From<std::env::Vars> for Context {

    /// ```
    /// # use mhtemplate::Context;
    /// let vars = std::env::vars();
    /// let ctx  = Context::from(vars);
    /// ```
    /// 
    fn from(vars: std::env::Vars) -> Self {
        let mut ctx = Context {
            ..Default::default()
        };
        for (name, val) in vars {
            ctx.set_var(name, val);
        }
        ctx
    }
}

impl From<HashMap<String, String>> for Context {

    /// ```
    /// # use mhtemplate::Context;
    /// # use std::collections::HashMap;
    /// let mut vars = HashMap::new();
    /// vars.insert(String::from("one"), String::from("1"));
    /// let mut ctx = Context::from(vars);
    /// ```
    /// 
    fn from(vars: HashMap<String, String>) -> Self {
        Context(vars.clone())
    }
}

impl From<&Context> for Context {
    fn from(ctx: &Context) -> Self {
        ctx.clone()
    }
}

impl Index<&str> for Context {
    type Output = String;

    fn index(&self, index: &str) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<&str> for Context {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        if !self.0.contains_key(index) {
            self.0.insert(String::from(index), String::new());
        }
        self.0.get_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let s1 = String::from("one");
        let i1 = String::from("1");
        let s2 = String::from("two");
        let i2 = String::from("2");

        let mut vars = HashMap::new();
        vars.insert(s1.clone(), i1.clone());
        let mut ctx = Context::from(vars);

        ctx[&s2] = i2.clone();
        assert_eq!(ctx[&s1], i1);
        assert_eq!(ctx[&s2], i2);
    }
}
