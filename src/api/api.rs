use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::json;
use rocket::Route;

use rocket::State;
use mongodb::{bson::doc, Collection};
use futures::stream::TryStreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

// import macro to create response
use crate::responses::responses::Response; // Now Rust can find it
use crate::new_response;
use crate::schema::schema::User;
use crate::schema::schema::create_user;
use crate::db::db::Database;


/*
#[get("/")]
fn index() -> Result<Json<Response>, Status> {
    //using macro to create a new response
    let response = new_response!(json!({"message": "Hello, world!"}));
    Ok(Json(response))
}
    */
    
#[post("/user/register")]
async fn register(db: &State<Database>) -> Result<Json<Response>, Status> {
    let collection: Collection<User> = db.client.lock().await.database("rocket-template").collection("users");

    //using macro to create a new response
    let user = create_user("username".to_string(), "email".to_string(), "user_id".to_string(), "password".to_string());
    if collection.insert_one(&user).await.is_ok() {
        let response = new_response!(user.to_json());
        Ok(Json(response))
    } else {
        Err(Status::InternalServerError)
    }
}

// Export the routes
pub fn api_routes() -> Vec<Route> {
    routes![register]
}