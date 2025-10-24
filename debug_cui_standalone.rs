use saba_core::{browser::Browser, http::HttpResponse};
use std::{cell::RefCell, rc::Rc};

fn main() {
    println!("Starting browser engine debug test...");

    let browser = Browser::new();

    // Mock のHTTPレスポンス
    let html = r#"<html><head><title>Test</title></head><body><h1>Test Page</h1><p>Hello World!</p><input type="text" name="test" placeholder="Test Input" /></body></html>"#;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    );

    let http_response = match HttpResponse::new(response) {
        Ok(res) => {
            println!("Mock response created successfully");
            res
        }
        Err(e) => {
            println!("Failed to create response: {:?}", e);
            return;
        }
    };

    // ページでレスポンスを処理
    println!("Processing response with browser...");
    let page = browser.borrow().current_page();
    page.borrow_mut().receive_response(http_response);

    // DisplayItemの数を確認
    let display_items = page.borrow().display_items();
    println!("Final display items count: {}", display_items.len());

    for (i, item) in display_items.iter().enumerate() {
        println!("Display item {}: {:?}", i, item);
        if item.is_input() {
            println!("  *** Found input item! ***");
        }
    }

    // ログも確認
    let logs = browser.borrow().logs();
    println!("\nBrowser logs:");
    for log in logs {
        println!("  {}", log);
    }
}
