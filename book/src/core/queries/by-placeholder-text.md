# By Placeholder Text

> `get_by_placeholder_text`, `query_by_placeholder_text`, `get_all_by_placeholder_text`, `query_all_by_placeholder_text`, `find_by_placeholder_text`, `find_all_by_placeholder_text`

## API

```rust,ignore
use testing_library_dom::{Matcher, QueryError};
use web_sys::HtmlElement;

fn get_by_placeholder_text<M: Into<Matcher>>(
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

This will search for all elements with a `placeholder` attribute and find one that matches the given [`Matcher`](./about-queries.md#matcher).

```html
<input placeholder="Username" />
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let input_node = screen.get_by_placeholder_text("Username");
```

> **Note**
>
> A placeholder is not a good substitute for a label so you should generally use `get_by_label_text` instead.

## Options

[`Matcher` options](./about-queries.md#precision).
