// #[path = "./http/server.rs"] mod server;
mod server;
mod http;

use server::HttpServer;
// use http::request::HttpRequest;     // adding to namespace via 'mod' from http/mod.rs
// use http::method::HttpMethod;       // adding to namespace via 'mod' from http/mod.rs
use http::HttpMethod;                   // adding to namespace via 'use' from http/mod.rs
use http::HttpRequest;                  // adding to namespace via 'use' from http/mod.rs


fn main() {
    let get = HttpMethod::GET;
    let post = HttpMethod::POST;
    let put = HttpMethod::PUT;
    let delete = HttpMethod::DELETE;

    // let http_server = server::HttpServer::new("127.0.0.1:8080".to_string());
    let http_server = HttpServer::new("127.0.0.1:8080".to_string());
    http_server.run();
}

// --------- Previous Layout ---------
// mod server {}

// mod http {
//     pub mod request {
//     }

//     pub mod method {
//     }
// }

/* GET /user?id=10 HTTP/1.1\r\n
 * HEADERS \r\n
 * BODY
 */


// RUN in Debugger:   gdb target/debug/rusty_http_server
// Debugger not working. TO DO: Debug the debugger...

// --------------------------- Terminal Output: ---------------------------
// Listening on 127.0.0.1:8080