extern crate alloc;

use saba_core::{
    browser::Browser,
    display_item::DisplayItem,
    renderer::html::parser::HtmlParser,
    renderer::html::token::HtmlTokenizer,
    renderer::css::cssom::StyleSheet,
    renderer::layout::layout_view::LayoutView,
};
use alloc::rc::Rc;
use alloc::string::ToString;

#[test]
fn test_input_element_rendering() {
    let browser = Browser::new();
    let html = r#"<html>
<body>
<h1>Test Page</h1>
<p>Hello World!</p>
<input type="text" name="username" placeholder="ユーザー名を入力してください" />
</body>
</html>"#.to_string();

    let t = HtmlTokenizer::new(Rc::downgrade(&browser), html);
    let window = HtmlParser::new(Rc::downgrade(&browser), t).construct_tree();

    let cssom = StyleSheet::new();
    let layout_view = LayoutView::new(
        Rc::downgrade(&browser),
        window.borrow().document(),
        &cssom,
    );

    let display_items = layout_view.paint();

    // Check if there's an input display item
    let has_input = display_items.iter().any(|item| {
        matches!(item, DisplayItem::Input { .. })
    });

    assert!(has_input, "No input display item found");

    // Check the specific input display item
    for item in &display_items {
        if let DisplayItem::Input {
            input_type,
            name,
            placeholder,
            value: _,
            style: _,
            layout_point: _,
            layout_size: _,
        } = item {
            assert_eq!(input_type, "text");
            assert_eq!(name, &Some("username".to_string()));
            assert_eq!(placeholder, &Some("ユーザー名を入力してください".to_string()));
        }
    }
}
