use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::schema::user_schema::User;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}


#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseBody {
    Data(Value),
    Message(String),
    AuthToken(String),
    User(User),
    Users(Vec<User>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}