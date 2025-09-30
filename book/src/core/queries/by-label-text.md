# By Label Text

> `get_by_label_text`, `query_by_label_text`, `get_all_by_label_text`, `query_all_by_label_text`, `find_by_label_text`, `find_all_by_label_text`

## API

```rust,ignore
use testing_library_dom::{Matcher, QueryError};
use web_sys::HtmlElement;

fn get_by_label_text<M: Into<Matcher>>(
    // If you're using `screen`, then skip the container argument:
    container: &HtmlElement,
    matcher: M,
    options: SelectorMatcherOptions,
) -> Result<HtmlElement, QueryError>;

struct SelectorMatcherOptions {
    selector: Option<String>,
    exact: Option<bool>,
    normalizer: Option<Rc<NormalizerFn>>,
}

type NormalizerFn = dyn Fn(String) -> String;
```

This will search for the label that matches the given [`Matcher`](./about-queries.md#matcher), then find the element associated with that label.

The example below will find the input node for the following DOM structures:

```html
<!-- for relationship between label and form element id -->
<label for="username-input">Username</label>
<input id="username-input" />

<!-- The aria-labelledby attribute with form elements -->
<label id="username-label">Username</label>
<input aria-labelledby="username-label" />

<!-- Wrapper labels -->
<label>Username <input /></label>

<!-- Wrapper labels where the label text is in another child element -->
<label>
    <span>Username</span>
    <input />
</label>

<!-- aria-label attributes
Take care because this is not a label that users can see on the page,
so the purpose of your input must be obvious to visual users. -->
<input aria-label="Username" />
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let input_node = screen.get_by_label_text("Username");
```

The example above does NOT find the input node for label text broken up by elements. You can use `get_by_role(AriaRole::Textbox, ByRoleOptions::default().name("Username"))` instead which is robust against switching to `aria-label` or `aria-labelledby`.

## Options

[`Matcher` options](./about-queries.md#precision), plus the following:

### `selector`

If it is important that you query a specific element (e.g. an `<input>`) you can provide a `selector` in the options:

```html
<!-- Multiple elements labelled via aria-labelledby -->
<label id="username">Username</label>
<input aria-labelledby="username" />
<span aria-labelledby="username">Please enter your username</span>

<!-- Multiple labels with the same text-->
<label>
    Username
    <input />
</label>
<label>
    Username
    <textarea></textarea>
</label>
```

```rust,ignore
let input_node = screen.get_by_label_text("Username", SelectorMatcherOptions::default().selector("input"));
```

> \*\*Note
>
> `get_by_label_text` will not work in the case where a `for` attribute on a `<label>` element matches an `id` attribute on a non-form element.
>
> ```html
> <!-- This case is not valid -->
> <!-- for between label and an element that is not a form element -->
> <section id="photos-section">
>     <label for="photos-section">Photos</label>
> </section>
> ```
