use web_sys::HtmlElement;

use crate::{
    MatcherOptions,
    types::{Method, Suggestion, Variant},
};

fn make_suggestion(
    query_name: Method,
    _element: &HtmlElement,
    content: String,
    variant: Variant,
    _name: Option<String>,
) -> Suggestion {
    let warning = None;

    let query_matcher = content.into();
    let query_options = MatcherOptions::default();

    // if let Some(name) = name {
    // query_options.name =
    // }

    // if query_name == Method::Role && is_inaccessible(element) {
    // query_options.
    // warning = Some();
    // }

    let query_method = format!("{variant}_by_{query_name}");

    Suggestion {
        query_name,
        query_method,
        query_matcher,
        query_options,
        variant,
        warning,
    }
}

fn can_suggest<T>(
    current_method: Method,
    requested_method: Option<Method>,
    data: Option<T>,
) -> Option<T> {
    if requested_method.is_none()
        || requested_method.is_some_and(|requested_method| requested_method == current_method)
    {
        data
    } else {
        None
    }
}

pub fn get_suggested_query(
    element: &HtmlElement,
    variant: Option<Variant>,
    method: Option<Method>,
) -> Option<Suggestion> {
    let variant = variant.unwrap_or(Variant::Get);

    // TODO

    let alt = element.get_attribute("alt");
    if let Some(alt) = can_suggest(Method::AltText, method, alt) {
        return Some(make_suggestion(
            Method::AltText,
            element,
            alt,
            variant,
            None,
        ));
    }

    None
}
