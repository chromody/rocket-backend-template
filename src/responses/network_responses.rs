use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
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
pub struct NetworkResponse {
    pub body: ResponseBody,
}