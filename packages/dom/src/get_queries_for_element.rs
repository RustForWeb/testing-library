use web_sys::HtmlElement;

use crate::{
    error::QueryError,
    queries::*,
    types::{Matcher, MatcherOptions, SelectorMatcherOptions, WaitForOptions},
};

pub fn get_queries_for_element(element: HtmlElement) -> BoundFunctions {
    BoundFunctions { element }
}

pub struct BoundFunctions {
    element: HtmlElement,
}

macro_rules! queries_for_element {
    ($(($name:ident, $options_type:ty)),*,) => {
        paste::paste! {
            impl BoundFunctions {
                $(pub fn [< find_by_ $name >]<M: Into<Matcher>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                    wait_for_options: WaitForOptions,
                ) -> Result<Option<HtmlElement>, QueryError> {
                    [< find_by_ $name >](&self.element, matcher, options, wait_for_options)
                })*

                $(pub fn [< find_all_by_ $name >]<M: Into<Matcher>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                    wait_for_options: WaitForOptions,
                ) -> Result<Vec<HtmlElement>, QueryError> {
                    [< find_all_by_ $name >](&self.element, matcher, options, wait_for_options)
                })*

                $(pub fn [< get_by_ $name >]<M: Into<Matcher>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                ) -> Result<Option<HtmlElement>, QueryError> {
                    [< get_by_ $name >](&self.element, matcher, options)
                })*

                $(pub fn [< get_all_by_ $name >]<M: Into<Matcher>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                ) -> Result<Vec<HtmlElement>, QueryError> {
                    [< get_all_by_ $name >](&self.element, matcher, options)
                })*

                $(pub fn [< query_by_ $name >]<M: Into<Matcher>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                ) -> Result<Option<HtmlElement>, QueryError> {
                    [< query_by_ $name >](&self.element, matcher, options)
                })*

                $(pub fn [< query_all_by_ $name >]<M: Into<Matcher>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                ) -> Result<Vec<HtmlElement>, QueryError> {
                    [< query_all_by_ $name >](&self.element, matcher, options)
                })*
            }
        }
    }
}

queries_for_element!(
    (alt_text, MatcherOptions),
    (display_value, MatcherOptions),
    (label_text, SelectorMatcherOptions),
    (placeholder_text, MatcherOptions),
    (test_id, MatcherOptions),
    (text, SelectorMatcherOptions),
    (title, MatcherOptions),
);
