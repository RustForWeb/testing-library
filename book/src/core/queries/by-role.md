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
    options: ByRoleOptions,
) -> Result<HtmlElement, QueryError>;

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

Queries for elements with the given role. Default roles are taken into consideration e.g. `<button />` has the `button` role without explicitly setting the `role` attribute. Here you can see [a table of HTML elements with their default and desired roles](https://www.w3.org/TR/html-aria/#docconformance).

Please note that setting a `role` and/or `aria-*` attribute that matches the implicit ARIA semantics is unnecessary and is **not recommended** as these properties are already set by the browser, and we must not use the `role` and `aria-*` attributes in a manner that conflicts with the semantics described. For example, a `button` element can't have the `role` attribute of `heading`, because the `button` element has default characteristics that conflict with the `heading` role.

> Roles are matched literally by string equality, without inheriting from the ARIA role hierarchy. As a result, querying a superclass role like `checkbox` will not include elements with a subclass role like `switch`.

You can query the returned element(s) by their [accessible name or description](https://www.w3.org/TR/accname-1.1/). The accessible name is for simple cases equal to e.g. the label of a form element, or the text content of a button, or the value of the `aria-label` attribute. It can be used to query a specific element if multiple elements with the same role are present on the rendered content. For an in-depth guide check out ["What is an accessible name?" from TPGi](https://www.tpgi.com/what-is-an-accessible-name/). If you only query for a single element with `get_by_text("The name")` it's oftentimes better to use `get_by_role(expected_role, ByRoleOptions::default().name("The name"))`. The accessible name query does not replace other queries such as `*_by_alt` or `*_by_title`. While the accessible name can be equal to these attributes, it does not replace the functionality of these attributes. For example `<img aria-label="fancy image" src="fancy.jpg" />` will be returned for `get_by_role(AriaRole::Img, ByRoleOptions::default().name("fancy image"))` However, the image will not display its description if `fancy.jpg` could not be loaded. Whether you want to assert this functionality in your test or not is up to you.

<div class="warning">

**Input type password**

Unfortunately, the spec defines that `<input type="password" />` has no implicit role. This means that in order to query this type of element we must fallback to a less powerful query such as [By Label Text](./by-label-text.md).

</div>

The example below will find the dialog container by the `dialog` role:

```html
<div role="dialog">...</div>
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use aria_query::AriaRole;
use testing_library_dom::screen;

let screen = screen();
let dialog_container = screen.get_by_role(AriaRole::Dialog);
```

## Options

### `hidden`

If you set `hidden` to `true` elements that are normally excluded from the accessibility tree are considered for the query as well. The default behavior follows <https://www.w3.org/TR/wai-aria-1.2/#tree_exclusion> with the exception of `role="none"` and `role="presentation"` which are considered in the query in any case. For example in

```html
<body>
    <main aria-hidden="true">
        <button>Open dialog</button>
    </main>
    <div role="dialog">
        <button>Close dialog</button>
    </div>
</body>
```

`get_by_role(AriaRole::Button)` would only return the "Close dialog"-button. To make assertions about the "Open dialog"-button you would need to use `get_all_by_role(AriaRole::Button, ByRoleOptions::default().hidden(true))`.

The default value for `hidden` can be [configured](../advanced/configuration-options.md#default_hidden).

### `selected`

You can filter the returned elements by their selected state by setting selected to `true` or `false`.

For example in

```html
<body>
    <div role="tablist">
        <button role="tab" aria-selected="true">Native</button>
        <button role="tab" aria-selected="false">React</button>
        <button role="tab" aria-selected="false">Cypress</button>
    </div>
</body>
```

you can get the "Native"-tab by calling `get_by_role(AriaRole::Tab, ByRoleOptions::default().selected(true))`. To learn more about the selected state and which elements can have this state see [ARIA `aria-selected`](https://www.w3.org/TR/wai-aria-1.2/#aria-selected).

### `busy`

You can filter the returned elements by their busy state by setting busy to `true` or `false`.

For example in

```html
<body>
    <section>
        <div role="alert" aria-busy="false">Login failed</div>
        <div role="alert" aria-busy="true">Error: Loading message...</div>
    </section>
</body>
```

you can get the "Login failed" alert by calling `get_by_role(AriaRole::Alert, ByRoleOptions::default().busy(false))`. To learn more about the busy state see [ARIA `aria-busy`](https://www.w3.org/TR/wai-aria-1.2/#aria-busy) and [MDN `aria-busy` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-busy).

### `checked`

You can filter the returned elements by their checked state by setting checked to `true` or `false`.

For example in

```html
<body>
    <section>
        <button role="checkbox" aria-checked="true">Sugar</button>
        <button role="checkbox" aria-checked="false">Gummy bears</button>
        <button role="checkbox" aria-checked="false">Whipped cream</button>
    </section>
</body>
```

you can get the "Sugar" option by calling `get_by_role(AriaRole::Checkbox, ByRoleOptions::default().checked(true))`. To learn more about the checked state and which elements can have this state see [ARIA `aria-checked`](https://www.w3.org/TR/wai-aria-1.2/#aria-checked).

> **Note**
>
> Checkboxes have a "mixed" state, which is considered neither checked nor unchecked (details [here](https://www.w3.org/TR/html-aam-1.0/#el-input-checkbox)).

### `current`

You can filter the returned elements by their current state by setting current to a boolean or string. Note that no `aria-current` attribute will match `false` since `false` is the default value for `aria-current`.

For example in

```html
<body>
    <nav>
        <a href="current/page" aria-current="page">👍</a>
        <a href="another/page">👎</a>
    </nav>
</body>
```

you can get the "👍" link by calling `get_by_role(AriaRole::Link, ByRoleOptions::default().current("page"))` and the "👎" by calling `get_by_role(AriaRole::Link, ByRoleOptions::default().current(false))`. To learn more about the current state see [ARIA `aria-current`](https://www.w3.org/TR/wai-aria-1.2/#aria-current).

### `pressed`

Buttons can have a pressed state. You can filter the returned elements by their pressed state by setting pressed to `true` or `false`.

For example in

```html
<body>
    <section>
        <button aria-pressed="true">👍</button>
        <button aria-pressed="false">👎</button>
    </section>
</body>
```

you can get the "👍" button by calling `get_by_role(AriaRole::Button, ByRoleOptions::default().pressed(true))`. To learn more about the pressed state see [ARIA `aria-pressed`](https://www.w3.org/TR/wai-aria-1.2/#aria-pressed).

### `suggest`

You can disable the ability to [throw suggestions](../advanced/configuration-options.md#throw_suggestions-experimental) for a specific query by setting this value to `false`.
Setting this value to `true` will throw suggestions for the specific query.

### `expanded`

You can filter the returned elements by their expanded state by setting expanded to `true` or `false`.

For example in

```html
<body>
    <nav>
        <ul>
            <li>
                <a aria-expanded="false" aria-haspopup="true" href="...">Expandable Menu Item</a>
                <ul>
                    <li><a href="#">Submenu Item 1</a></li>
                    <li><a href="#">Submenu Item 1</a></li>
                </ul>
            </li>
            <li><a href="#">Regular Menu Item</a></li>
        </ul>
    </nav>
</body>
```

you can get the "Expandable Menu Item" link by calling `get_by_role(AriaRole::Link, ByRoleOptions::default().expanded(false))`. To learn more about the expanded state and which elements can have this state see [ARIA `aria-expanded`](https://www.w3.org/TR/wai-aria-1.2/#aria-expanded).

### `query_fallbacks`

By default, it's assumed that the first role of each element is supported, so only the first role can be queried. If you need to query an element by any of its fallback roles instead, you can set `query_fallbacks` to `true`.

For example, `get_by_role(AriaRole::Switch)` would always match `<div role="switch checkbox" />` because it's the first role, while `get_by_role(AriaRole::Checkbox)` would not. However, `get_by_role(AriaRole::Checkbox, ByRoleOptions::default().query_fallbacks(true))` would enable all fallback roles and therefore match the same element.

> An element doesn't have multiple roles in a given environment. It has a single one. Multiple roles in the attribute are evaluated from left to right until the environment finds the first role it understands. This is useful when new roles get introduced and you want to start supporting those as well as older environments that don't understand that role (yet).

### `level`

An element with the `heading` role can be queried by any heading level `get_by_role(AriaRole::Heading)` or by a specific heading level using the `level` option `get_by_role(AriaRole::Heading, ByRoleOptions::default().level(2))`.

The `level` option queries the element(s) with the `heading` role matching the indicated level determined by the semantic HTML heading elements `<h1>-<h6>` or matching the `aria-level` attribute.

Given the example below,

```html
<body>
    <section>
        <h1>Heading Level One</h1>
        <h2>First Heading Level Two</h2>
        <h3>Heading Level Three</h3>
        <div role="heading" aria-level="2">Second Heading Level Two</div>
    </section>
</body>
```

you can query the "Heading Level Three" heading using `get_by_role(AriaRole::Heading, ByRoleOptions::default().level(3))`.

```rust,ignore
get_by_role(AriaRole::Heading, ByRoleOptions::default().level(1))
// <h1>Heading Level One</h1>

get_all_by_role(AriaRole::Heading, ByRoleOptions::default().level(2))
// [
//     <h2>First Heading Level Two</h2>,
//     <div role="heading" aria-level="2">Second Heading Level Two</div>
// ]
```

While it is possible to explicitly set `role="heading"` and `aria-level` attribute on an element, it is strongly encouraged to use the semantic HTML headings `<h1>-<h6>`.

To learn more about the `aria-level` property, see [ARIA `aria-level`](https://www.w3.org/TR/wai-aria-1.2/#aria-level).

> The `level` option is only applicable to the `heading` role. An error will be thrown when used with any other role.

### `value`

A range widget can be queried by any value `get_by_role(AriaRole::Spinbutton)` or by a specific value using the `value` option `get_by_role(AriaRole::Spinbutton, ByRoleOptions::default().value(ByRoleOptionsValue::default().now(5).min(0).max(10).text("medium")))`.

Note that you don't have to specify all properties in `value`. A subset is sufficient e.g. `ByRoleOptionsValue::default().now(5).text("medium")`.

Given the example below,

```html
<body>
    <section>
        <button role="spinbutton" aria-valuenow="5" aria-valuemin="0" aria-valuemax="10" aria-valuetext="medium">
            Volume
        </button>
        <button role="spinbutton" aria-valuenow="3" aria-valuemin="0" aria-valuemax="10" aria-valuetext="medium">
            Pitch
        </button>
    </section>
</body>
```

you can query specific spinbutton(s) with the following queries,

```rust,ignore
get_by_role(AriaRole::Spinbutton, ByRoleOptions::default().value(
    ByRoleOptionsValue::default().now(5)
))
// <button>Volume</button>

get_all_by_role(AriaRole::Spinbutton, ByRoleOptions::default().value(
    ByRoleOptionsValue::default().min(0)
))
// [
//     <button>Volume</button>,
//     <button>Pitch</button>
// ]
```

> Every specified property in value must match. For example, if you query for `ByRoleOptionsValue::default().min(0).now(3)`, `aria-valuemin` must be equal to 0 **AND** `aria-valuenow` must be equal to 3.

> The `value` option is only applicable to certain roles (check the linked MDN pages below for applicable roles). An error will be thrown when used with any other role.

To learn more about the `aria-value*` properties, see [MDN `aria-valuemin`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-valuemin), [MDN `aria-valuemax`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-valuemax), [MDN `aria-valuenow`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-valuenow), [MDN `aria-valuetext`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-valuetext).

### `description`

You can filter the returned elements by their [accessible description](https://www.w3.org/TR/accname-1.1/#mapping_additional_nd_description) for those cases where you have several elements with the same role and they don't have an accessible name but they do have a description.
This would be the case for elements with [`alertdialog`](https://www.w3.org/TR/wai-aria-1.1/#alertdialog) role, where the `aria-describedby` attribute is used to describe the element's content.

For example in

```html
<body>
    <ul>
        <li role="alertdialog" aria-describedby="notification-id-1">
            <div><button>Close</button></div>
            <div id="notification-id-1">You have unread emails</div>
        </li>
        <li role="alertdialog" aria-describedby="notification-id-2">
            <div><button>Close</button></div>
            <div id="notification-id-2">Your session is about to expire</div>
        </li>
    </ul>
</body>
```

You can query a specific element like this

```rust,ignore
get_by_role(AriaRole::Alertdialog, ByRoleOptions::default().description(
    "Your session is about to expire"
))
```

## Performance

`get_by_role` is the most preferred query to use as it most closely resembles the user experience, however the calculations it must perform to provide this confidence can be expensive (particularly with large DOM trees).

Where test performance is a concern it may be desirable to trade some of this confidence for improved performance.

`get_by_role` performance can be improved by setting the option [`hidden`](#hidden) to `true` and thereby avoid expensive visibility checks. Note that in doing so inaccessible elements will now be included in the result.

Another option may be to substitute `get_by_role` for simpler `get_by_label_text` and `get_by_text` queries which can be significantly faster though less robust alternatives.
