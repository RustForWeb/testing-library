# Accessibility

## Testing for Accessibility

One of the guiding principles of the Testing Library APIs is that they should enable you to test your app the way your users use it, including through accessibility interfaces like screen readers.

See the page on [queries](../queries/about-queries.md#priority) for details on how using a semantic HTML query can make sure your app works with browser accessibility APIs.

## `get_roles`

```rust,ignore
use testing_library_dom::{get_roles, GetRolesOptions};
use wasm_bindgen_test::console_log;
use web_sys::window;

let document = window()
    .expect("Window should exist.")
    .document()
    .expect("Document should exist.");

let nav = document.create_element("nav")?;
nav.set_inner_html("\
<ul>\
    <li>Item 1</li>\
    <li>Item 2</li>\
</ul>\
");

console_log!("{:#?}", get_roles(nav, GetRolesOptions::default()));

// {
//     Navigation: [
//         Element {
//             obj: Node {
//                 obj: EventTarget {
//                     obj: Object {
//                         obj: JsValue(HTMLElement),
//                     },
//                 },
//             },
//         },
//     ],
//     List: [
//         Element {
//             obj: Node {
//                 obj: EventTarget {
//                     obj: Object {
//                         obj: JsValue(HTMLUListElement),
//                     },
//                 },
//             },
//         },
//     ],
//     Listitem: [
//         Element {
//             obj: Node {
//                 obj: EventTarget {
//                     obj: Object {
//                         obj: JsValue(HTMLLIElement),
//                     },
//                 },
//             },
//         },
//         Element {
//             obj: Node {
//                 obj: EventTarget {
//                     obj: Object {
//                         obj: JsValue(HTMLLIElement),
//                     },
//                 },
//             },
//         },
//     ],
// }
```

## `is_inaccessible`

This function will compute if the given element should be excluded from the accessibility API by the browser. It implements every **MUST** criteria from the [Excluding Elements from the Accessibility Tree](https://www.w3.org/TR/wai-aria-1.2/#tree_exclusion) section in WAI-ARIA 1.2 with the exception of checking the `role` attribute.

It is defined as:

```rust,ignore
fn is_inaccessible(element: &Element) -> bool;
```

## `log_roles`

This helper function can be used to print out a list of all the implicit ARIA roles within a tree of DOM nodes, each role containing a list of all of the nodes which match that role. This can be helpful for finding ways to query the DOM under test with [By Role](../queries/by-role.md).

```rust,ignore
use testing_library_dom::{log_roles, PrettyRolesOptions};
use web_sys::window;

let document = window()
    .expect("Window should exist.")
    .document()
    .expect("Document should exist.");

let nav = document.create_element("nav")?;
nav.set_inner_html("\
<ul>\
    <li>Item 1</li>\
    <li>Item 2</li>\
</ul>\
");

log_roles(nav, PrettyRolesOptions::default());
```
