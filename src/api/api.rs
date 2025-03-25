use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::json;
use rocket::Route;
use mongodb::bson::{oid::ObjectId};

use rocket::State;
use mongodb::{Collection};
use mongodb::bson::{doc, from_bson};

// import macro to create response
use crate::responses::network_responses::{NetworkResponse, ResponseBody, Response}; // Now Rust can find it
use crate::responses::user_responses::UserError; // Now Rust can find it
use crate::schema::user_schema::User;
use crate::get_user_field;
use crate::schema::request_schema::LoginRequest;
use crate::db::db::Database;
use crate::schema::jwt_schema::{JWT, create_jwt};

#[get("/user")]
async fn info(db: &State<Database>, key: Result<JWT, NetworkResponse>) -> Result<Json<Response>, NetworkResponse> {
    let key = key?;

    // Since sub_id is a String, pass it directly as a reference
    let user_id = &key.claims.sub_id;

    let collection = db.client.lock().await.database("rocket-template").collection::<User>("users");

    match collection.find_one(doc! { "_id": user_id }).await {
        Ok(Some(user)) => {
            let response = Response { body: ResponseBody::User(user) };
            Ok(Json(response))
        }
        Ok(None) => Err(NetworkResponse::NotFound("User not found".to_string())),
        Err(_) => Err(NetworkResponse::Conflict("Internal Server Error".to_string())),
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
                        Response { body: ResponseBody::Message("Invalid Email".to_string()) }
                    )
                )
            );
        }
        Err(UserError::InvalidPassword) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        Response { body: ResponseBody::Message("Invalid Password".to_string()) }
                    )
                )
            );
        }
        Err(UserError::InvalidUsername) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        Response { body: ResponseBody::Message("Invalid Username".to_string()) }
                    )
                )
            );
        }
        Err(_) => {
            return Err(
                (
                    Status::InternalServerError,
                    Json(
                        Response { body: ResponseBody::Message("Error".to_string()) }
                    )
                )
            );
        }
    }

    if collection.insert_one(&user).await.is_ok() {
        let response = Response { body: ResponseBody::User(user) };
        Ok(Json(response))
    } else {
        Err((
            Status::InternalServerError,
            Json(
                Response { body: ResponseBody::Message("Error".to_string()) }
            )
        ))
    }
}

#[post("/login", data = "<login_data>")]
async fn login(db: &State<Database>, login_data: Json<LoginRequest>) -> Result<Json<Response>, NetworkResponse> {
    let username = &login_data.username;
    let password = &login_data.password;

    let collection = db.client.lock().await.database("rocket-template").collection("users");
    match collection.find_one(doc! { "username": username }).await {
        Ok(Some(doc)) => {
            if let Ok(user) = from_bson::<User>(doc) {
                if user.check_password(password) {
                    match create_jwt(user.get_id().unwrap(), user.get_username()) {
                        Ok(token) => {
                            return Ok(Json(
                                Response { body: ResponseBody::Message(token) }
                            ));
                        },
                        Err(err) => {
                            return Err(NetworkResponse::BadRequest(err.to_string()));
                        }
                    }
                    
                }
            }
            return Err(NetworkResponse::BadRequest("Invalid password".to_string()));
        }
        Ok(None) => Err(NetworkResponse::BadRequest("User not found".to_string())),
        Err(_) => Err(NetworkResponse::Conflict("Internal Service Error".to_string())),
    }
}



// Export the routes
pub fn api_routes() -> Vec<Route> {
    routes![info, register, login]
}