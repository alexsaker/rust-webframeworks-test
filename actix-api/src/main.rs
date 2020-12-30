use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn index() -> &'static str {
    "Welcome!"
}

async fn hello() -> &'static str {
    "Hello World!"
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn specific_hello(
    _req: HttpRequest,
    web::Path((name,)): web::Path<(String,)>,
) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
            .route("/hello/{name}", web::get().to(specific_hello))
            .route("/echo", web::post().to(echo))
    })
    .bind("127.0.0.1:8089")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Welcome!"##);

        Ok(())
    }

    #[actix_rt::test]
    async fn test_hello() -> Result<(), Error> {
        let app = App::new().route("/hello", web::get().to(hello));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Hello World!"##);

        Ok(())
    }

    #[actix_rt::test]
    async fn test_specific_hello() -> Result<(), Error> {
        let app = App::new().route("/hello/{name}", web::get().to(specific_hello));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/hello/John").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Hello John!"##);

        Ok(())
    }

    #[actix_rt::test]
    async fn test_echo() -> Result<(), Error> {
        let app = App::new().route("/echo", web::post().to(echo));
        let mut app = test::init_service(app).await;
        let payload = "toto";
        let req = test::TestRequest::post()
            .uri("/echo")
            .set_payload(payload)
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, payload);

        Ok(())
    }
}
