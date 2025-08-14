use std::{fmt::Display, rc::Rc};

use aria_query::AriaRole;
use regex::Regex;
use web_sys::Element;

pub type MatcherFunction = dyn Fn(String, Option<&Element>) -> bool;

#[derive(Clone)]
pub enum Matcher {
    Function(Rc<MatcherFunction>),
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
                Self::Function(_) => "MatcherFn".to_owned(),
                Self::Regex(regex) => regex.to_string(),
                Self::Number(n) => n.to_string(),
                Self::String(s) => s.clone(),
            }
        )
    }
}

impl From<Rc<MatcherFunction>> for Matcher {
    fn from(value: Rc<MatcherFunction>) -> Self {
        Self::Function(value)
    }
}

impl From<Regex> for Matcher {
    fn from(value: Regex) -> Self {
        Self::Regex(value)
    }
}

impl From<f64> for Matcher {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<&str> for Matcher {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for Matcher {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

pub type ByRoleMatcher = AriaRole;

pub type NormalizerFn = dyn Fn(String) -> String;

#[derive(Default)]
pub struct NormalizerOptions {
    pub trim: Option<bool>,
    pub collapse_whitespace: Option<bool>,
    pub normalizer: Option<Rc<NormalizerFn>>,
}

#[derive(Clone, Default)]
pub struct MatcherOptions {
    pub exact: Option<bool>,
    pub trim: Option<bool>,
    pub collapse_whitespace: Option<bool>,
    pub normalizer: Option<Rc<NormalizerFn>>,
    pub suggest: Option<bool>,
}

impl MatcherOptions {
    pub fn exact(mut self, value: bool) -> Self {
        self.exact = Some(value);
        self
    }

    pub fn trim(mut self, value: bool) -> Self {
        self.trim = Some(value);
        self
    }

    pub fn collapse_whitespace(mut self, value: bool) -> Self {
        self.collapse_whitespace = Some(value);
        self
    }

    pub fn normalizer(mut self, value: Rc<NormalizerFn>) -> Self {
        self.normalizer = Some(value);
        self
    }

    pub fn suggest(mut self, value: bool) -> Self {
        self.suggest = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ignore {
    False,
    String(String),
}

impl From<String> for Ignore {
    fn from(value: String) -> Self {
        Ignore::String(value)
    }
}

#[derive(Clone, Default)]
pub struct SelectorMatcherOptions {
    pub exact: Option<bool>,
    pub trim: Option<bool>,
    pub collapse_whitespace: Option<bool>,
    pub normalizer: Option<Rc<NormalizerFn>>,
    pub suggest: Option<bool>,
    pub selector: Option<String>,
    pub ignore: Option<Ignore>,
}

impl SelectorMatcherOptions {
    pub fn exact(mut self, value: bool) -> Self {
        self.exact = Some(value);
        self
    }

    pub fn trim(mut self, value: bool) -> Self {
        self.trim = Some(value);
        self
    }

    pub fn collapse_whitespace(mut self, value: bool) -> Self {
        self.collapse_whitespace = Some(value);
        self
    }

    pub fn normalizer(mut self, value: Rc<NormalizerFn>) -> Self {
        self.normalizer = Some(value);
        self
    }

    pub fn suggest(mut self, value: bool) -> Self {
        self.suggest = Some(value);
        self
    }

    pub fn selector(mut self, value: &str) -> Self {
        self.selector = Some(value.to_owned());
        self
    }

    pub fn ignore(mut self, value: Ignore) -> Self {
        self.ignore = Some(value);
        self
    }
}

#[derive(Default)]
pub struct DefaultNormalizerOptions {
    pub trim: Option<bool>,
    pub collapse_whitespace: Option<bool>,
}
