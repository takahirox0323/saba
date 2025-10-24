use saba_core::{
    browser::Browser,
    renderer::{
        css::cssom::StyleSheet,
        dom::node::{NodeKind},
        html::{parser::HtmlParser, token::HtmlTokenizer},
        layout::layout_view::LayoutView,
    },
};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_input_file_content_debug() {
    let browser = Browser::new();

    // test.htmlと同じ内容をテスト
    let html = r#"<html>
  <body>
    <h1>Test Page</h1>
    <p>Hello World!</p>
    <input type="text" name="username" placeholder="ユーザー名を入力してください" />
  </body>
</html>"#;

    println!("処理するHTML: {}", html);

    let t = HtmlTokenizer::new(Rc::downgrade(&browser), html.to_string());
    let window = HtmlParser::new(Rc::downgrade(&browser), t).construct_tree();

    // DOMを確認
    println!("\n=== DOM構造確認 ===");
    debug_print_dom(window.borrow().document(), 0);

    let cssom = StyleSheet::new();
    let layout_view = LayoutView::new(Rc::downgrade(&browser), window.borrow().document(), &cssom);

    println!("\n=== LayoutView.paint()結果 ===");
    let display_items = layout_view.paint();
    println!("DisplayItem総数: {}", display_items.len());

    for (i, item) in display_items.iter().enumerate() {
        println!("DisplayItem[{}]: {:?}", i, item);

        if item.is_input() {
            println!("  *** INPUT要素がDisplayItemに正常に変換されました！ ***");
        }
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
