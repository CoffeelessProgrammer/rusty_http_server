// RUN in Debugger:   gdb target/debug/rusty_http_server

fn main() {
    let get = HttpMethod::GET;
    let post = HttpMethod::POST;
    let put = HttpMethod::PUT;
    let delete = HttpMethod::DELETE;

    let http_server = Server::new("127.0.0.1:8080".to_string());
    http_server.run();
}

struct Server {
    addr: String
}

impl Server {
    // Rust Best Practice: constructors are named 'new'
    fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    http_method: HttpMethod
}

enum HttpMethod {
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

/* GET /user?id=10 HTTP/1.1\r\n
 * HEADERS \r\n
 * BODY
 */

// --------------------------- Terminal Output: ---------------------------
// Listening on 127.0.0.1:8080