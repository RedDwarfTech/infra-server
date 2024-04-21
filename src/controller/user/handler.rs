use actix_web::{get, HttpRequest};

/// Root Endpoint
///
/// Hello World Example
#[utoipa::path(
    context_path = "/v1",
    path = "/",
    responses(
        (status = 200, description = "Hello World!")
    )
)]
#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}
