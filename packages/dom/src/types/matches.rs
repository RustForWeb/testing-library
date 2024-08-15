use std::fmt::Display;

use regex::Regex;
use web_sys::Element;

pub type MatcherFunction = dyn Fn(String, Option<&Element>) -> bool;

pub enum Matcher {
    Function(Box<MatcherFunction>),
    Regex(Regex),
    Number(f64),
    String(String),
}

impl Display for Matcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function(_) => "MatcherFn".to_string(),
                Self::Regex(regex) => regex.to_string(),
                Self::Number(n) => format!("\n{n}\""),
                Self::String(s) => format!("\"{s}\""),
            }
        )
    }
}

// pub enum ByRoleMatcher

pub type NormalizerFn = dyn Fn(String) -> String;

#[derive(Default)]
pub struct NormalizerOptions {
    pub trim: Option<bool>,
    pub collapse_whitespace: Option<bool>,
    pub normalizer: Option<Box<NormalizerFn>>,
}

#[derive(Default)]
pub struct MatcherOptions {
    pub exact: Option<bool>,
    pub trim: Option<bool>,
    pub collapse_whitespace: Option<bool>,
    pub normalizer: Option<Box<NormalizerFn>>,
    pub suggest: Option<bool>,
}

#[derive(Default)]
pub struct DefaultNormalizerOptions {
    pub trim: Option<bool>,
    pub collapse_whitespace: Option<bool>,
}
