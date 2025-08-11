use std::{collections::HashMap, sync::LazyLock};

use aria_query::{
    AriaRoleDefinitionKey, AriaRoleRelationConcept, AriaRoleRelationConceptAttributeConstraint,
    ELEMENT_ROLES,
};
use dom_accessibility_api::{
    ComputeTextAlternativeOptions, compute_accessible_description, compute_accessible_name,
};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlInputElement, HtmlOptionElement};

use crate::{pretty_dom, types::ByRoleOptionsCurrent, util::html_collection_to_vec};

struct ElementRole {
    r#match: Box<dyn Fn(&Element) -> bool + Send + Sync>,
    roles: Vec<AriaRoleDefinitionKey>,
    specificity: usize,
}

fn make_element_selector(element: AriaRoleRelationConcept) -> String {
    format!(
        "{}{}",
        element.name,
        element
            .attributes
            .unwrap_or_default()
            .into_iter()
            .map(|attribute| {
                let constraints = attribute.constraints.unwrap_or_default();
                let should_not_exist =
                    constraints.contains(&AriaRoleRelationConceptAttributeConstraint::Undefined);
                let should_be_non_empty =
                    constraints.contains(&AriaRoleRelationConceptAttributeConstraint::Set);

                if let Some(value) = attribute.value {
                    format!("[{}=\"{}\"]", attribute.name, value)
                } else if should_not_exist {
                    format!(":not([{}])", attribute.name)
                } else if should_be_non_empty {
                    format!("[{}]:not([{}=\"\"])", attribute.name, attribute.name)
                } else {
                    format!("[{}]", attribute.name)
                }
            })
            .collect::<Vec<_>>()
            .join("")
    )
}

fn get_selector_specificity(element: &AriaRoleRelationConcept) -> usize {
    element
        .attributes
        .as_ref()
        .map(|attributes| attributes.len())
        .unwrap_or(0)
}

static ELEMENT_ROLE_LIST: LazyLock<Vec<ElementRole>> = LazyLock::new(|| {
    let mut result = vec![];

    for (element, roles) in ELEMENT_ROLES.iter() {
        let mut attributes = element.attributes.clone().unwrap_or_default();

        // https://github.com/testing-library/dom-testing-library/issues/814
        let type_text_index = attributes.iter().position(|attribute| {
            attribute.name == "type"
                && attribute
                    .value
                    .as_ref()
                    .is_some_and(|value| value == "text")
        });

        if let Some(type_text_index) = type_text_index {
            attributes.splice(type_text_index..type_text_index + 1, []);
        }

        let selector = make_element_selector(AriaRoleRelationConcept {
            name: element.name.clone(),
            attributes: Some(attributes),
            constraints: element.constraints.clone(),
        });

        result.push(ElementRole {
            r#match: Box::new(move |element| {
                if type_text_index.is_some()
                    && let Some(input_element) = element.dyn_ref::<HtmlInputElement>()
                    && input_element.type_() != "text"
                {
                    return false;
                }

                element.matches(&selector).unwrap_or(false)
            }),
            roles: roles.clone(),
            specificity: get_selector_specificity(element),
        });
    }

    result.sort_by(|left, right| right.specificity.cmp(&left.specificity));

    result
});

pub fn is_subtree_inaccessible(element: &Element) -> bool {
    if element
        .dyn_ref::<HtmlElement>()
        .is_some_and(|html_element| html_element.hidden())
    {
        return true;
    }

    if element.get_attribute("aria-hidden") == Some("true".into()) {
        return true;
    }

    let window = element
        .owner_document()
        .expect("Element should have owner document.")
        .default_view()
        .expect("Owner document should have default view.");

    if window
        .get_computed_style(element)
        .expect("Element should be valid.")
        .expect("Computed style should exist.")
        .get_property_value("display")
        .expect("Computed style should have display.")
        == "none"
    {
        return true;
    }

    false
}

// Partial implementation https://www.w3.org/TR/wai-aria-1.2/#tree_exclusion
// which should only be used for elements with a non-presentational role i.e.
// `role="none"` and `role="presentation"` will not be excluded.
//
// Implements aria-hidden semantics (i.e. parent overrides child)
// Ignores "Child Presentational: True" characteristics.
pub fn is_inaccessible(element: &Element) -> bool {
    let window = element
        .owner_document()
        .expect("Element should have owner document.")
        .default_view()
        .expect("Owner document should have default view.");

    // Since visibility is inherited we can exit early.
    if window
        .get_computed_style(element)
        .expect("Element should be valid.")
        .expect("Computed style should exist.")
        .get_property_value("visibility")
        .expect("Computed style should have visibility.")
        == "hidden"
    {
        return true;
    }

    let mut current_element = Some(element.clone());
    while let Some(element) = current_element.as_ref() {
        if is_subtree_inaccessible(element) {
            return true;
        }

        current_element = element.parent_element();
    }

    false
}

pub fn get_implicit_aria_roles(current_node: &Element) -> Vec<AriaRoleDefinitionKey> {
    for element_role in ELEMENT_ROLE_LIST.iter() {
        if (element_role.r#match)(current_node) {
            return element_role.roles.clone();
        }
    }

    vec![]
}

#[derive(Clone, Default)]
pub struct GetRolesOptions {
    pub hidden: Option<bool>,
}

pub fn get_roles(
    container: Element,
    options: GetRolesOptions,
) -> HashMap<AriaRoleDefinitionKey, Vec<Element>> {
    fn flatten_dom(element: Element) -> Vec<Element> {
        let mut elements = vec![element.clone()];
        elements.extend(
            html_collection_to_vec::<Element>(element.children())
                .into_iter()
                .flat_map(flatten_dom)
                .collect::<Vec<_>>(),
        );
        elements
    }

    let hidden = options.hidden.unwrap_or(false);

    flatten_dom(container)
        .into_iter()
        .filter(|element| hidden || !is_inaccessible(element))
        .fold(HashMap::new(), |mut acc, element| {
            // TODO: This violates html-aria which does not allow any role on every element.
            let roles = if element.has_attribute("role") {
                element
                    .get_attribute("role")
                    .expect("Attribute should exist.")
                    .split(' ')
                    .filter_map(|role| role.parse::<AriaRoleDefinitionKey>().ok())
                    .take(1)
                    .collect::<Vec<_>>()
            } else {
                get_implicit_aria_roles(&element)
            };

            for role in roles {
                acc.entry(role)
                    .and_modify(|entry| entry.push(element.clone()))
                    .or_insert_with(|| vec![element.clone()]);
            }

            acc
        })
}

#[derive(Clone, Default)]
pub struct PrettyRolesOptions {
    pub hidden: Option<bool>,
    pub include_description: Option<bool>,
}

fn pretty_roles(dom: Element, options: PrettyRolesOptions) -> String {
    let roles = get_roles(
        dom,
        GetRolesOptions {
            hidden: options.hidden,
        },
    );

    roles
        .into_iter()
        // We prefer to skip generic role, we don't recommend it.
        .filter(|(role, _)| *role != AriaRoleDefinitionKey::Generic)
        .map(|(role, elements)| {
            let delimiter_bar = "-".repeat(50);
            let elements_string = elements
                .iter()
                .map(|element| {
                    let name_string = format!(
                        "Name \"{}\":\n",
                        compute_accessible_name(element, ComputeTextAlternativeOptions::default())
                    );

                    let dom_string = pretty_dom(
                        Some(
                            element
                                .clone_node_with_deep(false)
                                .expect("Node should be cloned.")
                                .dyn_into::<Element>()
                                .expect("Cloned node should be an Element.")
                                .into(),
                        ),
                        None,
                    );

                    if options.include_description.unwrap_or(false) {
                        let description_string = format!(
                            "Description \"{}\":",
                            compute_accessible_description(
                                element,
                                ComputeTextAlternativeOptions::default()
                            )
                        );

                        format!("{name_string}{description_string}{dom_string}")
                    } else {
                        format!("{name_string}{dom_string}")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n\n");

            format!("{role}:\n\n{elements_string}\n\n{delimiter_bar}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn log_roles(dom: Element, options: PrettyRolesOptions) {
    log::info!("{}", pretty_roles(dom, options));
}

pub fn compute_aria_selected(element: &Element) -> Option<bool> {
    if element.tag_name() == "OPTION" {
        // Implicit value from HTML-AAM mappings: https://www.w3.org/TR/html-aam-1.0/#att-selected.
        Some(element.unchecked_ref::<HtmlOptionElement>().selected())
    } else {
        // Explicit value.
        check_boolean_attribute(element, "aria-selected")
    }
}

pub fn compute_aria_busy(element: &Element) -> bool {
    // https://www.w3.org/TR/wai-aria-1.1/#aria-busy
    element
        .get_attribute("aria-busy")
        .is_some_and(|value| value == "true")
}

pub fn compute_aria_checked(element: &Element) -> Option<bool> {
    if let Some(input_element) = element.dyn_ref::<HtmlInputElement>() {
        // Implicit value from HTML-AAM mappings:
        // https://www.w3.org/TR/html-aam-1.0/#att-indeterminate
        // https://www.w3.org/TR/html-aam-1.0/#att-checked
        if input_element.indeterminate() {
            None
        } else {
            Some(input_element.checked())
        }
    } else {
        // Explicit value.
        check_boolean_attribute(element, "aria-checked")
    }
}

pub fn compute_aria_pressed(element: &Element) -> Option<bool> {
    // https://www.w3.org/TR/wai-aria-1.1/#aria-pressed
    check_boolean_attribute(element, "aria-pressed")
}

pub fn compute_aria_current(element: &Element) -> ByRoleOptionsCurrent {
    // https://www.w3.org/TR/wai-aria-1.1/#aria-current

    check_boolean_attribute(element, "aria-current")
        .map(ByRoleOptionsCurrent::Bool)
        .or_else(|| {
            element
                .get_attribute("aria-current")
                .map(ByRoleOptionsCurrent::String)
        })
        .unwrap_or(ByRoleOptionsCurrent::Bool(false))
}

pub fn compute_aria_expanded(element: &Element) -> Option<bool> {
    // https://www.w3.org/TR/wai-aria-1.1/#aria-expanded
    check_boolean_attribute(element, "aria-expanded")
}

fn check_boolean_attribute(element: &Element, attribute: &str) -> Option<bool> {
    let attribute_value = element.get_attribute(attribute);

    if let Some(attribute_value) = attribute_value {
        if attribute_value == "true" {
            Some(true)
        } else if attribute_value == "false" {
            Some(false)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn compute_heading_level(element: &Element) -> Option<usize> {
    // Explicit value: https://www.w3.org/TR/wai-aria-1.2/#aria-level.
    element
        .get_attribute("aria-level")
        .and_then(|level| level.parse::<usize>().ok())
        .or_else(|| {
            // Implicit value: https://w3c.github.io/html-aam/#el-h1-h6.
            match element.tag_name().as_str() {
                "H1" => Some(1),
                "H2" => Some(2),
                "H3" => Some(3),
                "H4" => Some(4),
                "H5" => Some(5),
                "H6" => Some(6),
                _ => None,
            }
        })
}

pub fn compute_aria_value_now(element: &Element) -> Option<f64> {
    element
        .get_attribute("aria-valuenow")
        .and_then(|value_now| value_now.parse().ok())
}

pub fn compute_aria_value_max(element: &Element) -> Option<f64> {
    element
        .get_attribute("aria-valuemax")
        .and_then(|value_max| value_max.parse().ok())
}

pub fn compute_aria_value_min(element: &Element) -> Option<f64> {
    element
        .get_attribute("aria-valuemin")
        .and_then(|value_min| value_min.parse().ok())
}

pub fn compute_aria_value_text(element: &Element) -> Option<String> {
    element.get_attribute("aria-valuetext")
}
