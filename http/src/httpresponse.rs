use std::{collections::HashMap, io::Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(resp: HttpResponse) -> Self {
        let res = resp.clone();
        let body = &res.get_body();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res.get_version(),
            &res.get_status_code(),
            &res.get_status_text(),
            &res.get_headers(),
            body.len(),
            body,
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response: HttpResponse<'a> = Self::default();

        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Unknown".into(),
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, stream: &mut impl Write) -> Result<(), ()> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(stream, "{}", response_string);
        Ok(())
    }

    pub fn get_version(&self) -> &str {
        self.version
    }

    pub fn get_status_code(&self) -> &str {
        self.status_code
    }

    pub fn get_status_text(&self) -> &str {
        self.status_text
    }

    pub fn get_headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            // todo ???
            header_string.push_str(format!("{}: {}\r\n", k, v).as_str());
        }
        header_string
    }

    pub fn get_body(&self) -> &str {
        match &self.body {
            Some(body) => body.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));
        let response_expect = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expect);
    }

    #[test]
    fn test_resource_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxxx".into()));
        let response_expect = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expect);
    }

    #[test]
    fn test_http_resource_creation() {
        let response_expect = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        let http_string: String = response_expect.into();
        let actual_string =
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 4\r\n\r\nxxxx";
        assert_eq!(actual_string, http_string);
    }
}
