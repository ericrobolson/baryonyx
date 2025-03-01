use std::collections::HashMap;

pub use ip_address;
pub use serde;
pub use serde_json;
pub use tokio;
pub use warp;
use warp::{Filter, filters::path::FullPath};

pub type Err = Box<dyn std::error::Error>;

#[derive(Clone)]
pub struct Page {
    path: Option<String>,
    content: String,
}
impl Page {
    pub fn new() -> Self {
        Self {
            path: None,
            content: "".into(),
        }
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
}

pub type Res = Result<ServerBuilder, Err>;

pub struct ServerBuilder {
    port: u16,
    is_https: bool,
    dev_mode: bool,
    content_limit: u64,
    pages: HashMap<String, Page>,
}
impl ServerBuilder {
    /// Create a new server builder.
    pub fn new() -> Self {
        Self {
            port: 3000,
            content_limit: 1024 * 1024 * 10, // 10MB
            is_https: false,
            dev_mode: true,
            pages: HashMap::new(),
        }
    }

    /// Set the port for the server.
    pub fn with_port(mut self, port: u16) -> Res {
        self.port = port;
        Ok(self)
    }

    /// Set the HTTPS flag for the server.
    pub fn with_https(mut self, is_https: bool) -> Res {
        self.is_https = is_https;
        Ok(self)
    }

    /// Set the content limit for requests to the server.
    pub fn with_content_limit(mut self, content_limit: u64) -> Res {
        self.content_limit = content_limit;
        Ok(self)
    }

    /// Add a page to the server.
    pub fn with_page(mut self, path: &str, page: Page) -> Res {
        if let Some(page) = self.pages.insert(path.to_string(), page) {
            return Err(format!("Page already exists: {}", path).into());
        }
        Ok(self)
    }

    /// Start the server.
    pub async fn start(self) -> Result<(), Err> {
        let server_ip = if self.dev_mode {
            let localhost = [127, 0, 0, 1];
            localhost.into()
        } else {
            let ip_details = ip_address::get_details(self.port, "/", self.is_https).unwrap();
            ip_details.ip
        };

        match self.pages.clone().get("/") {
            Some(page) => page.clone(),
            None => return Err("No page at '/' found".into()),
        };

        let pages = self.pages.clone();
        let root = warp::any()
            .and(warp::path::full())
            .and(warp::get())
            .map(move |route: FullPath| {
                println!("Route: {:?}", route);

                let path = pages.get(route.as_str());
                if let Some(page) = path {
                    page.content.clone()
                } else {
                    "Not Found".into()
                }
            })
            .with(warp::filters::compression::gzip());

        println!("Running server on http://{}:{}", server_ip, self.port);
        warp::serve(root).run((server_ip, self.port)).await;

        Ok(())
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
