use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, HtmlOptionElement, HtmlSelectElement};

use crate::{
    build_queries,
    error::QueryError,
    get_node_text::get_node_text,
    matches::{fuzzy_matches, make_normalizer, matches},
    types::{Matcher, MatcherOptions, NormalizerOptions},
    util::{html_collection_to_vec, node_list_to_vec},
};

pub fn _query_all_by_display_value<M: Into<Matcher>>(
    container: &HtmlElement,
    value: M,
    options: MatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    let value = value.into();
    let matcher = match options.exact.unwrap_or(true) {
        true => matches,
        false => fuzzy_matches,
    };
    let match_normalizer = make_normalizer(NormalizerOptions {
        trim: options.trim,
        collapse_whitespace: options.collapse_whitespace,
        normalizer: options.normalizer,
    })?;

    Ok(node_list_to_vec::<HtmlElement>(
        container
            .query_selector_all("input,textarea,select")
            .map_err(QueryError::JsError)?,
    )
    .into_iter()
    .filter(|node| {
        if node.tag_name() == "SELECT" {
            html_collection_to_vec::<HtmlOptionElement>(
                node.unchecked_ref::<HtmlSelectElement>().options().into(),
            )
            .into_iter()
            .filter(|option| option.selected())
            .any(|option_node| {
                matcher(
                    Some(get_node_text(&option_node)),
                    Some(&option_node),
                    &value,
                    match_normalizer.as_ref(),
                )
            })
        } else {
            matcher(
                Some(node.unchecked_ref::<HtmlInputElement>().value()),
                Some(node),
                &value,
                match_normalizer.as_ref(),
            )
        }
    })
    .collect())
}

fn get_multiple_error(
    _container: &HtmlElement,
    value: Matcher,
    _options: MatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!(
        "Found multiple elements with the display value: {value}"
    ))
}

fn get_missing_error(
    _container: &HtmlElement,
    value: Matcher,
    _options: MatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!(
        "Unable to find an element with the display value: {value}"
    ))
}

build_queries!(
    _query_all_by_display_value,
    get_multiple_error,
    get_missing_error,
    display_value,
    crate::types::Matcher,
    crate::types::MatcherOptions
);

pub use internal::{
    find_all_by_display_value, find_by_display_value, get_all_by_display_value,
    get_by_display_value, query_all_by_display_value, query_by_display_value,
};
