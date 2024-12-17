use hyper::{Body, Request, Response};

pub async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match req.uri().path() {
        "/" => Ok(Response::new(Body::from("Welcome to the Rust Web Server!"))),
        "/api" => Ok(Response::new(Body::from("This is the API endpoint."))),
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::from("404 Not Found"))
            .unwrap()),
    }
}
