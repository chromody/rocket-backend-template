use rocket::serde::{Deserialize, Serialize};
use bcrypt::{hash, DEFAULT_COST};
use regex::Regex;

pub enum UserError {
    InvalidEmail,
    InvalidPassword,
    InvalidUsername,
}

#[derive(Deserialize, Serialize)]
pub struct UserRegister {
    username: String,
    email: String,
    password: String,
}

impl UserRegister {
    pub fn new(username: String, email: String, password: String) -> Result<Self, UserError> {
        let EMAIL_REGEX: Regex = Regex::new(r".$").expect("Invalid regex");
        let PWD_REGEX: Regex = Regex::new(r".$").expect("Invalid regex");
        let UNAME_REGEX: Regex = Regex::new(r".$").expect("Invalid regex");

        if !EMAIL_REGEX.is_match(&email) {
            return Err(UserError::InvalidEmail);
        }
        if !PWD_REGEX.is_match(&password) {
            return Err(UserError::InvalidPassword);
        }
        if !UNAME_REGEX.is_match(&username) {
            return Err(UserError::InvalidUsername);
        }

        let password = hash(password, DEFAULT_COST).unwrap();

        Ok(UserRegister {
            username: username,
            email: email,
            password: password,
        })
    }
    pub fn check_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap()
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn get_password(&self) -> &str {
        &self.password
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
                "password": self.password,
            }
        })
    }
}