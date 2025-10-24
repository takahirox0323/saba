use saba_core::{browser::Browser, http::HttpResponse};

#[test]
fn test_browser_integration_debug() {
    println!("Testing browser engine with input tag...");

    let browser = Browser::new();

    // test_simple.htmlと同じ内容
    let html = r#"<html>
  <head><title>Input Test</title></head>
  <body>
    <h1>Input Test Page</h1>
    <input type="text" name="test" placeholder="Test Input" />
    <p>End of page</p>
  </body>
</html>"#;

    let response_text = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    );

    println!("Creating HttpResponse...");
    let http_response = match HttpResponse::new(response_text) {
        Ok(res) => {
            println!("HTTP response created successfully");
            res
        }
        Err(e) => {
            println!("Failed to create response: {:?}", e);
            return;
        }
    };

    println!("Processing response with browser engine...");
    let page = browser.borrow().current_page();
    page.borrow_mut().receive_response(http_response);

    println!("=== Final Results ===");
    let display_items = page.borrow().display_items();
    println!("Display items count: {}", display_items.len());

    for (i, item) in display_items.iter().enumerate() {
        match item {
            saba_core::display_item::DisplayItem::Input { input_type, name, placeholder, .. } => {
                println!("DisplayItem[{}]: INPUT (type={}, name={:?}, placeholder={:?})",
                    i, input_type, name, placeholder);
            }
            saba_core::display_item::DisplayItem::Text { text, .. } => {
                println!("DisplayItem[{}]: TEXT ({})", i, text);
            }
            saba_core::display_item::DisplayItem::Rect { .. } => {
                println!("DisplayItem[{}]: RECT", i);
            }
            saba_core::display_item::DisplayItem::Img { .. } => {
                println!("DisplayItem[{}]: IMG", i);
            }
        }
    }

    println!("\n=== Browser Logs ===");
    let logs = browser.borrow().logs();
    for log in logs {
        println!("{:?}", log);
    }
}
