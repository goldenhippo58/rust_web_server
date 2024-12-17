use crate::config::Config;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

pub async fn start_server(config: Config) {
    // Parse server address and port
    let addr = format!("{}:{}", config.address, config.port)
        .parse()
        .expect("Invalid server address");

    // Create the service
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle_request)) });

    // Start the server
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Await the server future
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

// Handle incoming requests
async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("Hello, Rust Web Server!")))
}
