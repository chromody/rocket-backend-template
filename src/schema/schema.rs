use rocket::serde::{Deserialize, Serialize};
use bcrypt::{hash, DEFAULT_COST};
use regex::Regex;
use mongodb::bson::{oid::ObjectId, serde_helpers};

pub enum UserError {
    InvalidEmail,
    InvalidPassword,
    InvalidUsername,
    InvalidId
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    username: String,
    email: String,
    _id: Option<ObjectId>,
    password: String,
}

impl User {
    pub fn register(username: String, email: String, _id: Option<String>, password: String) -> Result<Self, UserError> {
        let EMAIL_REGEX: Regex = Regex::new(r".$").expect("Invalid regex");
        let PWD_REGEX: Regex = Regex::new(r".$").expect("Invalid regex");
        let UNAME_REGEX: Regex = Regex::new(r".$").expect("Invalid regex");
        let id = match _id {
            Some(id) => {
                match ObjectId::parse_str(id) {
                    Ok(object_id) => object_id,
                    Err(e) => return Err(UserError::InvalidId),
                }
            }
            None => ObjectId::new(),
        };

        if !EMAIL_REGEX.is_match(&email) {
            return Err(UserError::InvalidEmail);
        }
        if !PWD_REGEX.is_match  (&password) {
            return Err(UserError::InvalidPassword);
        }
        if !UNAME_REGEX.is_match(&username) {
            return Err(UserError::InvalidUsername);
        }

        let password = hash(password, DEFAULT_COST).unwrap();

        Ok(User {
            username: username,
            email: email,
            _id: Some(id),
            password: password,
        })
    }

    pub fn check_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap()
    }
    pub fn get_id(&self) -> Option<ObjectId> {
        self._id.clone()
    }
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
    pub fn get_email(&self) -> String {
        self.email.clone()
    }
    pub fn get_password(&self) -> String {
        self.password.clone()
    }
    pub fn set_id(&mut self, _id: ObjectId) {
        self._id = Some(_id);
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }


    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "user": {
                "username": self.username,
                "email": self.email,
                "_id": self._id,
                "password": self.password,
            }
        })
    }
}

//macro to get any of the fields from the user struct
#[macro_export] //use instead of getters
macro_rules! get_user_field {
    //matches field identifier to getter
    ($user:ident, username) => {
        $user.get_username()
    };
    ($user:ident, email) => {
        $user.get_email()
    };
    ($user:ident, _id) => {
        $user.get_id()
    };
    ($user:ident, password) => {
        $user.get_password()
    };
}

//macro to set any of the fields from the user struct
#[macro_export]
macro_rules! set_field {
    ($user:ident, username, $value:expr) => {
        $user.set_username($value)
    };
    ($user:ident, email, $value:expr) => {
        $user.set_email($value)
    };
    ($user:ident, _id, $value:expr) => {
        $user.set_id($value)
    };
    ($user:ident, password, $value:expr) => {
        $user.set_password($value)
    };
}