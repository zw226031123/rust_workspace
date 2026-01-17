use http::http_request::HttpRequest;
use http::http_response::HttpResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs};

pub struct PageNotFoundHandler;
pub struct StaticPageHandler;
pub struct WebServiceHandler;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse<'_>;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let content = fs::read_to_string(full_path);
        content.ok()
    }
}

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse<'_> {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}
impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse<'_> {
        let http::http_request::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("200.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("200.html")),
            path => match Self::load_file(path) {
                Some(content) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(content))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}
#[derive(Deserialize, Serialize)]
pub struct OrderStatus {
    id: i32,
    date: String,
    status: String,
}
impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_content = fs::read_to_string(full_path);
        let orders = serde_json::from_str(json_content.unwrap().as_str()).unwrap();
        orders
    }
}
impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse<'_> {
        let http::http_request::Resource::Path(p) = &req.resource;
        let route: Vec<_> = p.split('/').collect();
        if route.len() <= 2 {
            return HttpResponse::new("404", None, Self::load_file("404.html"));
        }
        match route[2] {
            "shopping" if route.len() > 2 && route[3] == "orders" => {
                let body = serde_json::to_string(&Self::load_json()).ok();
                let mut map = HashMap::new();
                map.insert("Content-type", "application/json");
                HttpResponse::new("200", Some(map), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}
