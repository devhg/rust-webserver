use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = parse_request_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(": ") {
                let (key, value) = parse_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.is_empty() {
                continue;
            } else {
                parsed_msg_body = line;
            }
        }
        // let msg_body: String = lines.join("\r\n");
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn parse_request_line(s: &str) -> (Method, Resource, Version) {
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

fn parse_header_line(s: &str) -> (String, String) {
    let mut words = s.split(": ");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = words.next() {
        key = k.to_string();
    }
    if let Some(v) = words.next() {
        value = v.to_string();
    }
    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let get: Method = "GET".into();
        assert_eq!(get, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let get: Version = "HTTP/1.1".into();
        assert_eq!(get, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let req =
            "GET /geetings HTTP/1.1\r\nHost: localhost:8000\r\n\\r\nHello, world!".to_string();

        let mut header_expected = HashMap::new();
        header_expected.insert("Host".to_string(), "localhost:8000".to_string());
        let http_req: HttpRequest = req.into();

        assert_eq!(http_req.method, Method::Get);
        assert_eq!(http_req.version, Version::V1_1);
        assert_eq!(http_req.resource, Resource::Path("/geetings".to_string()));
        assert_eq!(http_req.headers, header_expected);
    }
}
