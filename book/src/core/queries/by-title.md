# By Title

> `get_by_title`, `query_by_title`, `get_all_by_title`, `query_all_by_title`, `find_by_title`, `find_all_by_title`

## API

```rust,ignore
use testing_library_dom::{Matcher, QueryError};
use web_sys::HtmlElement;

fn get_by_title<M: Into<Matcher>>(
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

Returns the element that has the matching `title` attribute.

Will also find a `title` element within an SVG.

```html
<span title="Delete" id="2"></span>
<svg>
    <title>Close</title>
    <g><path /></g>
</svg>
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let delete_element = screen.get_by_title("Delete");
let close_element = screen.get_by_title("Close");
```

## Options

[`Matcher` options](./about-queries.md#precision).
