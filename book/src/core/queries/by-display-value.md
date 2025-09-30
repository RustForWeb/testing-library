# By Display Value

> `get_by_display_value`, `query_by_display_value`, `get_all_by_display_value`, `query_all_by_display_value`, `find_by_display_value`, `find_all_by_display_value`

## API

```rust,ignore
use testing_library_dom::{Matcher, QueryError};
use web_sys::HtmlElement;

fn get_by_display_value<M: Into<Matcher>>(
    // If you're using `screen`, then skip the container argument:
    container: &HtmlElement,
    matcher: M,
    options: MatcherOptions,
) -> Result<HtmlElement, QueryError>;

struct MatcherOptions {
    exact: Option<bool>,
    normalizer: Option<Rc<NormalizerFn>>,
}

type NormalizerFn = dyn Fn(String) -> String;
```

Returns the `input`, `textarea`, or `select` element that has the matching display value.

### `input` tags

```html
<input type="text" id="lastName" />
```

```rust,ignore
document.get_element_by_id("lastName")?.set_value("Norris");
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let last_name_input = screen.get_by_display_value("Norris");
```

### `textarea` tags

```html
<textarea id="messageTextArea" />
```

```rust,ignore
document.get_element_by_id("messageTextArea")?.set_value("Hello World");
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let message_text_area = screen.get_by_display_value("Hello World");
```

### `select` tags

In case of select, this will search for a `<select>` whose selected `<option>` matches the given [`Matcher`](./about-queries.md#matcher).

```html
<select>
    <option value="">State</option>
    <option value="AL">Alabama</option>
    <option selected value="AK">Alaska</option>
    <option value="AZ">Arizona</option>
</select>
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let select_element = screen.get_by_display_value("Alaska");
```

## Options

[`Matcher` options](./about-queries.md#precision).
