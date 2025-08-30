use web_sys::{Element, HtmlElement};

use crate::{
    Matcher, MatcherOptions, NormalizerOptions,
    config::get_config,
    error::QueryError,
    matches::{fuzzy_matches, make_normalizer, matches},
    util::node_list_to_vec,
};

pub fn get_element_error(message: Option<String>, container: Element) -> QueryError {
    (get_config().get_element_error)(message, container)
}

pub fn get_multiple_elements_found_error(message: String, container: Element) -> QueryError {
    get_element_error(
        Some(format!(
            "{message}\n\n(If this is intentional, then use the `*_all_by_*` variant of the query (like `query_all_by_text`, `get_all_by_text`, or `find_all_by_text`))."
        )),
        container,
    )
}

pub fn query_all_by_attribute<M: Into<Matcher>>(
    attribute: &str,
    container: &HtmlElement,
    text: M,
    MatcherOptions {
        exact,
        trim,
        collapse_whitespace,
        normalizer,
        ..
    }: MatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    let text = text.into();
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
            node.get_attribute(attribute),
            Some(node),
            &text,
            match_normalizer.as_ref(),
        )
    })
    .collect())
}

pub fn query_by_attribute<M: Into<Matcher>>(
    attribute: &str,
    container: &HtmlElement,
    text: M,
    options: MatcherOptions,
) -> Result<Option<HtmlElement>, QueryError> {
    let text = text.into();

    let mut els = query_all_by_attribute(attribute, container, text.clone(), options)?;
    if els.len() > 1 {
        Err(get_multiple_elements_found_error(
            format!("Found multiple elements by [{attribute}={text}]"),
            container.clone().into(),
        ))
    } else {
        Ok(els.pop())
    }
}

pub fn get_suggestion_error(suggestion: String, container: Element) -> QueryError {
    (get_config().get_element_error)(
        Some(format!(
            "A better query is available, try this: {suggestion}",
        )),
        container,
    )
}

#[macro_export]
macro_rules! make_single_query {
    ($all_query:ident, $get_multiple_error:ident, $name:ident, $matcher_type:ty, $options_type:ty, $return_type:ty, $func:ident $(,$args:literal)*) => {
        pub fn $name<M: Into<$matcher_type>>(
            container: &HtmlElement,
            matcher: M,
            options: $options_type,
        ) -> Result<$return_type, QueryError> {
            let matcher = matcher.into();

            let mut els = $all_query(container, matcher.clone(), options.clone())?;
            if els.len() > 1 {
                let element_strings = els
                    .into_iter()
                    .map(|element| format!("{}", get_element_error(None, element.into())))
                    .collect::<Vec<_>>()
                    .join("\n\n");

                Err(get_multiple_elements_found_error(
                    format!(
                        "{}\n\nHere are the matching elements:\n\n{}",
                        $get_multiple_error(container, matcher, options)?,
                        element_strings
                    ),
                    container.clone().into(),
                ))
            } else {
                Ok(Vec::$func(&mut els $(,$args)*))
            }
        }
    };
}

#[macro_export]
macro_rules! make_get_all_query {
    ($all_query:ident, $get_missing_error:ident, $name:ident, $matcher_type:ty, $options_type:ty) => {
        pub fn $name<M: Into<$matcher_type>>(
            container: &HtmlElement,
            matcher: M,
            options: $options_type,
        ) -> Result<Vec<HtmlElement>, QueryError> {
            let matcher = matcher.into();

            let els = $all_query(container, matcher.clone(), options.clone())?;
            if els.is_empty() {
                return Err((get_config().get_element_error)(
                    Some($get_missing_error(container, matcher, options)?),
                    container.clone().into(),
                ));
            } else {
                Ok(els)
            }
        }
    };
}

#[macro_export]
macro_rules! make_find_query {
    ($getter:ident, $name:ident, $matcher_type:ty, $options_type:ty, $return_type:ty) => {
        pub async fn $name<M: Into<$matcher_type>>(
            container: &HtmlElement,
            matcher: M,
            options: $options_type,
            wait_for_options: WaitForOptions,
        ) -> Result<$return_type, WaitForError<QueryError>> {
            wait_for(
                {
                    let matcher = matcher.into();

                    move || $getter(&container, matcher.clone(), options.clone())

                    // TODO: Remove if not using async for `wait_for` callback.
                    // move || {
                    //     let matcher = matcher.clone();
                    //     let container = container.clone();
                    //     let options = options.clone();

                    //     Box::pin(
                    //         async move { $getter(&container, matcher.clone(), options.clone()) },
                    //     )
                    // }
                },
                wait_for_options.container(container.clone()),
            )
            .await
        }
    };
}

#[macro_export]
macro_rules! wrap_single_query_with_suggestion {
    ($query:ident, $query_by_all_name:expr, $variant:expr, $name:ident, $matcher_type:ty, $options_type:ty, $return_type:ty) => {
        pub fn $name<M: Into<$matcher_type>>(
            container: &HtmlElement,
            matcher: M,
            options: $options_type,
        ) -> Result<$return_type, QueryError> {
            let element = $query(container, matcher, options)?;
            let suggest = get_config().throw_suggestions;

            if let Some(element) = Option::<&HtmlElement>::from(&element) {
                if suggest {
                    let suggestion = get_suggested_query(element, Some($variant), None);
                    if let Some(suggestion) = suggestion {
                        if !$query_by_all_name.ends_with(&suggestion.query_name.to_string()) {
                            return Err(get_suggestion_error(
                                suggestion.to_string(),
                                container.clone().into(),
                            ));
                        }
                    }
                }
            }

            Ok(element)
        }
    };
}

#[macro_export]
macro_rules! wrap_all_by_query_with_suggestion {
    ($query:ident, $query_by_all_name:expr, $variant:expr, $name:ident, $matcher_type:ty, $options_type:ty) => {
        pub fn $name<M: Into<$matcher_type>>(
            container: &HtmlElement,
            matcher: M,
            options: $options_type,
        ) -> Result<Vec<HtmlElement>, QueryError> {
            let els = $query(container, matcher, options)?;
            let suggest = get_config().throw_suggestions;

            if !els.is_empty() && suggest {
                // TODO
            }

            Ok(els)
        }
    };
}

#[macro_export]
macro_rules! build_queries {
    ($query_by_all:ident, $get_multiple_error:ident, $get_missing_error:ident, $name:ident, $matcher_type:ty, $options_type:ty) => {
        paste::paste! {
            mod internal {
                use web_sys::HtmlElement;

                use $crate::{
                    config::get_config,
                    error::{QueryError, WaitForError},
                    query_helpers::{get_element_error, get_multiple_elements_found_error, get_suggestion_error},
                    types::{Variant, WaitForOptions},
                    suggestions::{get_suggested_query},
                    wait_for::{wait_for},
                };

                use super::{$query_by_all, $get_multiple_error, $get_missing_error};

                // Query all by
                $crate::wrap_all_by_query_with_suggestion!($query_by_all, stringify!($query_by_all), Variant::QueryAll, [<query_all_by_ $name>], $matcher_type, $options_type);

                // Internal query by
                $crate::make_single_query!($query_by_all, $get_multiple_error, [<_query_by_ $name>], $matcher_type, $options_type, Option<HtmlElement>, pop);

                // Query by
                $crate::wrap_single_query_with_suggestion!([<_query_by_ $name>], stringify!($query_by_all), Variant::Query, [<query_by_ $name>], $matcher_type, $options_type, Option<HtmlElement>);

                // Internal get all by
                $crate::make_get_all_query!($query_by_all, $get_missing_error, [<_get_all_by_ $name>], $matcher_type, $options_type);

                // Get all by
                $crate::wrap_all_by_query_with_suggestion!([<_get_all_by_ $name>], stringify!($query_by_all).replace("query", "get"), Variant::GetAll, [<get_all_by_ $name>], $matcher_type, $options_type);

                // Internal get by
                $crate::make_single_query!([<_get_all_by_ $name>], $get_multiple_error, [<_get_by_ $name>], $matcher_type, $options_type, HtmlElement, swap_remove, 0);

                // Get by
                $crate::wrap_single_query_with_suggestion!([<_get_by_ $name>], stringify!($query_by_all), Variant::Get, [<get_by_ $name>], $matcher_type, $options_type, HtmlElement);

                // Internal find all by
                $crate::wrap_all_by_query_with_suggestion!([<_get_all_by_ $name>], stringify!($query_by_all), Variant::FindAll, [<_find_all_by_ $name>], $matcher_type, $options_type);

                // Find all by
                $crate::make_find_query!([<_find_all_by_ $name>], [<find_all_by_ $name>], $matcher_type, $options_type, Vec<HtmlElement>);

                // Internal find by
                $crate::wrap_single_query_with_suggestion!([<_get_by_ $name>], stringify!($query_by_all), Variant::Find, [<_find_by_ $name>], $matcher_type, $options_type, HtmlElement);

                // Find by
                $crate::make_find_query!([<_find_by_ $name>], [<find_by_ $name>], $matcher_type, $options_type, HtmlElement);
            }
        }
    };
}
