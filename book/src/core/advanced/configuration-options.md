# Configuration Options

## Introduction

## Options

### `default_hidden`

The default value for the [`hidden` option](../queries/by-role.md#hidden) used by `get_by_role`. Defaults to `false`.

### `default_ignore`

The default value for the [`ignore` option](../queries/by-text.md#ignore) used by `get_by_text`. Also determines the nodes that are being ignored when errors are printed.

Defaults to `script, style`.

### `throw_suggestions` (experimental)

When enabled, if [better queries](../queries/about-queries.md#priority) are available, the test will fail and provide a suggested query to use instead. Defaults to `false`.

To disable a suggestion for a single query just add `.suggest(false)` as an option.

```rust,ignore
// Will not throw a suggestion.
screen.get_by_test_id("foo", MatcherOptions::default().suggest(false))
```

> **Note**
>
> When this option is enabled, it may provide suggestions that lack an intuitive implementation. Typically this happens for [roles which cannot be named](https://w3c.github.io/aria/#namefromprohibited), most notably paragraphs. For instance, if you attempt to use `get_by_text`, you may encounter the following error:
>
> ```text
> A better query is available, try this:
>     get_by_role(AriaRole::Paragraph)
> ```
>
> However, there is no direct way to query paragraphs using the config parameter, such as in `get_by_role(AriaRole::Paragraph, ByRoleOptions::default().name("Hello World"))`.
>
> To address this issue, you can leverage a custom function to validate the element's structure, as shown in the example below. More information can be found in the [GitHub issue](https://github.com/testing-library/dom-testing-library/issues/1306).
>
> ```rust,ignore
> get_by_role(
>     AriaRole::Paragraph,
>     ByRoleOptions::default().name(Rc::new(|_, element| {
>         element.map(|element| element.text_content()).is_some_and(|content| content == "Hello World")
>     }))
> )
> ```

### `test_id_attribute`

The attribute used by `get_by_test_id` and related queries. Defaults to `data-testid`.

### `get_element_error`

A function that returns the error used when [get or find queries](../queries/about-queries.md#types-of-queries) fail. Takes the error message and container as arguments.

### `async_util_timeout`

The global timeout value in milliseconds used by `wait_for` utilities. Defaults to 1000ms.
