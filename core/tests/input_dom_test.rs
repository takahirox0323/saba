extern crate alloc;

use saba_core::{
    browser::Browser,
    renderer::html::parser::HtmlParser,
    renderer::html::token::HtmlTokenizer,
    renderer::dom::node::{NodeKind, ElementKind},
};
use alloc::string::ToString;

fn print_dom_tree(node: &Option<alloc::rc::Rc<core::cell::RefCell<saba_core::renderer::dom::node::Node>>>, depth: usize) {
    if let Some(n) = node {
        let indent = "  ".repeat(depth);
        match &n.borrow().kind() {
            NodeKind::Document => println!("{}Document", indent),
            NodeKind::Element(e) => println!("{}Element: {}", indent, e.kind()),
            NodeKind::Text(t) => println!("{}Text: '{}'", indent, t),
        }

        print_dom_tree(&n.borrow().first_child(), depth + 1);
        print_dom_tree(&n.borrow().next_sibling(), depth);
    }
}

#[test]
fn test_input_dom_structure() {
    let html_content = r#"<html><body><h1>Test Page</h1><p>Hello World!</p><input type="text" name="username" placeholder="ユーザー名を入力してください" /></body></html>"#;

    let browser = Browser::new();
    let t = HtmlTokenizer::new(alloc::rc::Rc::downgrade(&browser), html_content.to_string());
    let window = HtmlParser::new(alloc::rc::Rc::downgrade(&browser), t).construct_tree();

    println!("DOM Structure:");
    print_dom_tree(&Some(window.borrow().document()), 0);

    // Check if input element exists in DOM
    fn find_input_in_dom(node: &Option<alloc::rc::Rc<core::cell::RefCell<saba_core::renderer::dom::node::Node>>>) -> bool {
        if let Some(n) = node {
            if let NodeKind::Element(e) = &n.borrow().kind() {
                if e.kind() == ElementKind::Input {
                    println!("Found input element with attributes:");
                    for attr in &e.attributes() {
                        println!("  {}: {}", attr.name(), attr.value());
                    }
                    return true;
                }
            }

            if find_input_in_dom(&n.borrow().first_child()) {
                return true;
            }
            if find_input_in_dom(&n.borrow().next_sibling()) {
                return true;
            }
        }
        false
    }

    let has_input = find_input_in_dom(&Some(window.borrow().document()));
    assert!(has_input, "Input element not found in DOM");
}
