use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use models::User;
use serde_json;
async fn hello(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let json = r#"
        {
          "email": "another@example.com",
          "username":"Alex",
          "age": 40
        }
    "#;

    let user: User = serde_json::from_str(json).unwrap();
    
    let hello = "Hello";
    let body = format!("{},{}", hello, user.username);
    Ok(Response::new(Body::from(body)))
}


#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
