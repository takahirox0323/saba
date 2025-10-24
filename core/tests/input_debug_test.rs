use saba_core::{
    browser::Browser,
    renderer::{
        css::cssom::StyleSheet,
        dom::node::{ElementKind, NodeKind},
        html::{parser::HtmlParser, token::HtmlTokenizer},
        layout::layout_view::LayoutView,
    },
};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_input_parsing_debug() {
    let browser = Browser::new();
    let html = r#"<html><body><h1>Test Page</h1><p>Hello World!</p><input type="text" name="username" placeholder="ユーザー名を入力してください" /></body></html>"#;
    let t = HtmlTokenizer::new(Rc::downgrade(&browser), html.to_string());
    let window = HtmlParser::new(Rc::downgrade(&browser), t).construct_tree();

    // DOMの構造をデバッグ出力
    println!("=== DOM構造デバッグ ===");
    debug_print_dom(window.borrow().document(), 0);

    let body = find_element_by_kind(window.borrow().document(), ElementKind::Body)
        .expect("body要素が見つかりません");

    println!("\n=== body要素の子要素 ===");
    let mut current = body.borrow().first_child();
    let mut index = 0;
    while let Some(child) = current {
        println!("子要素 {}: {:?}", index, child.borrow().kind());
        if let NodeKind::Element(element) = child.borrow().kind() {
            println!("  要素名: {}", element.kind());
            if element.kind() == ElementKind::Input {
                println!("  *** INPUT要素発見! ***");
                println!("  属性: {:?}", element.attributes());
            }
        }
        current = child.borrow().next_sibling();
        index += 1;
    }
}

fn debug_print_dom(node: Rc<RefCell<saba_core::renderer::dom::node::Node>>, depth: usize) {
    let indent = "  ".repeat(depth);
    match node.borrow().kind() {
        NodeKind::Document => println!("{}Document", indent),
        NodeKind::Element(element) => {
            println!("{}Element: {} (attributes: {:?})", indent, element.kind(), element.attributes());
        }
        NodeKind::Text(text) => println!("{}Text: {:?}", indent, text),
    }

    let mut current = node.borrow().first_child();
    while let Some(child) = current {
        debug_print_dom(child.clone(), depth + 1);
        current = child.borrow().next_sibling();
    }
}

fn find_element_by_kind(
    node: Rc<RefCell<saba_core::renderer::dom::node::Node>>,
    target_kind: ElementKind,
) -> Option<Rc<RefCell<saba_core::renderer::dom::node::Node>>> {
    if let NodeKind::Element(element) = node.borrow().kind() {
        if element.kind() == target_kind {
            return Some(node.clone());
        }
    }

    let mut current = node.borrow().first_child();
    while let Some(child) = current {
        if let Some(found) = find_element_by_kind(child.clone(), target_kind) {
            return Some(found);
        }
        current = child.borrow().next_sibling();
    }

    None
}

#[test]
fn test_layout_view_with_input() {
    let browser = Browser::new();
    let html = r#"<html><body><h1>Test Page</h1><p>Hello World!</p><input type="text" name="username" placeholder="ユーザー名を入力してください" /></body></html>"#;
    let t = HtmlTokenizer::new(Rc::downgrade(&browser), html.to_string());
    let window = HtmlParser::new(Rc::downgrade(&browser), t).construct_tree();

    let cssom = StyleSheet::new();
    let layout_view = LayoutView::new(Rc::downgrade(&browser), window.borrow().document(), &cssom);

    println!("\n=== LayoutView構築デバッグ ===");
    if let Some(_root) = layout_view.root() {
        println!("Layout root exists");
    } else {
        println!("No layout root found");
    }

    println!("\n=== DisplayItem生成デバッグ ===");
    let display_items = layout_view.paint();
    println!("Generated {} display items", display_items.len());

    for (i, item) in display_items.iter().enumerate() {
        println!("DisplayItem {}: {:?}", i, item);
        if item.is_input() {
            println!("  *** INPUT DisplayItem発見! ***");
        }
    }
}
