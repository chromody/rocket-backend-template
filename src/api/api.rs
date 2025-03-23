use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::json;
use rocket::Route;

use rocket::State;
use mongodb::{bson::doc, Collection};
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


/*
#[get("/")]
fn index() -> Result<Json<Response>, Status> {
    //using macro to create a new response
    let response = new_response!(json!({"message": "Hello, world!"}));
    Ok(Json(response))
}
    */
    

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

// Export the routes
pub fn api_routes() -> Vec<Route> {
    routes![info, register]
}