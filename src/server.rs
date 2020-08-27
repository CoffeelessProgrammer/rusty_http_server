pub struct HttpServer {
    addr: String
}

impl HttpServer {
    // Rust Best Practice: constructors are named 'new'
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
    }
}