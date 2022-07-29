use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;


// query string = a=1&b=2&c&d=&e===&d=7&d=abc
fn main() {
    let svr = Server::new("127.0.0.1:8080".to_string());
    svr.run(WebsiteHandler::new("C:\\Users\\Aamir Ali\\IdeaProjects\\NewServer\\src\\htdocs".to_string()));
}

