fn main() {
    let http_server = Server::new("127.0.0.1:8080");
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
    }
}

// --------------------------- Terminal Output: ---------------------------
// $ cargo build
//    Compiling rusty_http_server v0.1.0 (C:\Users\Coffeeless\Documents\Sandboxes\RustPlayground\RustyHttpServer)
// error[E0308]: mismatched types
//  --> src\main.rs:2:35
//   |
// 2 |     let http_server = Server::new("127.0.0.1:8080");
//   |                                   ^^^^^^^^^^^^^^^^
//   |                                   |
//   |                                   expected struct `std::string::String`, found `&str`
//   |                                   help: try using a conversion method: `"127.0.0.1:8080".to_string()`
// 
// error: aborting due to previous error