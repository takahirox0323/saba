extern crate alloc;

use saba_core::{
    browser::Browser,
    display_item::DisplayItem,
    http::HttpResponse,
};
use alloc::string::ToString;

#[test]
fn test_input_rendering_with_real_html() {
    // Simulate HTTP response with test.html content
    let html_content = r#"<html><body><h1>Test Page</h1><p>Hello World!</p><input type="text" name="username" placeholder="ユーザー名を入力してください" /></body></html>"#;

    let browser = Browser::new();

    // Create HTTP response in correct format
    let raw_response = format!(
        "HTTP/1.1 200 OK\n\n{}",
        html_content
    );

    let response = HttpResponse::new(raw_response).expect("Failed to create HTTP response");

    // Process the response
    {
        let page = browser.borrow().current_page();
        page.borrow_mut().receive_response(response);
    }

    // Get display items
    let display_items = browser.borrow().current_page().borrow().display_items();

    println!("Total display items: {}", display_items.len());
    for (i, item) in display_items.iter().enumerate() {
        println!("Item {}: {:?}", i, item);
    }

    // Check if input element is rendered
    let input_items: Vec<_> = display_items.iter()
        .filter(|item| matches!(item, DisplayItem::Input { .. }))
        .collect();

    assert!(!input_items.is_empty(), "No input display items found");

    // Verify the input properties
    if let DisplayItem::Input {
        input_type,
        name,
        placeholder,
        value: _,
        style: _,
        layout_point: _,
        layout_size: _,
    } = input_items[0] {
        assert_eq!(input_type, "text");
        assert_eq!(name, &Some("username".to_string()));
        assert_eq!(placeholder, &Some("ユーザー名を入力してください".to_string()));
    }

    println!("Input element successfully rendered! Display items count: {}", display_items.len());
    println!("Input display items count: {}", input_items.len());
}
