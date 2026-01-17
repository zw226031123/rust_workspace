use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::http_request::{HttpMethod, HttpRequest, Resource};
use http::http_response::HttpResponse;
use std::io::Write;

pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            HttpMethod::GET => match &req.resource {
                Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            HttpMethod::POST => {}
            HttpMethod::Uninitialized => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
