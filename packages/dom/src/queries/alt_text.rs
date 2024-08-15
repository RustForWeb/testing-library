use regex::Regex;
use web_sys::{Element, HtmlElement};

use crate::{
    error::QueryError,
    query_all_by_attribute,
    types::{Matcher, MatcherOptions},
};

pub fn query_all_by_alt_text(
    container: HtmlElement,
    alt: &Matcher,
    options: MatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    // check_container_type(container);

    let valid_tag_regex = Regex::new(r"^(img|input|area|.+-.+)$").expect("Regex should be valid.");

    Ok(
        query_all_by_attribute("alt".to_string(), container, alt, options)?
            .into_iter()
            .filter(|node| valid_tag_regex.is_match(&node.tag_name()))
            .collect(),
    )
}

fn _get_multiple_error(_c: Option<Element>, alt: String) -> String {
    format!("Found multiple elements with alt text: {alt}")
}

fn _get_missing_error(_c: Option<Element>, alt: String) -> String {
    format!("Unable to find an element with alt text: {alt}")
}
