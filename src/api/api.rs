use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::json;
use rocket::Route;
// import macro to create response
use crate::responses::responses::Response; // Now Rust can find it
use crate::new_response;
use crate::schema::schema::Users;
use crate::schema::schema::create_user;

/*
#[get("/")]
fn index() -> Result<Json<Response>, Status> {
    //using macro to create a new response
    let response = new_response!(json!({"message": "Hello, world!"}));
    Ok(Json(response))
}
    */
    
#[post("/user/register")]
fn register() -> Result<Json<Response>, Status> {
    //using macro to create a new response
    let user = create_user("username".to_string(), "email".to_string(), "user_id".to_string(), "password".to_string());

    let response = new_response!(user.to_json());
    Ok(Json(response))
}

// Export the routes
pub fn api_routes() -> Vec<Route> {
    routes![register]
}