use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Users {
    username: String,
    email: String,
    user_id: String,
    password: String,
}

impl Users {
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

pub fn create_user(username: String, email: String, user_id: String, password: String) -> Users {
    Users {
        username: username,
        email: email,
        user_id: user_id,
        password: password,
    }
}
