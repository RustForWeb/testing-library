use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::{
    build_queries,
    error::QueryError,
    get_node_text::get_node_text,
    matches::{fuzzy_matches, make_normalizer, matches},
    types::{Matcher, MatcherOptions, NormalizerOptions},
    util::node_list_to_vec,
};

fn is_svg_title(node: &HtmlElement) -> bool {
    node.tag_name().to_lowercase() == "title"
        && node
            .parent_node()
            .and_then(|parent_node| parent_node.dyn_into::<Element>().ok())
            .is_some_and(|parent_node| parent_node.tag_name().to_lowercase() == "svg")
}

pub(crate) fn _query_all_by_title<M: Into<Matcher>>(
    container: &HtmlElement,
    text: M,
    options: MatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    let text = text.into();
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
            .query_selector_all("[title], svg > title")
            .map_err(QueryError::JsError)?,
    )
    .into_iter()
    .filter(|node| {
        matcher(
            node.get_attribute("title"),
            Some(node),
            &text,
            match_normalizer.as_ref(),
        ) || (is_svg_title(node)
            && matcher(
                Some(get_node_text(node)),
                Some(node),
                &text,
                match_normalizer.as_ref(),
            ))
    })
    .collect())
}

fn get_multiple_error(
    _container: &HtmlElement,
    title: Matcher,
    _options: MatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!("Found multiple elements with the title: {title}"))
}

fn get_missing_error(
    _container: &HtmlElement,
    title: Matcher,
    _options: MatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!("Unable to find an element with the title: {title}"))
}

build_queries!(
    _query_all_by_title,
    get_multiple_error,
    get_missing_error,
    title,
    crate::types::Matcher,
    crate::types::MatcherOptions
);

pub use internal::{
    find_all_by_title, find_by_title, get_all_by_title, get_by_title, query_all_by_title,
    query_by_title,
};
