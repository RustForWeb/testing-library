use regex::Regex;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlInputElement, HtmlLabelElement, Node};

use crate::util::node_list_to_vec;

const LABELLED_NODE_NAMES: [&str; 7] = [
    "button", "meter", "output", "progress", "select", "textarea", "input",
];

fn get_text_content(node: &Node) -> Option<String> {
    if LABELLED_NODE_NAMES.contains(&node.node_name().to_lowercase().as_str()) {
        Some("".into())
    } else if node.node_type() == Node::TEXT_NODE {
        node.text_content()
    } else {
        Some(
            node_list_to_vec::<Node>(node.child_nodes())
                .iter()
                .filter_map(get_text_content)
                .collect::<Vec<_>>()
                .join(""),
        )
    }
}

pub fn get_label_content(element: &Element) -> Option<String> {
    if element.tag_name().to_lowercase() == "label" {
        get_text_content(element)
    } else {
        element
            .dyn_ref::<HtmlInputElement>()
            .map(|input_element| input_element.value())
            .or(element.text_content())
    }
}

pub fn get_real_labels(element: &HtmlElement) -> Vec<HtmlLabelElement> {
    if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
        return input
            .labels()
            .map(node_list_to_vec::<HtmlLabelElement>)
            .unwrap_or_default();
    }

    if !is_labelable(element) {
        return vec![];
    }

    element
        .owner_document()
        .expect("Element should have owner document.")
        .query_selector_all("label")
        .map(node_list_to_vec::<HtmlLabelElement>)
        .unwrap_or_default()
        .into_iter()
        .filter(|label| label.control().is_some_and(|control| control == *element))
        .collect()
}

fn is_labelable(element: &Element) -> bool {
    Regex::new(r"BUTTON|METER|OUTPUT|PROGRESS|SELECT|TEXTAREA")
        .expect("Regex should be valid.")
        .is_match(&element.tag_name())
        || element.tag_name() == "INPUT" && element.get_attribute("type") != Some("hidden".into())
}

#[derive(Clone, Debug)]
pub struct Label {
    pub content: Option<String>,
    pub form_control: Option<HtmlElement>,
}

pub fn get_labels(
    container: &HtmlElement,
    element: &HtmlElement,
    selector: Option<String>,
) -> Vec<Label> {
    let selector = selector.unwrap_or("*".into());

    let aria_labelled_by = element.get_attribute("aria-labelledby");
    let labels_id = aria_labelled_by
        .as_ref()
        .map(|aria_labelled_by| aria_labelled_by.split(' ').collect::<Vec<_>>())
        .unwrap_or_default();

    if labels_id.is_empty() {
        get_real_labels(element)
            .into_iter()
            .map(|label| {
                let text_to_match = get_label_content(&label);
                let labelled_form_control = label
                    .query_selector_all("button, input, meter, output, progress, select, textarea")
                    .ok()
                    .and_then(|form_control_elements| {
                        node_list_to_vec::<HtmlElement>(form_control_elements)
                            .into_iter()
                            .find(|form_control_element| {
                                form_control_element.matches(&selector).unwrap_or(false)
                            })
                    });

                Label {
                    content: text_to_match,
                    form_control: labelled_form_control,
                }
            })
            .collect()
    } else {
        labels_id
            .into_iter()
            .map(|label_id| {
                container
                    .query_selector(&format!("[id=\"{label_id}\""))
                    .map(|labelling_element| Label {
                        content: labelling_element
                            .and_then(|labelling_element| get_label_content(&labelling_element)),
                        form_control: None,
                    })
                    .unwrap_or(Label {
                        content: Some("".into()),
                        form_control: None,
                    })
            })
            .collect()
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::{HtmlInputElement, HtmlLabelElement};

    use crate::helpers::get_document;

    use super::get_real_labels;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn hidden_inputs_are_not_labelable() {
        let element = get_document()
            .create_element("input")
            .expect("Element should be created.")
            .unchecked_into::<HtmlInputElement>();
        element.set_type("hidden");

        let expected: Vec<HtmlLabelElement> = vec![];
        assert_eq!(expected, get_real_labels(&element));
    }
}
