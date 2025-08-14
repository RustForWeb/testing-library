# By Role

> `get_by_role`, `query_by_role`, `get_all_by_role`, `query_all_by_role`, `find_by_role`, `find_all_by_role`

## API

```rust,ignore
use aria_query::AriaRole;
use testing_library_dom::{Matcher, QueryError};
use web_sys::HtmlElement;

fn get_by_role(
    // If you're using `screen`, then skip the container argument:
    container: &HtmlElement,
    role: AriaRole,
    options: ByRoleOptions
) -> Result<Option<HtmlElement>, QueryError>;

struct ByRoleOptions {
    hidden: Option<bool>,
    name: Option<Matcher>,
    description: Option<Matcher>,
    selected: Option<bool>,
    busy: Option<bool>,
    checked: Option<bool>,
    pressed: Option<bool>,
    suggest: Option<bool>,
    current: Option<ByRoleOptionsCurrent>,
    expanded: Option<bool>,
    query_fallbacks: Option<bool>,
    level: Option<usize>,
    value: Option<ByRoleOptionsValue>,
}

enum ByRoleOptionsCurrent {
    Bool(bool),
    String(String),
}

struct ByRoleOptionsValue {
    now: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,
    text: Option<Matcher>,
}
```

Queries for elements with the given role <!-- (and it also accepts a TextMatch) -->. Default roles are taken into consideration e.g. `<button />` has the `button` role without explicitly setting the `role` attribute. Here you can see [a table of HTML elements with their default and desired roles](https://www.w3.org/TR/html-aria/#docconformance).

Please note that setting a `role` and/or `aria-*` attribute that matches the implicit ARIA semantics is unnecessary and is **not recommended** as these properties are already set by the browser, and we must not use the `role` and `aria-*` attributes in a manner that conflicts with the semantics described. For example, a `button` element can't have the `role` attribute of `heading`, because the `button` element has default characteristics that conflict with the `heading` role.

<!-- TODO: Ensure text still applies to `AriaRole`. -->

> Roles are matched literally by string equality, without inheriting from the ARIA role hierarchy. As a result, querying a superclass role like `checkbox` will not include elements with a subclass role like `switch`.

You can query the returned element(s) by their [accessible name or description](https://www.w3.org/TR/accname-1.1/). The accessible name is for simple cases equal to e.g. the label of a form element, or the text content of a button, or the value of the `aria-label` attribute. It can be used to query a specific element if multiple elements with the same role are present on the rendered content. For an in-depth guide check out ["What is an accessible name?" from TPGi](https://www.tpgi.com/what-is-an-accessible-name/). If you only query for a single element with `get_by_text("The name")` it's oftentimes better to use `get_by_role(expected_role, { name: "The name" })`. The accessible name query does not replace other queries such as `*_by_alt` or `*_by_title`. While the accessible name can be equal to these attributes, it does not replace the functionality of these attributes. For example `<img aria-label="fancy image" src="fancy.jpg" />` will be returned for `get_by_role("img", { name: "fancy image" })`. However, the image will not display its description if `fancy.jpg` could not be loaded. Whether you want to assert this functionality in your test or not is up to you.

<div class="warning">

**Input type password**

Unfortunately, the spec defines that `<input type="password" />` has no implicit role. This means that in order to query this type of element we must fallback to a less powerful query such as [By Label Text](./by-label-text.md).

</div>

## Options

### `hidden`

TODO

### `selected`

TODO

### `busy`

TODO

### `checked`

TODO

### `current`

TODO

### `pressed`

TODO

### `suggest`

TODO

### `expanded`

TODO

### `query_fallbacks`

TODO

### `level`

TODO

### `value`

TODO

### `description`

TODO

## Performance

`get_by_role` is the most preferred query to use as it most closely resembles the user experience, however the calculations it must perform to provide this confidence can be expensive (particularly with large DOM trees).

Where test performance is a concern it may be desirable to trade some of this confidence for improved performance.

`get_by_role` performance can be improved by setting the option [`hidden`](#hidden) to `true` and thereby avoid expensive visibility checks. Note that in doing so inaccessible elements will now be included in the result.

Another option may be to substitute `get_by_role` for simpler `get_by_label_text` and `get_by_text` queries which can be significantly faster though less robust alternatives.
