
use anyhow::{Context, Result};
use regex::{Captures, Regex};
use std::error;
use std::fmt;

pub fn regex_captures<'a>(regex: &Regex, string: &'a str) -> Result<Captures<'a>> {
    regex.captures(string).with_context(|| format!("`{}` did not match `{}`", string, regex.as_str()))
}

pub fn capture_group<'a>(captures: &'a Captures, group: usize) -> &'a str {
    captures.get(group)
        .ok_or_else(|| format!("Invalid capture group {} for {:?}", group, captures)).unwrap().as_str()
}

#[derive(Debug)]
pub struct ParsingError {
    msg: String,
}

impl ParsingError {
    pub fn new(msg: String) -> ParsingError {
        ParsingError { msg }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for ParsingError {}
