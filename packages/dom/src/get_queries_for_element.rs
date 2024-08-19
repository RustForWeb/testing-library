use web_sys::HtmlElement;

use crate::{
    error::QueryError,
    queries::{get_by_alt_text, query_by_alt_text},
    types::{Matcher, MatcherOptions},
};

pub struct BoundFunctions {
    element: HtmlElement,
}

impl BoundFunctions {
    pub fn query_by_alt_text<M: Into<Matcher>>(
        &self,
        alt: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        query_by_alt_text(&self.element, alt, options)
    }

    pub fn get_by_alt_text<M: Into<Matcher>>(
        &self,
        alt: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        get_by_alt_text(&self.element, alt, options)
    }
}

pub fn get_queries_for_element(element: HtmlElement) -> BoundFunctions {
    // TODO
    BoundFunctions { element }
}
