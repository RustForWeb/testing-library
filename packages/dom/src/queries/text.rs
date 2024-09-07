use web_sys::HtmlElement;

use crate::{
    build_queries,
    config::get_config,
    error::QueryError,
    get_node_text::get_node_text,
    matches::{fuzzy_matches, make_normalizer, matches},
    types::{Ignore, Matcher, NormalizerOptions, SelectorMatcherOptions},
    util::node_list_to_vec,
};

pub fn _query_all_by_text<M: Into<Matcher>>(
    container: &HtmlElement,
    text: M,
    options: SelectorMatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    let text = text.into();
    let selector = options.selector.unwrap_or("*".into());
    let ignore = options.ignore.unwrap_or(get_config().default_ignore.into());
    let matcher = match options.exact.unwrap_or(true) {
        true => matches,
        false => fuzzy_matches,
    };
    let match_normalizer = make_normalizer(NormalizerOptions {
        trim: options.trim,
        collapse_whitespace: options.collapse_whitespace,
        normalizer: options.normalizer,
    })?;

    let mut base_array = vec![];
    if container.matches(&selector).map_err(QueryError::JsError)? {
        base_array.push(container.clone());
    }

    Ok(base_array
        .into_iter()
        .chain(node_list_to_vec::<HtmlElement>(
            container
                .query_selector_all(&selector)
                .map_err(QueryError::JsError)?,
        ))
        .filter(|node| match &ignore {
            Ignore::False => true,
            Ignore::String(ignore) => !node.matches(ignore).unwrap_or(false),
        })
        .filter(|node| {
            matcher(
                Some(get_node_text(node)),
                Some(node),
                &text,
                match_normalizer.as_ref(),
            )
        })
        .collect())
}

fn get_multiple_error(_container: &HtmlElement, text: Matcher) -> Result<String, QueryError> {
    Ok(format!("Found multiple elements with the text: {text}"))
}

fn get_missing_error(
    _container: &HtmlElement,
    text: Matcher,
    options: SelectorMatcherOptions,
) -> Result<String, QueryError> {
    let match_normalizer = make_normalizer(NormalizerOptions {
        trim: options.trim,
        collapse_whitespace: options.collapse_whitespace,
        normalizer: options.normalizer,
    })?;
    let text = text.to_string();
    let normalized_text = match_normalizer(text.clone());
    let is_normalized_different = normalized_text != text;

    let selector = options.selector.unwrap_or("*".into());
    let is_custom_selector = selector != "*";

    Ok(format!(
        "Unable to find an element with the text: {}{}. \
        This could be because the text is broken up by multiple elements. \
        In this case, you can provide a function for your text matcher to make your matcher more flexible.",
        match is_normalized_different {
            true => format!("{normalized_text} (normalized from '{text}')"),
            false => text,
        },
        match is_custom_selector {
            true => format!(", which matches selector '{selector}'"),
            false => "".into(),
        }
    ))
}

build_queries!(
    _query_all_by_text,
    get_multiple_error,
    get_missing_error,
    text,
    crate::types::SelectorMatcherOptions
);

pub use internal::{
    find_all_by_text, find_by_text, get_all_by_text, get_by_text, query_all_by_text, query_by_text,
};
