use super::server::Handler;
use crate::http::{Response, Request, StatusCode};
use std::fs;

pub struct WebsiteHandler{
    path: String,
}

impl WebsiteHandler{
    pub fn new(path:String) -> Self {
        WebsiteHandler{path}
    }

    fn read_file(&self, file_path:&str) -> Option<String> {
        let path = format!("{}/{}", self.path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Some one is trying to mess with our server. requested {}" file_path);
                    None
                }
            }
            Err(e) => None
        }

    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, self.read_file(request.path()))
    }
}