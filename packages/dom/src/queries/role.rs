use std::collections::HashSet;

use aria_query::{AriaProperty, AriaRole, ROLES, ROLE_ELEMENTS};
use web_sys::HtmlElement;

use crate::{
    build_queries,
    error::QueryError,
    get_config,
    types::{ByRoleMatcher, ByRoleOptions, ByRoleOptionsName},
    util::node_list_to_vec,
};

pub fn _query_all_by_role<M: Into<ByRoleMatcher>>(
    container: &HtmlElement,
    role: M,
    options: ByRoleOptions,
) -> Result<Vec<HtmlElement>, QueryError> {
    let role = role.into();
    let _hidden = options.hidden.unwrap_or(get_config().default_hidden);
    let _query_fallbacks = options.query_fallbacks.unwrap_or(false);
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
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaSelected)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-selected` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if busy.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaBusy)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-busy` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if checked.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaChecked)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-checked` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if pressed.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaPressed)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-pressed` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    // All currently released ARIA versions support `aria-current` on all roles.
    // Leaving this for symmetry and forward compatibility.
    if current.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaCurrent)
        })
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
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaValuenow)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuenow` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if value_max.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaValuemax)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuemax` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if value_min.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaValuemin)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuemin` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if value_text.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaValuetext)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuetext` is not supported on role \"{role}\"."
        )));
    }

    // Guard against unknown roles.
    if expanded.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaExpanded)
        })
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
    .filter(|_node| {
        // TODO

        false
    })
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

fn get_name_hint(name: Option<ByRoleOptionsName>) -> String {
    match name {
        Some(ByRoleOptionsName::String(name)) => format!(" and name \"{name}\""),
        Some(ByRoleOptionsName::Regex(name)) => format!(" and name `{}`", name),
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
