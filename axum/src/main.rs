use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use std::net::Ipv4Addr;
use types::WebhookCallback;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod types;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Account Service Webhook API",
        version = "0.1.0",
        description = "This is the account service Webhook API documentation",
    ),
    paths(post_webhook),
    components(schemas(
        WebhookCallback,
        types::SIWFSignup,
        types::HandleChanged,
        types::HandleCreated,
        types::KeyAdded
    ))
)]
struct ApiDoc;

const WEBHOOK_BASE_URL: &str = "/webhooks";
const WEBHOOK_ENDPOINT: &str = "/account-service";

#[tokio::main]
async fn main() {
    const HOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const PORT: u16 = 8888;

    println!(
        "Starting Webhook Server on http://{}:{}{}{}",
        HOST, PORT, WEBHOOK_BASE_URL, WEBHOOK_ENDPOINT
    );
    println!(
        "OpenAPI Spec available at http://{}:{}/docs/openapi.json",
        HOST, PORT
    );
    println!(
        "Swagger UI available at http://{}:{}/docs/swagger",
        HOST, PORT
    );

    // build our application with a route
    let app = Router::new()
        .route("/webhooks/account-service", post(post_webhook))
        .merge(SwaggerUi::new("/docs/swagger").url("/docs/openapi.json", ApiDoc::openapi()));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind((HOST, PORT)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    post,
    path = "/webhooks/account-service",
    request_body = WebhookCallback,
    responses(
        (status = 200, description = "Webhook received", body = WebhookCallback),
        (status = NOT_FOUND, description = "Webhook not found")
    )
)]
async fn post_webhook(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<WebhookCallback>,
) -> impl IntoResponse {
    // insert your application logic here
    tracing::info!("Received webhook: {:?}", payload);
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    println!("Received webhook: {:?}", payload);
    (StatusCode::CREATED, Json(()))
}
