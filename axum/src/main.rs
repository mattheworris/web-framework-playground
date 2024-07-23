use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use tracing_subscriber;
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

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    // build our application with a route
    let app = Router::new()
        .route("/webhooks/account-service", post(post_webhook))
        .merge(SwaggerUi::new("/docs/swagger").url("/docs/openapi.json", ApiDoc::openapi()));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5555")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
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
