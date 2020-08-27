use super::method::HttpMethod;

pub struct HttpRequest {
    path: String,
    query_string: Option<String>,
    // http_method: crate::http::method::HttpMethod
    // http_method: super::method::HttpMethod  // alternative to above
    http_method: HttpMethod
}