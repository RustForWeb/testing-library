# By Text

> `get_by_text`, `query_by_text`, `get_all_by_text`, `query_all_by_text`, `find_by_text`, `find_all_by_text`

## API

```rust,ignore
use testing_library_dom::{Matcher, QueryError};
use web_sys::HtmlElement;

fn get_by_text<M: Into<Matcher>>(
    // If you're using `screen`, then skip the container argument:
    container: &HtmlElement,
    matcher: M,
    options: SelectorMatcherOptions,
) -> Result<HtmlElement, QueryError>;

struct SelectorMatcherOptions {
    selector: Option<String>,
    exact: Option<bool>,
    ignore: Option<Ignore>,
    normalizer: Option<Rc<NormalizerFn>>,
}

enum Ignore {
    False,
    String(String),
}

type NormalizerFn = dyn Fn(String) -> String;
```

This will search for all elements that have a text node with `textContent` matching the given [`Matcher`](./about-queries.md#matcher).

```html
<a href="/about">About ℹ️</a>
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let about_anchor_node = screen.get_by_text(Regex::new(r"(?i)about")?);
```

It also works with `input`s whose `type` attribute is either `submit` or `button`:

```html
<input type="submit" value="Send data" />
```

## Options

[`Matcher` options](./about-queries.md#precision), plus the following:

### `selector`

See [By Label Text](./by-label-text.md#selector) for more details on how and when to use the `selector` option.

### `ignore`

The `ignore` option accepts a query selector. If the [`node.matches`](https://developer.mozilla.org/en-US/docs/Web/API/Element/matches) returns true for that selector, the node will be ignored. This defaults to `"script, style"` because generally you don't want to select these tags, but if your content is in an inline script file, then the script tag could be returned.

If you'd rather disable this behavior, set `ignore` to `Ignore::False`.
