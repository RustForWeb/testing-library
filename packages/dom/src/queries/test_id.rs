use web_sys::HtmlElement;

use crate::{
    build_queries,
    error::QueryError,
    get_config, query_all_by_attribute,
    types::{Matcher, MatcherOptions},
};

fn get_test_id_attribute() -> String {
    get_config().test_id_attribute
}

pub fn _query_all_by_test_id<M: Into<Matcher>>(
    container: &HtmlElement,
    id: M,
    options: MatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    query_all_by_attribute(get_test_id_attribute(), container, id, options)
}

fn get_multiple_error(_container: &HtmlElement, id: Matcher) -> String {
    format!(
        "Found multiple elements by: [{}=\"{}\"]",
        get_test_id_attribute(),
        id
    )
}

fn get_missing_error(_container: &HtmlElement, id: Matcher) -> String {
    format!(
        "Unable to find an element by: [{}=\"{}\"]",
        get_test_id_attribute(),
        id
    )
}

build_queries!(
    _query_all_by_test_id,
    get_multiple_error,
    get_missing_error,
    test_id
);

pub use internal::{
    find_all_by_test_id, find_by_test_id, get_all_by_test_id, get_by_test_id, query_all_by_test_id,
    query_by_test_id,
};