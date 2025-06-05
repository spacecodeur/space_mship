use httpserver::{Json, Router, get, run_http_server};
use services_manager::{get_service_port, ServicesName};
use serde::Serialize;

#[derive(Serialize)]
struct SimpleResponse {
    message: String,
}

async fn helloworld() -> Json<SimpleResponse> {
    // try me from a docker container inner the ${APP_NAME}-docker-network network
    // with commands :
    //       - curl --location --request GET 'http://service-web:34700'
    Json(SimpleResponse {
        message: "hello ! I'm the service web !".to_string(),
    })
}

fn main() {
    let port = get_service_port(ServicesName::WEB);
    let app = Router::new().route("/", get(helloworld));
    run_http_server(app, &port);
}