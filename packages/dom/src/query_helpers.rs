use web_sys::HtmlElement;

use crate::{
    error::QueryError,
    matches::{fuzzy_matches, make_normalizer, matches},
    util::node_list_to_vec,
    Matcher, MatcherOptions, NormalizerOptions,
};

pub fn query_all_by_attribute(
    attribute: String,
    container: HtmlElement,
    text: &Matcher,
    MatcherOptions {
        exact,
        trim,
        collapse_whitespace,
        normalizer,
        ..
    }: MatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    let exact = exact.unwrap_or(true);

    let matcher = match exact {
        true => matches,
        false => fuzzy_matches,
    };
    let match_normalizer = make_normalizer(NormalizerOptions {
        trim,
        collapse_whitespace,
        normalizer,
    })?;

    Ok(node_list_to_vec::<HtmlElement>(
        container
            .query_selector_all(&format!("[{attribute}]"))
            .map_err(QueryError::JsError)?,
    )
    .into_iter()
    .filter(|node| {
        matcher(
            node.get_attribute(&attribute),
            Some(node),
            text,
            match_normalizer.as_ref(),
        )
    })
    .collect())
}

pub fn query_by_attribute(
    attribute: String,
    container: HtmlElement,
    text: &Matcher,
    options: MatcherOptions,
) -> Result<Option<HtmlElement>, QueryError> {
    let mut els = query_all_by_attribute(attribute.clone(), container, text, options)?;
    if els.len() > 1 {
        Err(QueryError::MultipleElements(format!(
            "Found multiple elements by [{attribute}={text}]"
        )))
    } else {
        Ok(els.pop())
    }
}
