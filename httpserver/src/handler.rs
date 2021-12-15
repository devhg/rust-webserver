use http::httprequest;
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let content = fs::read_to_string(full_path);
        content.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let httprequest::Resource::Path(s) = &req.resource;
        let routes: Vec<&str> = s.split("/").collect();
        match routes[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(content) => {
                    let mut map = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(content))
                }
                None => HttpResponse::new("404", None, None),
            },
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/orders.json", data_path);
        let content = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> = serde_json::from_str(content.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        let httprequest::Resource::Path(s) = &_req.resource;
        let routes: Vec<&str> = s.split("/").collect();
        // localhost:8000/api/shopping/orders
        match routes[2] {
            "shopping" if routes.len() > 2 && routes[3] == "orders" => {
                let body = serde_json::to_string(&Self::load_json()).unwrap();
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), Some(body))
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}
