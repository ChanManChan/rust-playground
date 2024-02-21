#![allow(dead_code)]

use std::env;

use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
 
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    server.run(WebsiteHandler::new(public_path));
}

// GET /user?id=10 HTTP/1.1\r\n
// HEADERS \r\n
// BODY