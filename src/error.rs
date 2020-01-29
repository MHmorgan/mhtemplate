// Copyright 2020 Magnus Aa. Hirth. All rights reserved.

use regex;

use std::error::Error;
use std::fmt;
use std::io;

/// mhtemplate specific Result, returning `TmplError`
pub type Result<T> = std::result::Result<T, TmplError>;

/// TmplError is the error returned by all methods of the mhtemplate crate
#[derive(Debug, Clone)]
pub struct TmplError {
    message: String,
}

impl TmplError {
    pub fn new<T: ToString>(msg: T) -> Self {
        TmplError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for TmplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TmplError {
    fn description(&self) -> &str {
        self.message.as_ref()
    }
}

// -----------------------------------------------------------------------------
// From

macro_rules! impl_error {
    ($type:ty) => {
        impl From<$type> for TmplError {
            fn from(err: $type) -> Self {
                TmplError {
                    message: err.to_string(),
                }
            }
        }
    };
}

impl_error!(std::num::ParseFloatError);
impl_error!(std::num::ParseIntError);
impl_error!(io::Error);
impl_error!(regex::Error);
