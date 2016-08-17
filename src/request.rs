
struct Request {
    url: &str,
    query: &str,
    format: &str
}

trait Build {
    fn send(&self);
}

impl Build for Request {
    fn new(url: &str, query: &str, format: &str) -> Request {
        Request {
            url: url,
            query: query,
            format: format
        }
    }

    fn send(&self) {
        
    }
}
