use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::json;
use rocket::Route;

use rocket::State;
use mongodb::{Collection};
use mongodb::bson::{doc, from_bson};

// import macro to create response
use crate::responses::network_responses::{NetworkResponse, ResponseBody}; // Now Rust can find it
use crate::responses::user_responses::UserError; // Now Rust can find it
use crate::schema::user_schema::User;
use crate::get_user_field;
use crate::schema::request_schema::LoginRequest;
use crate::db::db::Database;

#[get("/user")]
async fn info() -> Result<Json<NetworkResponse>, (Status, Json<NetworkResponse>)> {
    let user = User::register("Username".to_string(), "Email@Email.com".to_string(), Some("67df84c0386ceba9b9b3bc99".to_string()), "Password".to_string());
    match user {
        Ok(user) => {
            let response = NetworkResponse { body: ResponseBody::Data(user.to_json()) };
            Ok(Json(response))
        }
        Err(UserError::InvalidEmail) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        NetworkResponse { body: ResponseBody::Message("Error".to_string()) }
                    )
                )
            );
        }
        Err(UserError::InvalidPassword) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        NetworkResponse { body: ResponseBody::Message("Error".to_string()) }
                    )
                )
            );
        }
        Err(UserError::InvalidUsername) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        NetworkResponse { body: ResponseBody::Message("Error".to_string()) }
                    )
                )
            );
        }
        Err(_) => {
            return Err(
                (
                    Status::InternalServerError,
                    Json(
                        NetworkResponse { body: ResponseBody::Message("Error".to_string()) }
                    )
                )
            );
        }
    }
}

#[post("/user", data = "<register_data>")]
async fn register(db: &State<Database>, register_data: Json<User>) -> Result<Json<NetworkResponse>, (Status, Json<NetworkResponse>)> {
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
                        NetworkResponse { body: ResponseBody::Message("Invalid Email".to_string()) }
                    )
                )
            );
        }
        Err(UserError::InvalidPassword) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        NetworkResponse { body: ResponseBody::Message("Invalid Password".to_string()) }
                    )
                )
            );
        }
        Err(UserError::InvalidUsername) => {
            return Err(
                (
                    Status::BadRequest,
                    Json(
                        NetworkResponse { body: ResponseBody::Message("Invalid Username".to_string()) }
                    )
                )
            );
        }
        Err(_) => {
            return Err(
                (
                    Status::InternalServerError,
                    Json(
                        NetworkResponse { body: ResponseBody::Message("Error".to_string()) }
                    )
                )
            );
        }
    }

    if collection.insert_one(&user).await.is_ok() {
        let response = NetworkResponse { body: ResponseBody::Data(user.to_json()) };
        Ok(Json(response))
    } else {
        Err((
            Status::InternalServerError,
            Json(
                NetworkResponse { body: ResponseBody::Message("Error".to_string()) }
            )
        ))
    }
}

//TODO use JWT
#[post("/login", data = "<login_data>")]
async fn login(db: &State<Database>, login_data: Json<LoginRequest>) -> Result<Json<NetworkResponse>, (Status, Json<NetworkResponse>)> {
    let username = &login_data.username;
    let password = &login_data.password;

    let collection = db.client.lock().await.database("rocket-template").collection("users");
    match collection.find_one(doc! { "username": username }).await {
        Ok(Some(doc)) => {
            if let Ok(user) = from_bson::<User>(doc) {
                if user.check_password(password) {
                    return Ok(Json(
                        NetworkResponse { body: ResponseBody::Message("Successful login".to_string()) }
                    ));
                }
            }
            Err((Status::BadRequest, Json(NetworkResponse { body: ResponseBody::Message("Invalid Password".to_string()) })))
        }
        Ok(None) => Err((Status::BadRequest, Json(NetworkResponse { body: ResponseBody::Message("User not found".to_string()) }))),
        Err(_) => Err((Status::InternalServerError, Json(NetworkResponse { body: ResponseBody::Message("Error".to_string()) }))),
    }
}



// Export the routes
pub fn api_routes() -> Vec<Route> {
    routes![info, register, login]
}