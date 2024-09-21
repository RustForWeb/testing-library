use regex::Regex;
use web_sys::HtmlElement;

use crate::{
    build_queries,
    error::QueryError,
    query_helpers::query_all_by_attribute,
    types::{Matcher, MatcherOptions},
};

pub fn _query_all_by_alt_text<M: Into<Matcher>>(
    container: &HtmlElement,
    alt: M,
    options: MatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    let valid_tag_regex = Regex::new(r"^(img|input|area|.+-.+)$").expect("Regex should be valid.");

    Ok(
        query_all_by_attribute("alt".to_string(), container, alt, options)?
            .into_iter()
            .filter(|node| valid_tag_regex.is_match(&node.tag_name()))
            .collect(),
    )
}

fn get_multiple_error(
    _container: &HtmlElement,
    alt: Matcher,
    _options: MatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!("Found multiple elements with the alt text: {alt}"))
}

fn get_missing_error(
    _container: &HtmlElement,
    alt: Matcher,
    _options: MatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!(
        "Unable to find an element with the alt text: {alt}"
    ))
}

build_queries!(
    _query_all_by_alt_text,
    get_multiple_error,
    get_missing_error,
    alt_text,
    crate::types::Matcher,
    crate::types::MatcherOptions
);

pub use internal::{
    find_all_by_alt_text, find_by_alt_text, get_all_by_alt_text, get_by_alt_text,
    query_all_by_alt_text, query_by_alt_text,
};
