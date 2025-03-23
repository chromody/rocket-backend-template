use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::json;
use rocket::Route;

use rocket::State;
use mongodb::{Collection};
use mongodb::bson::{doc, from_bson};

use futures::stream::TryStreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

use regex::Regex;

// import macro to create response
use crate::responses::responses::Response; // Now Rust can find it
use crate::new_response;
use crate::schema::schema::{User, UserError};
#[macro_use]
use crate::get_user_field;
use crate::db::db::Database;

#[get("/user")]
async fn info() -> Result<Json<Response>, (Status, Json<Response>)> {
    let user = User::register("Username".to_string(), "Email@Email.com".to_string(), Some("67df84c0386ceba9b9b3bc99".to_string()), "Password".to_string());
    match user {
        Ok(user) => {
            let response = new_response!(user.to_json());
            Ok(Json(response))
        }
        Err(UserError::InvalidEmail) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        new_response!()
                    )
                )
            );
        }
        Err(UserError::InvalidPassword) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        new_response!()
                    )
                )
            );
        }
        Err(UserError::InvalidUsername) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        new_response!()
                    )
                )
            );
        }
        Err(_) => {
            return Err(
                (
                    Status::InternalServerError,
                    Json(
                        new_response!()
                    )
                )
            );
        }
    }
}

#[post("/user", data = "<register_data>")]
async fn register(db: &State<Database>, register_data: Json<User>) -> Result<Json<Response>, (Status, Json<Response>)> {
    let collection: Collection<User> = db.client.lock().await.database("rocket-template").collection("users");

    //using macro to create a new response
    let user: User;
    let user_result: Result<User, UserError> = User::register(get_user_field!(register_data, username), get_user_field!(register_data, email), None, get_user_field!(register_data, password));
    match user_result {
        Ok(user_unwrapped) =>{
            user = user_unwrapped;
        } //checking for each type of error
        Err(UserError::InvalidEmail) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        new_response!(
                            json!(
                                {
                                    "error": "Invalid email",
                                    "email": get_user_field!(register_data, email)
                                }
                            )
                        )
                    )
                )
            );
        }
        Err(UserError::InvalidPassword) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        new_response!(
                            json!(
                                {
                                    "error": "Invalid password",
                                    "password": get_user_field!(register_data, password)
                                }
                            )
                        )
                    )
                )
            );
        }
        Err(UserError::InvalidUsername) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        new_response!(
                            json!(
                                {
                                    "error": "Invalid username",
                                    "username": get_user_field!(register_data, username)
                                }
                            )
                        )
                    )
                )
            );
        }
        Err(_) => {
            return Err(
                (
                    Status::InternalServerError,
                    Json(
                        new_response!(
                            json!(
                                {
                                    "error": "Internal server error"
                                }
                            )
                        )
                    )
                )
            );
        }
    }

    if collection.insert_one(&user).await.is_ok() {
        let response = new_response!(user.to_json());
        Ok(Json(response))
    } else {
        Err((
            Status::InternalServerError,
            Json(
                new_response!(
                    json!(
                        {
                            "error": "Internal server error",
                        }
                    )
                )
            )
        ))
    }
}

#[post("/login", data = "<login_data>")]
async fn login(db: &State<Database>, login_data: Json<serde_json::Value>) -> Result<Json<Response>, (Status, Json<Response>)> {
    let username = login_data.get("username").and_then(|v| v.as_str()).unwrap_or("");
    let password = login_data.get("password").and_then(|v| v.as_str()).unwrap_or("");
    
    if username.is_empty() || password.is_empty() {
        return Err((Status::BadRequest, Json(new_response!(json!({ "error": "Missing username or password" })))));
    }

    let collection = db.client.lock().await.database("rocket-template").collection("users");
    match collection.find_one(doc! { "username": username }).await {
        Ok(Some(doc)) => {
            if let Ok(user) = from_bson::<User>(doc) {
                if user.check_password(password) {
                    return Ok(Json(new_response!(user.to_json())));
                }
            }
            Err((Status::BadRequest, Json(new_response!(json!({ "error": "Invalid password" })))))
        }
        Ok(None) => Err((Status::BadRequest, Json(new_response!(json!({ "error": "User not found" }))))),
        Err(_) => Err((Status::InternalServerError, Json(new_response!(json!({ "error": "Internal server error" }))))),
    }
}



// Export the routes
pub fn api_routes() -> Vec<Route> {
    routes![info, register, login]
}