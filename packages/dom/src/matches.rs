use std::rc::Rc;

use regex::Regex;
use web_sys::Element;

use crate::{
    DefaultNormalizerOptions, Matcher, NormalizerFn, NormalizerOptions, error::QueryError,
};

pub fn fuzzy_matches(
    text_to_match: Option<String>,
    node: Option<&Element>,
    matcher: &Matcher,
    normalizer: &NormalizerFn,
) -> bool {
    if let Some(text_to_match) = text_to_match {
        let normalized_text = normalizer(text_to_match);

        match matcher {
            Matcher::Function(matcher) => matcher(normalized_text, node),
            Matcher::Regex(matcher) => match_regex(matcher, normalized_text),
            Matcher::Number(matcher) => normalized_text == matcher.to_string(),
            Matcher::String(matcher) => normalized_text.to_lowercase() == matcher.to_lowercase(),
        }
    } else {
        false
    }
}

pub fn matches(
    text_to_match: Option<String>,
    node: Option<&Element>,
    matcher: &Matcher,
    normalizer: &NormalizerFn,
) -> bool {
    if let Some(text_to_match) = text_to_match {
        let normalized_text = normalizer(text_to_match);

        match matcher {
            Matcher::Function(matcher) => matcher(normalized_text, node),
            Matcher::Regex(matcher) => match_regex(matcher, normalized_text),
            Matcher::Number(matcher) => normalized_text == matcher.to_string(),
            Matcher::String(matcher) => normalized_text == *matcher,
        }
    } else {
        false
    }
}

pub fn get_default_normalizer(
    DefaultNormalizerOptions {
        trim,
        collapse_whitespace,
    }: DefaultNormalizerOptions,
) -> Rc<NormalizerFn> {
    let trim = trim.unwrap_or(true);
    let collapse_whitespace = collapse_whitespace.unwrap_or(true);

    Rc::new(move |text| {
        let mut normalized_text = text;

        if trim {
            normalized_text = normalized_text.trim().to_string();
        }

        if collapse_whitespace {
            normalized_text = Regex::new(r"\s+")
                .expect("Regex should be valid.")
                .replace_all(&normalized_text, " ")
                .to_string();
        }

        normalized_text
    })
}

/// Constructs a normalizer to pass to matches functions.
pub fn make_normalizer(
    NormalizerOptions {
        trim,
        collapse_whitespace,
        normalizer,
    }: NormalizerOptions,
) -> Result<Rc<NormalizerFn>, QueryError> {
    if let Some(normalizer) = normalizer {
        if trim.is_some() || collapse_whitespace.is_some() {
            Err(QueryError::Configuration("\n\
                `trim` and `collapse_whitespace` are not supported with a normalizer. \n\
                If you want to use the default trim and `collapse_whitespace logic in your normalizer, \n\
                use `get_default_normalizer(DefaultNormalizerOptions {trim, collapse_whitespace})` and compose that into your normalizer.\
            ".to_owned()))
        } else {
            Ok(normalizer)
        }
    } else {
        // No custom normalizer specified. Just use default.
        Ok(get_default_normalizer(DefaultNormalizerOptions {
            trim,
            collapse_whitespace,
        }))
    }
}

fn match_regex(matcher: &Regex, text: String) -> bool {
    matcher.is_match(&text)
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use std::rc::Rc;

    use regex::Regex;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use crate::types::Matcher;

    use super::{fuzzy_matches, matches};

    wasm_bindgen_test_configure!(run_in_browser);

    fn normalizer(text: String) -> String {
        text
    }

    #[wasm_bindgen_test]
    fn matchers_accept_strings() {
        assert!(matches(
            Some("ABC".to_owned()),
            None,
            &Matcher::String("ABC".to_owned()),
            &normalizer,
        ));
        assert!(fuzzy_matches(
            Some("ABC".to_owned()),
            None,
            &Matcher::String("ABC".to_owned()),
            &normalizer,
        ));
    }

    #[wasm_bindgen_test]
    fn matchers_accept_regex() {
        assert!(matches(
            Some("ABC".to_owned()),
            None,
            &Matcher::Regex(Regex::new("ABC").expect("Regex should be valid.")),
            &normalizer,
        ));
        assert!(fuzzy_matches(
            Some("ABC".to_owned()),
            None,
            &Matcher::Regex(Regex::new("ABC").expect("Regex should be valid.")),
            &normalizer,
        ));
    }

    #[wasm_bindgen_test]
    fn matchers_accept_functions() {
        assert!(matches(
            Some("ABC".to_owned()),
            None,
            &Matcher::Function(Rc::new(|text, _| text == "ABC")),
            &normalizer,
        ));
        assert!(fuzzy_matches(
            Some("ABC".to_owned()),
            None,
            &Matcher::Function(Rc::new(|text, _| text == "ABC")),
            &normalizer,
        ));
    }

    #[wasm_bindgen_test]
    fn matchers_return_false_if_text_to_match_is_not_a_string() {
        assert!(!matches(
            None,
            None,
            &Matcher::String("ABC".to_owned()),
            &normalizer,
        ));
        assert!(!fuzzy_matches(
            None,
            None,
            &Matcher::String("ABC".to_owned()),
            &normalizer,
        ));
    }
}
