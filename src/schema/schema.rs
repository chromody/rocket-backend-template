use rocket::serde::{Deserialize, Serialize};
use bcrypt::{hash, DEFAULT_COST};

#[derive(Deserialize, Serialize)]
pub struct User {
    username: String,
    email: String,
    user_id: String,
    password: String,
}

impl User {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "user": {
                "username": self.username,
                "email": self.email,
                "user_id": self.user_id,
                "password": self.password,
            }
        })
    }
}

pub fn create_user(username: String, email: String, user_id: String, password: String) -> User {
    let password = hash(password, DEFAULT_COST).unwrap();

    User {
        username: username,
        email: email,
        user_id: user_id,
        password: password,
    }
}