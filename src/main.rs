use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Health {
    status: String,
}

async fn current_health() -> impl Responder {
    HttpResponse::Ok().json(Health { status: "OK".to_string() })
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let server_port = env::var("SERVER_PORT").unwrap_or("9090".to_string());
    HttpServer::new(|| {
        App::new()
            .route("/health-check", web::get().to(current_health))
    })
        .bind(format!("127.0.0.1:{}", server_port))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_health_check() -> Result<(), Error> {
        let mut app = test::init_service(
            App::new().service(web::resource("/").route(web::get().to(current_health))),
        ).await;

        let req = test::TestRequest::get()
            .uri("/")
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        let response_obj = serde_json::from_slice::<Health>(&response_body)?;
        assert_eq!(response_obj.status, "OK");

        Ok(())
    }
}
