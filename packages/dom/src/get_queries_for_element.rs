use web_sys::HtmlElement;

use crate::{
    ByRoleMatcher, ByRoleOptions,
    error::QueryError,
    queries::*,
    types::{Matcher, MatcherOptions, SelectorMatcherOptions, WaitForOptions},
};

pub fn get_queries_for_element(element: HtmlElement) -> BoundQueries {
    BoundQueries { element }
}

pub struct BoundQueries {
    element: HtmlElement,
}

macro_rules! queries_for_element {
    ($(($name:ident, $matcher_type:ty, $options_type:ty)),*,) => {
        paste::paste! {
            impl BoundQueries {
                $(pub async fn [< find_by_ $name >]<M: Into<$matcher_type>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                    wait_for_options: WaitForOptions,
                ) -> Result<HtmlElement, QueryError> {
                    [< find_by_ $name >](&self.element, matcher, options, wait_for_options).await
                })*

                $(pub async fn [< find_all_by_ $name >]<M: Into<$matcher_type>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                    wait_for_options: WaitForOptions,
                ) -> Result<Vec<HtmlElement>, QueryError> {
                    [< find_all_by_ $name >](&self.element, matcher, options, wait_for_options).await
                })*

                $(pub fn [< get_by_ $name >]<M: Into<$matcher_type>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                ) -> Result<HtmlElement, QueryError> {
                    [< get_by_ $name >](&self.element, matcher, options)
                })*

                $(pub fn [< get_all_by_ $name >]<M: Into<$matcher_type>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                ) -> Result<Vec<HtmlElement>, QueryError> {
                    [< get_all_by_ $name >](&self.element, matcher, options)
                })*

                $(pub fn [< query_by_ $name >]<M: Into<$matcher_type>>(
                    &self,
                    matcher: M,
                    options: $options_type,
                ) -> Result<Option<HtmlElement>, QueryError> {
                    [< query_by_ $name >](&self.element, matcher, options)
                })*

                $(pub fn [< query_all_by_ $name >]<M: Into<$matcher_type>>(
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
    (alt_text, Matcher, MatcherOptions),
    (display_value, Matcher, MatcherOptions),
    (label_text, Matcher, SelectorMatcherOptions),
    (placeholder_text, Matcher, MatcherOptions),
    (role, ByRoleMatcher, ByRoleOptions),
    (test_id, Matcher, MatcherOptions),
    (text, Matcher, SelectorMatcherOptions),
    (title, Matcher, MatcherOptions),
);
