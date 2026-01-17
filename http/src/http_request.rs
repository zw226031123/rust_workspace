use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    Uninitialized,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            _ => HttpMethod::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20,
    Uninitialized,
}
impl From<&str> for HttpVersion {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.0" => HttpVersion::Http10,
            "HTTP/1.1" => HttpVersion::Http11,
            "HTTP/2.0" => HttpVersion::Http20,
            _ => HttpVersion::Uninitialized,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub version: HttpVersion,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}
fn process_req_line(s: &str) -> (HttpMethod, Resource, HttpVersion) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}
fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }
    (key, value)
}
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parse_method = HttpMethod::Uninitialized;
        let mut parse_version = HttpVersion::Uninitialized;
        let mut parse_resource = Resource::Path("".to_string());
        let mut parse_headers = HashMap::new();
        let mut parse_body = "";
        for line in req.lines() {
            if line.contains("HTTP/1.1") {
                let (method, resource, version) = process_req_line(line);
                parse_method = method;
                parse_resource = resource;
                parse_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parse_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parse_body = line;
            }
        }
        HttpRequest {
            method: parse_method,
            resource: parse_resource,
            version: parse_version,
            headers: parse_headers,
            body: parse_body.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_http_method_into() {
        let m: HttpMethod = "GET".into();
        assert_eq!(m, HttpMethod::GET);
    }
    #[test]
    fn test_http_version_into() {
        let v: HttpVersion = "HTTP/1.1".into();
        assert_eq!(v, HttpVersion::Http11);
    }
    #[test]
    fn test_read_http() {
        // 测试完整 HTTP 请求解析
        let http_request_str = String::from(
            "GET /index.html HTTP/1.1\r\n\
                                Host: localhost:8080\r\n\
                                Content-Type: application/json\r\n\
                                \r\n\
                                {\"key\": \"value\"}",
        );

        let request: HttpRequest = http_request_str.into();

        // 验证请求行解析
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.resource, Resource::Path("/index.html".to_string()));
        assert_eq!(request.version, HttpVersion::Http11);

        // 验证头部解析
        println!("{:#?}", request.headers);
        assert_eq!(request.headers.len(), 2);
        assert_eq!(
            request.headers.get("Host"),
            Some(&"localhost:8080".to_string())
        );
        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );

        // 验证消息体解析
        assert_eq!(request.body, "{\"key\": \"value\"}");
    }
}
