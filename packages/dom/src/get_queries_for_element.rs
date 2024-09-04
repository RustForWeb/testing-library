use web_sys::HtmlElement;

use crate::{
    error::QueryError,
    get_by_placeholder_text, get_by_test_id, get_by_title,
    queries::{get_by_alt_text, get_by_display_value, query_by_alt_text, query_by_display_value},
    query_by_placeholder_text, query_by_test_id, query_by_title,
    types::{Matcher, MatcherOptions},
};

pub struct BoundFunctions {
    element: HtmlElement,
}

impl BoundFunctions {
    pub fn get_by_alt_text<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        get_by_alt_text(&self.element, matcher, options)
    }

    pub fn get_by_display_value<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        get_by_display_value(&self.element, matcher, options)
    }

    pub fn get_by_placeholder_text<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        get_by_placeholder_text(&self.element, matcher, options)
    }

    pub fn get_by_test_id<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        get_by_test_id(&self.element, matcher, options)
    }

    pub fn get_by_title<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        get_by_title(&self.element, matcher, options)
    }

    pub fn query_by_alt_text<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        query_by_alt_text(&self.element, matcher, options)
    }

    pub fn query_by_display_value<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        query_by_display_value(&self.element, matcher, options)
    }

    pub fn query_by_placeholder_text<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        query_by_placeholder_text(&self.element, matcher, options)
    }

    pub fn query_by_test_id<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        query_by_test_id(&self.element, matcher, options)
    }

    pub fn query_by_title<M: Into<Matcher>>(
        &self,
        matcher: M,
        options: MatcherOptions,
    ) -> Result<Option<HtmlElement>, QueryError> {
        query_by_title(&self.element, matcher, options)
    }
}

pub fn get_queries_for_element(element: HtmlElement) -> BoundFunctions {
    // TODO
    BoundFunctions { element }
}
