# By Alt Text

> `get_by_alt_text`, `query_by_alt_text`, `get_all_by_alt_text`, `query_all_by_alt_text`, `find_by_alt_text`, `find_all_by_alt_text`

## API

```rust,ignore
use testing_library_dom::{Matcher, QueryError};
use web_sys::HtmlElement;

fn get_by_alt_text<M: Into<Matcher>>(
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

This will return the element (normally an `<img>`) that has the given `alt` text. Note that it only supports elements which accept an `alt` attribute or [custom elements](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements) (since we don't know if a custom element implements `alt` or not): `<img>`, `<input>`, and `<area>` (intentionally excluding `<applet>` as it's deprecated).

```html
<img alt="Incredibles 2 Poster" src="/incredibles-2.png" />
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let incredibles_poster_img = screen.get_by_alt_text(Regex::new(r"(?i)incredibles.*? poster")?);
```

## Options

[`Matcher` options](./about-queries.md#precision).
