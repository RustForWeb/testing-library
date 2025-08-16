#![cfg(target_arch = "wasm32")]

mod helpers;

use std::{collections::HashMap, hash::RandomState};

use indoc::indoc;
use log::Level;
use ordered_hash_map::OrderedHashMap;
use pretty_assertions::assert_eq;
use testing_library_dom::{
    AriaRoleDefinitionKey, GetRolesOptions, MatcherOptions, PrettyRolesOptions,
    get_implicit_aria_roles, get_roles, is_inaccessible, log_roles,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::{Element, HtmlElement};

use self::helpers::test_utils::{RenderReturn, document, render};

wasm_bindgen_test_configure!(run_in_browser);

struct Setup {
    unnamed_section: Element,
    named_section: Element,
    anchor: Element,
    h1: Element,
    h2: Element,
    h3: Element,
    nav: Element,
    article: Element,
    a_ul: Element,
    a_li1: Element,
    a_li2: Element,
    b_ul: Element,
    b_li1: Element,
    b_li2: Element,
    table: Element,
    tbody: Element,
    tr: Element,
    td1: Element,
    td2: Element,
    td3: Element,
    unnamed_form: Element,
    named_form: Element,
    radio: Element,
    radio2: Element,
    input: Element,
    input2: Element,
    textarea: Element,
    dt: Element,
    dd: Element,
    header: Element,
    invalid_anchor: Element,
    unnamed_img: Element,
    presentation_img: Element,
    named_img: Element,
    footer: Element,
}

fn setup() -> Setup {
    let RenderReturn {
        container_queries, ..
    } = render(
        indoc! {"
          <header data-testid=\"a-header\">Banner header</header>
          <section aria-label=\"a region\" data-testid='named-section'>
            <a href=\"http://whatever.com\" data-testid=\"a-link\">link</a>
            <a data-testid=\"invalid-link\">invalid link</a>

            <nav data-testid='a-nav' />

            <h1 data-testid='a-h1'>Main Heading</h1>
            <h2 data-testid='a-h2'>Sub Heading</h2>
            <h3 data-testid='a-h3'>Tertiary Heading</h3>

            <article data-testid='a-article'>
              <ul data-testid='a-list'>
                <li data-testid='a-list-item-1'>Item 1</li>
                <li data-testid='a-list-item-2'>Item 2</li>
              </ul>

              <table data-testid='a-table'>
                <tbody data-testid='a-tbody'>
                  <tr data-testid='a-row'>
                    <td data-testid='a-cell-1'>Cell 1</td>
                    <td data-testid='a-cell-2'>Cell 2</td>
                    <td data-testid='a-cell-3'>Cell 3</td>
                  </tr>
                </tbody>
              </table>

              <form aria-label=\"a form\" data-testid='named-form'>
                <input type='radio' data-testid='a-radio-1' />
                <input type='radio' data-testid='a-radio-2' />
                <input type='text' data-testid='a-input-1' />
                <input type='text' data-testid='a-input-2' />
                <textarea data-testid='a-textarea'></textarea>
              </form>

              <ul data-testid='b-list'>
                <li data-testid='b-list-item-1'>Item 1</li>
                <li data-testid='b-list-item-2'>Item 2</li>
              </ul>

              <form data-testid=\"a-form\" />
              <section data-testid=\"a-section\" />
            </article>
            <dl>
              <dt data-testid=\"a-dt\">Term</dt>
              <dd data-testid=\"a-dd\">Definition</dd>
            </dl>

            <img src=\"http://example.com/image.png\" data-testid='a-img-1'/>
            <img alt=\"\" src=\"http://example.com/image.png\" data-testid='a-img-2'/>
            <img alt=\"a meaningful description\" src=\"http://example.com/image.png\" data-testid='a-img-3'/>
          </section>
          <footer data-testid=\"a-footer\">Contentinfo footer</footer>
        "},
        None,
    );

    Setup {
        unnamed_section: container_queries
            .get_by_test_id("a-section", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        named_section: container_queries
            .get_by_test_id("named-section", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        anchor: container_queries
            .get_by_test_id("a-link", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        h1: container_queries
            .get_by_test_id("a-h1", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        h2: container_queries
            .get_by_test_id("a-h2", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        h3: container_queries
            .get_by_test_id("a-h3", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        nav: container_queries
            .get_by_test_id("a-nav", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        article: container_queries
            .get_by_test_id("a-article", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        a_ul: container_queries
            .get_by_test_id("a-list", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        a_li1: container_queries
            .get_by_test_id("a-list-item-1", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        a_li2: container_queries
            .get_by_test_id("a-list-item-2", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        b_ul: container_queries
            .get_by_test_id("b-list", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        b_li1: container_queries
            .get_by_test_id("b-list-item-1", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        b_li2: container_queries
            .get_by_test_id("b-list-item-2", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        table: container_queries
            .get_by_test_id("a-table", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        tbody: container_queries
            .get_by_test_id("a-tbody", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        tr: container_queries
            .get_by_test_id("a-row", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        td1: container_queries
            .get_by_test_id("a-cell-1", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        td2: container_queries
            .get_by_test_id("a-cell-2", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        td3: container_queries
            .get_by_test_id("a-cell-3", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        unnamed_form: container_queries
            .get_by_test_id("a-form", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        named_form: container_queries
            .get_by_test_id("named-form", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        radio: container_queries
            .get_by_test_id("a-radio-1", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        radio2: container_queries
            .get_by_test_id("a-radio-2", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        input: container_queries
            .get_by_test_id("a-input-1", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        input2: container_queries
            .get_by_test_id("a-input-2", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        textarea: container_queries
            .get_by_test_id("a-textarea", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        dt: container_queries
            .get_by_test_id("a-dt", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        dd: container_queries
            .get_by_test_id("a-dd", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        header: container_queries
            .get_by_test_id("a-header", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        invalid_anchor: container_queries
            .get_by_test_id("invalid-link", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        unnamed_img: container_queries
            .get_by_test_id("a-img-1", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        presentation_img: container_queries
            .get_by_test_id("a-img-2", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        named_img: container_queries
            .get_by_test_id("a-img-3", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
        footer: container_queries
            .get_by_test_id("a-footer", MatcherOptions::default())
            .expect("Get should succeed.")
            .into(),
    }
}

#[wasm_bindgen_test]
fn get_roles_returns_expected_roles_for_various_dom_nodes() {
    let Setup {
        anchor,
        h1,
        h2,
        h3,
        nav,
        article,
        a_ul,
        a_li1,
        a_li2,
        b_ul,
        b_li1,
        b_li2,
        table,
        tbody,
        tr,
        td1,
        td2,
        td3,
        radio,
        radio2,
        input,
        input2,
        textarea,
        named_section,
        named_form,
        dd,
        dt,
        header,
        invalid_anchor,
        unnamed_section,
        unnamed_img,
        presentation_img,
        named_img,
        footer,
        ..
    } = setup();

    assert_eq!(
        // Compare without ordering.
        HashMap::<_, _, RandomState>::from_iter(get_roles(
            named_section.clone(),
            GetRolesOptions::default()
        )),
        HashMap::from_iter([
            (AriaRoleDefinitionKey::Link, vec![anchor]),
            (AriaRoleDefinitionKey::Heading, vec![h1, h2, h3]),
            (AriaRoleDefinitionKey::Navigation, vec![nav]),
            (AriaRoleDefinitionKey::Radio, vec![radio, radio2]),
            (AriaRoleDefinitionKey::Article, vec![article]),
            (AriaRoleDefinitionKey::List, vec![a_ul, b_ul]),
            (
                AriaRoleDefinitionKey::Listitem,
                vec![a_li1, a_li2, b_li1, b_li2]
            ),
            (AriaRoleDefinitionKey::Table, vec![table]),
            (AriaRoleDefinitionKey::Row, vec![tr]),
            (AriaRoleDefinitionKey::Cell, vec![td1, td2, td3]),
            (
                AriaRoleDefinitionKey::Textbox,
                vec![input, input2, textarea]
            ),
            (AriaRoleDefinitionKey::Rowgroup, vec![tbody]),
            (AriaRoleDefinitionKey::Form, vec![named_form]),
            (AriaRoleDefinitionKey::Region, vec![named_section]),
            (AriaRoleDefinitionKey::Term, vec![dt]),
            (AriaRoleDefinitionKey::Definition, vec![dd]),
            (
                AriaRoleDefinitionKey::Generic,
                vec![invalid_anchor, unnamed_section]
            ),
            (AriaRoleDefinitionKey::Img, vec![unnamed_img, named_img]),
            (AriaRoleDefinitionKey::Presentation, vec![presentation_img]),
        ])
    );

    assert_eq!(
        get_roles(header.clone(), GetRolesOptions::default()),
        OrderedHashMap::from_iter([(AriaRoleDefinitionKey::Banner, vec![header])])
    );
    assert_eq!(
        get_roles(footer.clone(), GetRolesOptions::default()),
        OrderedHashMap::from_iter([(AriaRoleDefinitionKey::Contentinfo, vec![footer])])
    );
}

#[ignore]
#[wasm_bindgen_test]
fn log_roles_calls_log_info_with_output_from_pretty_roles() {
    let Setup { named_section, .. } = setup();

    testing_logger::setup();

    log_roles(named_section, PrettyRolesOptions::default());

    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 1);
        assert_eq!(captured_logs[0].level, Level::Info);
        assert_eq!(
            captured_logs[0].body,
            include_str!("./snapshots/role_helpers.snap")
        );
    });
}

#[wasm_bindgen_test]
fn get_implicit_aria_roles_returns_expected_roles_for_various_dom_nodes() {
    let Setup {
        named_section,
        h1,
        unnamed_form,
        radio,
        input,
        ..
    } = setup();

    assert_eq!(
        get_implicit_aria_roles(&named_section),
        vec![AriaRoleDefinitionKey::Region]
    );
    assert_eq!(
        get_implicit_aria_roles(&h1),
        vec![AriaRoleDefinitionKey::Heading]
    );
    assert_eq!(get_implicit_aria_roles(&unnamed_form), vec![]);
    assert_eq!(
        get_implicit_aria_roles(&radio),
        vec![AriaRoleDefinitionKey::Radio]
    );
    assert_eq!(
        get_implicit_aria_roles(&input),
        vec![AriaRoleDefinitionKey::Textbox]
    );
}

#[wasm_bindgen_test]
fn is_inaccessible_should_return_expected() {
    let document = document();
    let cases = [
        ("<div />", false),
        ("<div aria-hidden=\"false\" />", false),
        ("<div style=\"visibility: visible\" />", false),
        ("<div hidden />", true),
        ("<div style=\"display: none;\"/>", true),
        ("<div style=\"visibility: hidden;\"/>", true),
        ("<div aria-hidden=\"true\" />", true),
    ];

    for (html, expected) in cases {
        // Chrome's `getComputedStyle` only works when appending the element to the document.
        let container = document
            .create_element("div")
            .expect("Element should be created.")
            .unchecked_into::<HtmlElement>();
        document
            .body()
            .expect("Document should have body.")
            .append_child(&container)
            .expect("Child should be appended.");

        let RenderReturn { container, .. } = render(html, Some(container));

        container
            .first_child()
            .expect("Element should have one child.")
            .append_child(
                &document
                    .create_element("button")
                    .expect("Element should be created."),
            )
            .expect("Child should be appended.");

        assert_eq!(
            is_inaccessible(
                &container
                    .query_selector("button")
                    .expect("Query should succeed.")
                    .expect("Query should return an element."),
            ),
            expected,
            "is_inaccessible for {html} returns {expected}",
        );

        container.remove();
    }
}
