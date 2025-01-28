use std::collections::HashSet;

use aria_query::{AriaProperty, AriaRole, ROLES, ROLE_ELEMENTS};
use dom_accessibility_api::{
    compute_accessible_description, compute_accessible_name, ComputeTextAlternativeOptions,
};
use web_sys::HtmlElement;

use crate::{
    build_queries,
    config::get_config,
    error::QueryError,
    matches::matches,
    role_helpers::{
        compute_aria_busy, compute_aria_checked, compute_aria_current, compute_aria_expanded,
        compute_aria_pressed, compute_aria_selected, compute_aria_value_max,
        compute_aria_value_min, compute_aria_value_now, compute_aria_value_text,
        compute_heading_level, get_implicit_aria_roles, is_inaccessible,
    },
    types::{ByRoleMatcher, ByRoleOptions, Matcher},
    util::node_list_to_vec,
};

pub fn _query_all_by_role<M: Into<ByRoleMatcher>>(
    container: &HtmlElement,
    role: M,
    options: ByRoleOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    let role = role.into();
    let role_string = role.to_string();

    let hidden = options.hidden.unwrap_or(get_config().default_hidden);
    let name = options.name;
    let description = options.description;
    let query_fallbacks = options.query_fallbacks.unwrap_or(false);
    let selected = options.selected;
    let busy = options.busy;
    let checked = options.checked;
    let pressed = options.pressed;
    let current = options.current;
    let level = options.level;
    let expanded = options.expanded;
    let options_value = options.value.unwrap_or_default();
    let value_now = options_value.now;
    let value_min = options_value.min;
    let value_max = options_value.max;
    let value_text = options_value.text;

    // Guard against unknown roles.
    if selected.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaSelected))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-selected` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if busy.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaBusy))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-busy` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if checked.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaChecked))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-checked` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if pressed.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaPressed))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-pressed` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    // All currently released ARIA versions support `aria-current` on all roles.
    // Leaving this for symmetry and forward compatibility.
    if current.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaCurrent))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-current` is not supported on role \"{role}\"."
        )));
    }

    // Guard against using `level` option with any role other than `heading`.
    if level.is_some() && role != AriaRole::Heading {
        return Err(QueryError::Unsupported(format!(
            "Role \"{role}\" cannot have \"level\" property."
        )));
    }

    // Guard against unknown roles.
    if value_now.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaValuenow))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuenow` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if value_max.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaValuemax))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuemax` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if value_min.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaValuemin))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuemin` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if value_text.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaValuetext))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuetext` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if expanded.is_some()
        && !ROLES
            .get(&role.into())
            .is_some_and(|role| role.props.contains_key(&AriaProperty::AriaExpanded))
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-expanded` is not supported on role \"{role}\"."
        )));
    }

    Ok(node_list_to_vec::<HtmlElement>(
        container
            .query_selector_all(
                // Only query elements that can be matched by the following filters.
                &make_role_selector(role),
            )
            .map_err(QueryError::JsError)?,
    )
    .into_iter()
    .filter(|node| {
        if let Some(role_value) = node.get_attribute("role") {
            if query_fallbacks {
                return role_value
                    .split(' ')
                    .filter(|role_attribute_token| !role_attribute_token.is_empty())
                    .any(|role_attribute_token| role_attribute_token == role_string);
            }

            // Other wise only send the first token to match.
            return role_value
                .split(' ')
                .next()
                .is_some_and(|first_role_attribute_token| {
                    first_role_attribute_token == role_string
                });
        }

        let implicit_roles = get_implicit_aria_roles(node);

        implicit_roles
            .into_iter()
            .any(|implicit_role| implicit_role == role.into())
    })
    .filter(|element| {
        if selected.is_some() {
            return selected == compute_aria_selected(element);
        }
        if let Some(busy) = busy {
            return busy == compute_aria_busy(element);
        }
        if checked.is_some() {
            return checked == compute_aria_checked(element);
        }
        if pressed.is_some() {
            return pressed == compute_aria_pressed(element);
        }
        if let Some(current) = &current {
            return *current == compute_aria_current(element);
        }
        if expanded.is_some() {
            return expanded == compute_aria_expanded(element);
        }
        if level.is_some() {
            return level == compute_heading_level(element);
        }
        if value_now.is_some() || value_max.is_some() || value_min.is_some() || value_text.is_some()
        {
            let mut value_matches = true;

            if value_now.is_some() {
                value_matches = value_matches && value_now == compute_aria_value_now(element);
            }
            if value_max.is_some() {
                value_matches = value_matches && value_max == compute_aria_value_max(element);
            }
            if value_min.is_some() {
                value_matches = value_matches && value_min == compute_aria_value_min(element);
            }
            if let Some(value_text) = &value_text {
                let normalizer = |text| text;

                value_matches = value_matches
                    && matches(
                        compute_aria_value_text(element),
                        Some(element),
                        value_text,
                        &normalizer,
                    );
            }

            return value_matches;
        }

        // Don't care if ARIA attributes are unspecified.
        true
    })
    .filter(|element| {
        if let Some(name) = &name {
            let normalizer = |text| text;

            matches(
                Some(compute_accessible_name(
                    element,
                    ComputeTextAlternativeOptions::default(),
                )),
                Some(element),
                name,
                &normalizer,
            )
        } else {
            // Don't care
            true
        }
    })
    .filter(|element| {
        if let Some(description) = &description {
            let normalizer = |text| text;

            matches(
                Some(compute_accessible_description(
                    element,
                    ComputeTextAlternativeOptions::default(),
                )),
                Some(element),
                description,
                &normalizer,
            )
        } else {
            // Don't care
            true
        }
    })
    .filter(|element| hidden || !is_inaccessible(element))
    .collect())
}

fn make_role_selector(role: ByRoleMatcher) -> String {
    let explicit_role_selector = format!("*[role~=\"{role}\"]");

    let role_relations = ROLE_ELEMENTS.get(&role.into());
    let implicit_role_selectors = role_relations.map(|role_relations| {
        role_relations
            .iter()
            .map(|relation| relation.name.clone())
            .collect::<HashSet<String>>()
    });

    let mut selectors = vec![explicit_role_selector];

    if let Some(implicit_role_selectors) = implicit_role_selectors {
        selectors.extend(implicit_role_selectors);
    }

    selectors.join(",")
}

fn get_name_hint(name: Option<Matcher>) -> String {
    match name {
        Some(Matcher::String(name)) => format!(" and name \"{name}\""),
        Some(Matcher::Regex(name)) => format!(" and name `{}`", name),
        Some(Matcher::Number(name)) => format!(" and name `{}`", name),
        Some(Matcher::Function(_name)) => " and name `Fn`".into(),
        None => "".into(),
    }
}

fn get_multiple_error(
    _container: &HtmlElement,
    role: ByRoleMatcher,
    options: ByRoleOptions,
) -> Result<String, QueryError> {
    Ok(format!(
        "Found multiple elements with the role \"{}\"{}",
        role,
        get_name_hint(options.name)
    ))
}

fn get_missing_error(
    _container: &HtmlElement,
    role: ByRoleMatcher,
    options: ByRoleOptions,
) -> Result<String, QueryError> {
    let hidden = options.hidden.unwrap_or(get_config().default_hidden);

    let name_hint = "";
    let description_hint = "";
    let role_message = "";
    // TODO

    Ok(format!(
        "Unable to find an {}element with the role \"{}\"{}{}\n\n{}",
        match hidden {
            true => "",
            false => "accessible ",
        },
        role,
        name_hint,
        description_hint,
        role_message.trim()
    ))
}

build_queries!(
    _query_all_by_role,
    get_multiple_error,
    get_missing_error,
    role,
    crate::types::ByRoleMatcher,
    crate::types::ByRoleOptions
);

pub use internal::{
    find_all_by_role, find_by_role, get_all_by_role, get_by_role, query_all_by_role, query_by_role,
};
