# About Queries

## Overview

Queries are the methods that Testing Library gives you to find elements on the page. There are several [types of queries](#types-of-queries) ("get", "find", "query"); the difference between them is whether the query will return an error if no element is found or if it will return a future and retry. Depending on what page content you are selecting, different queries may be more or less appropriate. See the [priority guide](#priority) for recommendations on how to make use of semantic queries to test your page in the most accessible way.

After selecting an element, you can use the [Events API](../user-actions/firing-events.md) or [user-event](#) to fire events and simulate user interactions with the page, or make assertions about the element.

There are Testing Library helper methods that work with queries. As elements appear and disappear in response to actions, [Async APIs](#) like [`wait_for`](#) or [`find_by` queries](#) can be used to await the changes in the DOM. To find only elements that are children of a specific element, you can use [`within`](#). If necessary, there are also a few options you can configure, like the timeout for retries and the default test ID attribute.

## Example

TODO

## Types of Queries

- Single Elements
    - `get_by...`: Returns the matching node for a query, and returns a descriptive error if no elements match or if more than one match is found (use `get_all_by` instead if more than one element is expected).
    - `query_by...`: Returns the matching node for a query, and returns `None` if no elements match. This is useful for asserting an element that is not present. Returns an error if more than one match is found (use `query_all_by` instead if this is OK).
    - `find_by...`: Returns a future which resolves when an element is found which matches the given query. The future is rejected if no element is found or if more than one element is found after a default timeout of 1000ms (use `find_all_by` if you need to find more than one element).
        - `find_by` methods are a combination of `get_by` queries and [`wait_for`](#). They accept the `wait_for` options as the last argument (i.e. `screen.find_by_text("text", query_options, wait_for_options).await`).
- Multiple Elements
    - `get_all_by...`: Returns a vector of all matching nodes for a query, and returns an error if no elements match.
    - `query_all_by...`: Returns a vector of all matching nodes for a query, and returns an empty vector (`vec![]`) if no elements match.
    - `find_all_by...`: Returns a future which resolves to a vector of elements when any elements are found which match the given query. The future is rejected if no elements are found after a default timeout of 1000ms.

| Type of Query         | 0 Matches | 1 Match | >1 Matches | Retry |
| --------------------- | --------- | ------- | ---------- | ----- |
| **Single Element**    |           |         |            |       |
| `get_by`              | Error     | Element | Error      | No    |
| `query_by`            | `None`    | Element | Error      | No    |
| `find_by`             | Error     | Element | Error      | Yes   |
| **Multiple Elements** |           |         |            |       |
| `get_all_by`          | Error     | Vector  | Vector     | No    |
| `query_all_by`        | `vec![]`  | Vector  | Vector     | No    |
| `find_all_by`         | Error     | Vector  | Vector     | Yes   |

## Priority

Based on the [Guiding Principles](https://testing-library.com/docs/guiding-principles/), your test should resemble how users interact with your code (component, page, etc.) as much as possible. With this in mind, we recommend this order of priority:

1. **Queries Accessible to Everyone** Queries that reflect the experience of visual/mouse users as well as those that use assistive technology.
    1. `get_by_role`: This can be used to query every element that is exposed in the [accessibility tree](https://developer.mozilla.org/en-US/docs/Glossary/Accessibility_tree). With the `name` option you can filter the returned elements by their [accessible name](https://www.w3.org/TR/accname-1.1/). This should be your top preference for just about everything. There's not much you can't get with this (if you can't, it's possible your UI is inaccessible). Most often, this will be used with the `name` option like so: `get_by_role(AriaRole::Button, ByRoleOptions::default().name("submit"))`. Check the list of roles.
    2. `get_by_label_text`: This method is really good for form fields. When navigating through a website form, users find elements using label text. This method emulates that behavior, so it should be your top preference.
    3. `get_by_placeholder_text`: [A placeholder is not a substitute for a label](https://www.nngroup.com/articles/form-design-placeholders/). But if that's all you have, then it's better than alternatives.
    4. `get_by_text`: Outside of forms, text content is the main way users find elements. This method can be used to find non-interactive elements (like divs, spans, and paragraphs).
    5. `get_by_display_value`: The current value of a form element can be useful when navigating a page with filled-in values.
2. **Semantic Queries** HTML5 and ARIA compliant selectors. Note that the user experience of interacting with these attributes varies greatly across browsers and assistive technology.
    1. `get_by_alt_text`: If your element is one which supports `alt` text (`img`, `area`, `input`, and any custom element), then you can use this to find that element.
    2. `get_by_title`: The title attribute is not consistently read by screenreaders, and is not visible by default for sighted users
3. **Test IDs**
    1. `get_by_test_id`: The user cannot see (or hear) these, so this is only recommended for cases where you can't match by role or text or it doesn't make sense (e.g. the text is dynamic).

## Using Queries

The base queries from DOM Testing Library require you to pass a `container` as the first argument. Most framework-implementations of Testing Library provide a pre-bound version of these queries when you render your components with them, which means you do not have to provide a container. In addition, if you just want to query `document.body` then you can use the [`screen`](#screen) export as demonstrated below (using `screen` is recommended).

The primary argument to a query can be a _string_, _regular expression_, or _function_. There are also options to adjust how node text is parsed. See [`Matcher`](#matcher) for documentation on what can be passed to a query.

Given the following DOM elements (which can be rendered by Dioxus, Leptos, Yew, or plain HTML code):

```html
<body>
    <div id="app">
        <label for="username-input">Username</label>
        <input id="username-input" />
    </div>
</body>
```

You can use a query to find an element (by label text, in this case):

```rust,ignore
use testing_library_dom::{get_by_label_text, Screen, SelectorMatcherOptions};

// With screen:
let input_node_1 = Screen::get_by_label_text("Username", SelectorMatcherOptions::default()).expect("Get should succeed.");

// Without screen, you need to provide a container:
let container = document.query_selector("#app").expect("Query should succeed.").expect("Element should exist.");
let input_node_2 = get_by_label_text(&container, "Username", SelectorMatcherOptions::default()).expect("Get should succeed.");
```

### `Options`

You can pass an `Options` struct instance to the query. See the docs for each query to see available options, e.g. [By Role API](./by-role.md).

### `screen`

All of the queries exported by DOM Testing Library accept a `container` as the first argument. Because querying the entire `document.body` is very common, DOM Testing Library also exports a `screen` function which returns a struct that has every query that is pre-bound to `document.body` (using the [`within`](#) functionality). <!-- Wrappers such as React Testing Library re-export screen so you can use it the same way. -->

Here's how you use it:

<!-- TODO: Tabs with framework examples -->

```rust,ignore
use testing_library_dom::screen;

let screen = screen();
let example_input = screen.get_by_label_text("Example").expect("Get should succeed.");
```

## `Matcher`

Most of the query APIs take a `Matcher` as an argument, which means the argument can be either a _string_, _regex_, or a _function_ of signature `Fn(String, Option<&Element>) -> bool` which returns `true` for a match and `false` for a mismatch.

### Examples

Given the following HTML:

```html
<div>Hello World</div>
```

**_Will_ find the div:**

```rust,ignore
// Matching a string:
screen.get_by_text("Hello World"); // Full string match
screen.get_by_text("llo Worl", SelectorMatcherOptions::default().exact(false)); // Substring match
screen.get_by_text("hello world", SelectorMatcherOptions::default().exact(false)); // Ignore case

// Matching a regex:
screen.get_by_text(Regex::new(r"World")?); // Substring match
screen.get_by_text(Regex::new(r"(?i)world")?); // Substring match, ignore case
screen.get_by_text(Regex::new(r"(?i)^hello world$")?); // Full string match, ignore case
screen.get_by_text(Regex::new(r"(?i)Hello W?oRlD")?); // Substring match, ignore case, searches for "hello world" or "hello orld"

// Matching with a custom function:
screen.get_by_text(|content, element| => content.starts_with("Hello"));
```

**_Will not_ find the div:**

```rust,ignore
// Full string does not match
screen.get_by_text("Goodbye World");

// Case-sensitive regex with different case
screen.get_by_text(Regex::new("hello world")?);

// Function looking for a span when it's actually a div
screen.get_by_text(|content, element| {
  element.is_some_and(|element| element.tag_name().to_lower_case() == "span") && content.starts_with("Hello")
});
```

### Precision

Queries that take a `Matcher` also accept a struct instance as the final argument that can contain options that affect the precision of string matching:

- `exact`: Defaults to `true`; matches full strings, case-sensitive. When false, matches substrings and is not case-sensitive.
    - It has no effect when used together with regex or function arguments.
    - In most cases, using a regex instead of a string combined with `.exact(false)` gives you more control over fuzzy matching so it should be preferred.
- `normalizer`: An optional function which overrides normalization behavior. See [Normalization](#normalization).

### Normalization

Before running any matching logic against text in the DOM, Testing Library automatically normalizes that text. By default, normalization consists of trimming whitespace from the start and end of text, and **collapsing multiple adjacent whitespace characters within the string into a single space**.

If you want to prevent that normalization, or provide alternative normalization (e.g. to remove Unicode control characters), you can provide a `normalizer` function in the options object. This function will be given a string and is expected to return a normalized version of that string.

> **Note**
>
> Specifying a value for `normalizer` replaces the built-in normalization, but you can call `get_default_normalizer` to obtain a built-in normalizer, either to adjust that normalization or to call it from your own normalizer.

`get_default_normalizer` takes an options object which allows the selection of behaviour:

- `trim`: Defaults to `true`. Trims leading and trailing whitespace.
- `collapse_whitespace`: Defaults to `true`. Collapses inner whitespace (newlines, tabs, repeated spaces) into a single space.

#### Normalization Examples

To perform a match against text without trimming:

```rust,ignore
screen.get_by_text(
    "text",
    SelectorMatcherOptions::default().normalizer(get_default_normalizer(
        DefaultNormalizerOptions::default().trim(false),
    )),
);
```

To override normalization to remove some Unicode characters whilst keeping some (but not all) of the built-in normalization behavior:

```rust,ignore
screen.get_by_text(
    "text",
    SelectorMatcherOptions::default().normalizer({
        let regex = Regex::new(r"[\u200E-\u200F]*")?;
        let normalizer =
            get_default_normalizer(DefaultNormalizerOptions::default().trim(false));

        Rc::new(move |text| regex.replace_all(&normalizer(text), "").to_string())
    }),
);
```

## Manual Queries

On top of the queries provided by the testing library, you can use the regular [`query_selector` DOM API](https://docs.rs/web-sys/latest/web_sys/struct.Document.html#method.query_selector) to query elements. Note that using this as an escape hatch to query by `class` or `id` is not recommended because they are invisible to the user. Use a `testid` if you have to, to make your intention to fall back to non-semantic queries clear and establish a stable API contract in the HTML.

<!-- TODO: Use `container` from framework `render` API.-->

```rust,ignore
let foo = document.query_selector('[data-foo="bar"]').expect("Query should succeed.").expect("Element should exist.");
```
