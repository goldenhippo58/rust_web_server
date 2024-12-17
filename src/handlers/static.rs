use hyper::{Body, Request, Response};
use std::fs;

pub async fn static_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = format!("./public{}", req.uri().path());
    if let Ok(contents) = fs::read_to_string(&path) {
        Ok(Response::new(Body::from(contents)))
    } else {
        Ok(Response::builder()
            .status(404)
            .body(Body::from("File not found"))
            .unwrap())
    }
}
