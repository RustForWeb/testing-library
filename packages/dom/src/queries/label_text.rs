use web_sys::HtmlElement;

use crate::{
    build_queries,
    error::QueryError,
    label_helpers::{get_labels, get_real_labels},
    matches::{fuzzy_matches, make_normalizer, matches},
    query_all_by_attribute,
    types::{Matcher, MatcherOptions, NormalizerOptions, SelectorMatcherOptions},
    util::node_list_to_vec,
};

pub fn _query_all_by_label_text<M: Into<Matcher>>(
    container: &HtmlElement,
    text: M,
    options: SelectorMatcherOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    let text = text.into();
    let selector = options.selector.unwrap_or("*".to_owned());
    let matcher = match options.exact.unwrap_or(true) {
        true => matches,
        false => fuzzy_matches,
    };
    let match_normalizer = make_normalizer(NormalizerOptions {
        trim: options.trim,
        collapse_whitespace: options.collapse_whitespace,
        normalizer: options.normalizer,
    })?;

    let mut matching_labelled_elements = node_list_to_vec::<HtmlElement>(
        container
            .query_selector_all("*")
            .map_err(QueryError::JsError)?,
    )
    .into_iter()
    .filter(|element| {
        !get_real_labels(element).is_empty() || element.has_attribute("aria-labelledby")
    })
    .fold(vec![], |mut labelled_elements, labelled_element| {
        let label_list = get_labels(container, &labelled_element, Some(selector.clone()));

        for label in &label_list {
            if let Some(form_control) = label.form_control.as_ref()
                && matcher(
                    label.content.clone(),
                    label.form_control.as_deref(),
                    &text,
                    match_normalizer.as_ref(),
                )
            {
                labelled_elements.push(form_control.clone());
            }
        }

        let labels_value = label_list
            .into_iter()
            .filter_map(|label| label.content)
            .collect::<Vec<_>>();

        if matcher(
            Some(labels_value.join(" ")),
            Some(labelled_element.as_ref()),
            &text,
            match_normalizer.as_ref(),
        ) {
            labelled_elements.push(labelled_element.clone());
        }

        if labels_value.len() > 1 {
            for (index, label_value) in labels_value.iter().enumerate() {
                if matcher(
                    Some(label_value.clone()),
                    Some(labelled_element.as_ref()),
                    &text,
                    match_normalizer.as_ref(),
                ) {
                    labelled_elements.push(labelled_element.clone());
                }

                let labels_filtered = labels_value
                    .clone()
                    .splice(index..index + 1, vec![])
                    .collect::<Vec<_>>();

                if labels_filtered.len() > 1
                    && matcher(
                        Some(labels_filtered.join(" ")),
                        Some(labelled_element.as_ref()),
                        &text,
                        match_normalizer.as_ref(),
                    )
                {
                    labelled_elements.push(labelled_element.clone());
                }
            }
        }

        labelled_elements
    });

    matching_labelled_elements.append(&mut query_all_by_attribute(
        "aria-label",
        container,
        text,
        MatcherOptions {
            exact: options.exact,
            normalizer: Some(match_normalizer),
            trim: None,
            collapse_whitespace: None,
            suggest: None,
        },
    )?);

    let mut unique_matching_labelled_elements = vec![];
    for element in matching_labelled_elements {
        if !element.matches(&selector).unwrap_or(false) {
            continue;
        }

        if !unique_matching_labelled_elements.contains(&element) {
            unique_matching_labelled_elements.push(element);
        }
    }

    Ok(unique_matching_labelled_elements)
}

// TODO: implement get_all_by_label_text override

fn get_multiple_error(
    _container: &HtmlElement,
    text: Matcher,
    _options: SelectorMatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!(
        "Found multiple elements with the label text: {text}"
    ))
}

fn get_missing_error(
    _container: &HtmlElement,
    text: Matcher,
    _options: SelectorMatcherOptions,
) -> Result<String, QueryError> {
    Ok(format!(
        // "Unable to find an element with the label text: {text}"
        "Unable to find a label with the text of: {text}"
    ))
}

build_queries!(
    _query_all_by_label_text,
    get_multiple_error,
    get_missing_error,
    label_text,
    crate::types::Matcher,
    crate::types::SelectorMatcherOptions
);

pub use internal::{
    find_all_by_label_text, find_by_label_text, get_all_by_label_text, get_by_label_text,
    query_all_by_label_text, query_by_label_text,
};
