use web_sys::HtmlElement;

use crate::{
    build_queries,
    error::QueryError,
    query_helpers::query_all_by_attribute,
    types::{Matcher, MatcherOptions},
};

pub fn _query_all_by_placeholder_text<M: Into<Matcher>>(
    container: &HtmlElement,
    text: M,
    options: MatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    query_all_by_attribute("placeholder", container, text, options)
}

fn get_multiple_error(
    _container: &HtmlElement,
    text: Matcher,
    _options: MatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!(
        "Found multiple elements with the placeholder text: {text}"
    ))
}

fn get_missing_error(
    _container: &HtmlElement,
    text: Matcher,
    _options: MatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!(
        "Unable to find an element with the placeholder text: {text}"
    ))
}

build_queries!(
    _query_all_by_placeholder_text,
    get_multiple_error,
    get_missing_error,
    placeholder_text,
    crate::types::Matcher,
    crate::types::MatcherOptions
);

pub use internal::{
    find_all_by_placeholder_text, find_by_placeholder_text, get_all_by_placeholder_text,
    get_by_placeholder_text, query_all_by_placeholder_text, query_by_placeholder_text,
};
