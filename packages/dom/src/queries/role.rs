use web_sys::HtmlElement;

use crate::{
    build_queries,
    error::QueryError,
    types::{ByRoleOptions, Matcher},
};

pub fn _query_all_by_role<M: Into<Matcher>>(
    _container: &HtmlElement,
    _alt: M,
    options: ByRoleOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    if options.selected.is_some() {
        // TODO
    }

    Ok(vec![])
}

fn get_multiple_error(_container: &HtmlElement, alt: Matcher) -> Result<String, QueryError> {
    Ok(format!("Found multiple elements with the alt text: {alt}"))
}

fn get_missing_error(
    _container: &HtmlElement,
    alt: Matcher,
    _options: ByRoleOptions,
) -> Result<String, QueryError> {
    Ok(format!(
        "Unable to find an element with the alt text: {alt}"
    ))
}

build_queries!(
    _query_all_by_role,
    get_multiple_error,
    get_missing_error,
    role,
    crate::types::ByRoleOptions
);

pub use internal::{
    find_all_by_role, find_by_role, get_all_by_role, get_by_role, query_all_by_role, query_by_role,
};
