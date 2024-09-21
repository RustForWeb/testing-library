use aria_query::{AriaProperty, ROLES};
use web_sys::HtmlElement;

use crate::{
    build_queries,
    error::QueryError,
    get_config,
    types::{ByRoleMatcher, ByRoleOptions},
};

pub fn _query_all_by_role<M: Into<ByRoleMatcher>>(
    _container: &HtmlElement,
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

    if selected.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaSelected)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-selected` is not supported on role \"{role}\"."
        )));
    }

    if busy.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaBusy)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-busy` is not supported on role \"{role}\"."
        )));
    }

    if checked.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaChecked)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-checked` is not supported on role \"{role}\"."
        )));
    }

    if pressed.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaPressed)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-pressed` is not supported on role \"{role}\"."
        )));
    }

    if current.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaCurrent)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-current` is not supported on role \"{role}\"."
        )));
    }

    if level.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaLevel)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-level` is not supported on role \"{role}\"."
        )));
    }

    if value_now.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaValuenow)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuenow` is not supported on role \"{role}\"."
        )));
    }

    if value_max.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaValuemax)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuemax` is not supported on role \"{role}\"."
        )));
    }

    if value_min.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaValuemin)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuemin` is not supported on role \"{role}\"."
        )));
    }

    if value_text.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaValuetext)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-valuetext` is not supported on role \"{role}\"."
        )));
    }

    if expanded.is_some()
        && !ROLES.get(&role.into()).map_or(false, |role| {
            role.props.contains_key(&AriaProperty::AriaExpanded)
        })
    {
        return Err(QueryError::Unsupported(format!(
            "`aria-expanded` is not supported on role \"{role}\"."
        )));
    }

    // TODO

    Ok(vec![])
}

fn get_multiple_error(_container: &HtmlElement, role: ByRoleMatcher) -> Result<String, QueryError> {
    Ok(format!("Found multiple elements with the role: {role}"))
}

fn get_missing_error(
    _container: &HtmlElement,
    role: ByRoleMatcher,
    _options: ByRoleOptions,
) -> Result<String, QueryError> {
    Ok(format!("Unable to find an element with the role: {role}"))
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
