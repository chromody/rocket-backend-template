use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::json;
use rocket::Route;
// import macro to create response
use crate::responses::responses::Response; // Now Rust can find it
use crate::new_response;

#[get("/")]
fn index() -> Result<Json<Response>, Status> {
    //using macro to create a new response
    let response = new_response!(json!({"message": "Hello, world!"}));
    Ok(Json(response))
}

// Export the routes
pub fn api_routes() -> Vec<Route> {
    routes![index]
}