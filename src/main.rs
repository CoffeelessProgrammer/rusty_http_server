use http::request::HttpRequest;
use server::HttpServer;

fn main() {
    let get = http::method::HttpMethod::GET;
    let post = http::method::HttpMethod::POST;
    let put = http::method::HttpMethod::PUT;
    let delete = http::method::HttpMethod::DELETE;

    // let http_server = server::HttpServer::new("127.0.0.1:8080".to_string());
    let http_server = HttpServer::new("127.0.0.1:8080".to_string());
    http_server.run();
}
mod server {
    pub struct HttpServer {
        addr: String
    }
    
    impl HttpServer {
        // Rust Best Practice: constructors are named 'new'
        pub fn new(addr: String) -> Self {
            HttpServer {
                addr
            }
        }
    
        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {
    pub mod request {
        use super::method::HttpMethod;

        pub struct HttpRequest {
            path: String,
            query_string: Option<String>,
            // http_method: crate::http::method::HttpMethod
            // http_method: super::method::HttpMethod  // alternative to above
            http_method: HttpMethod
        }
    }

    pub mod method {
        pub enum HttpMethod {
            GET,
            HEAD,
            POST,
            PUT,
            PATCH,
            DELETE,
            CONNECT,
            OPTIONS,
            TRACE
        }
    }
}

/* GET /user?id=10 HTTP/1.1\r\n
 * HEADERS \r\n
 * BODY
 */


// RUN in Debugger:   gdb target/debug/rusty_http_server

// --------------------------- Terminal Output: ---------------------------
// Listening on 127.0.0.1:8080