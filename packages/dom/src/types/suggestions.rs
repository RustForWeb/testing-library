use std::fmt::Display;

use crate::types::{Matcher, MatcherOptions};

pub struct Suggestion {
    pub query_name: Method,
    pub query_method: String,
    pub query_matcher: Matcher,
    pub query_options: MatcherOptions,
    pub variant: Variant,
    pub warning: Option<String>,
}

impl Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(warning) = self.warning.as_ref() {
            log::warn!("{warning}");
        }

        let text = match &self.query_matcher {
            Matcher::String(matcher) => format!("'{matcher}'"),
            matcher => format!("{}", matcher),
        };

        let options = ", TODO";

        write!(f, "{}({}{})", self.query_method, text, options)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Variant {
    Find,
    FindAll,
    Get,
    GetAll,
    Query,
    QueryAll,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Variant::Find => "find",
                Variant::FindAll => "find_all",
                Variant::Get => "get",
                Variant::GetAll => "get_all",
                Variant::Query => "query",
                Variant::QueryAll => "query_all",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Method {
    AltText,
    DisplayValue,
    LabelText,
    PlaceholderText,
    Role,
    TestId,
    Text,
    Title,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Method::AltText => "alt_text",
                Method::DisplayValue => "display_value",
                Method::LabelText => "label_text",
                Method::PlaceholderText => "placeholder_text",
                Method::Role => "role",
                Method::TestId => "test_id",
                Method::Text => "text",
                Method::Title => "title",
            }
        )
    }
}
