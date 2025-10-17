extern crate alloc;

use net_std as net;
use ui_cui as ui;

use alloc::rc::Rc;
use alloc::string::String;
use core::cell::RefCell;
use net::http::HttpClient;
use saba_core::browser::Browser;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use saba_core::url::Url;
use ui::app::Tui;

fn handle_url(url: String) -> Result<HttpResponse, Error> {
    println!("handle_url called with: {}", url);

    // parse url
    let parsed_url = match Url::new(url.to_string()).parse() {
        Ok(url) => {
            println!("URL parsed successfully: host={}, port={}, path={}",
                     url.host(), url.port(), url.path());
            url
        }
        Err(e) => {
            let error_msg = format!("Failed to parse URL: {:?}", e);
            println!("{}", error_msg);
            return Err(Error::UnexpectedInput(error_msg));
        }
    };

    // send a HTTP request and get a response
    println!("Sending HTTP request to {}:{}{}...",
             parsed_url.host(), parsed_url.port(), parsed_url.path());
    let client = HttpClient::new();
    let response = match client.get(
        parsed_url.host(),
        parsed_url
            .port()
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("port number should be u16 but got {}", parsed_url.port())),
        parsed_url.path(),
    ) {
        Ok(res) => {
            println!("Received response with status code: {}", res.status_code());
            // redirect to Location
            if res.status_code() == 302 {
                let location = match res.header_value("Location") {
                    Ok(value) => {
                        println!("Redirecting to: {}", value);
                        value
                    }
                    Err(_) => return Ok(res),
                };
                let redirect_parsed_url = Url::new(location);

                let redirect_client = HttpClient::new();
                match redirect_client.get(
                    redirect_parsed_url.host(),
                    redirect_parsed_url
                        .port()
                        .parse::<u16>()
                        .unwrap_or_else(|_| {
                            panic!("port number should be u16 but got {}", parsed_url.port())
                        }),
                    redirect_parsed_url.path(),
                ) {
                    Ok(res) => {
                        println!("Redirect response received with status code: {}", res.status_code());
                        res
                    }
                    Err(e) => {
                        let error_msg = format!("Redirect request failed: {:?}", e);
                        println!("{}", error_msg);
                        return Err(Error::Network(error_msg));
                    }
                }
            } else {
                res
            }
        }
        Err(e) => {
            let error_msg = format!("Failed to get HTTP response: {:?}", e);
            println!("{}", error_msg);
            return Err(Error::Network(error_msg));
        }
    };

    println!("HTTP request completed successfully");
    Ok(response)
}

fn main() {
    // initialize the main browesr struct
    let browser = Browser::new();

    // initialize the UI object
    let ui = Rc::new(RefCell::new(Tui::new(browser)));

    match ui.borrow_mut().start(handle_url) {
        Ok(_) => {}
        Err(e) => {
            println!("browser fails to start {:?}", e);
        }
    };
}
