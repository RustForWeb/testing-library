# By Test ID

> `get_by_test_id`, `query_by_test_id`, `get_all_by_test_id`, `query_all_by_test_id`, `find_by_test_id`, `find_all_by_test_id`

## API

```rust,ignore
use testing_library_dom::{Matcher, QueryError};
use web_sys::HtmlElement;

fn get_by_test_id<M: Into<Matcher>>(
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

A shortcut to `container.query_selector(format!("[data-testid=\"{your_id}\"]"))` (and it also accepts a [`Matcher`](./about-queries.md#matcher)).

```html
<div data-testid="custom-element" />
```

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let element = screen.get_by_test_id("custom-element");
```

> In the spirit of the [guiding principles](https://testing-library.com/docs/guiding-principles/), it is recommended to use this only after the other queries don't work for your use case. Using `data-testid` attributes do not resemble how your software is used and should be avoided if possible. That said, they are way better than querying based on DOM structure or styling CSS class names. Learn more about `data-testid` from the blog post ["Making your UI tests resilient to change"](https://kentcdodds.com/blog/making-your-ui-tests-resilient-to-change).

## Options

[`Matcher` options](./about-queries.md#precision).

## Overriding `data-testid`

The `*_by_test_id` functions in DOM Testing Library use the attribute `data-testid` by default, following the precedent set by [React Native Web](https://github.com/testing-library/react-testing-library/issues/1) which uses a `testID` prop to emit a `data-testid` attribute on the element, and we recommend you adopt that attribute where possible. But if you already have an existing codebase that uses a different attribute for this purpose, you can override this value via `configure`:

```rust,ignore
use testing_library_dom::{configure, ConfigFnOrPartial, PartialConfig};

configure(ConfigFnOrPartial::Partial(
    PartialConfig::default().test_id_attribute("data-my-test-attribute")
));
```
