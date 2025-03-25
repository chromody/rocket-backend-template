use rocket::serde::{Deserialize, Serialize};
use jsonwebtoken::errors::{Error, ErrorKind};
use mongodb::bson::{oid::ObjectId};

use chrono::Utc;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Algorithm, Header, Validation}; // 👈 New!
use std::env;
use dotenvy::dotenv;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: ObjectId,
    exp: usize
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims
}


pub fn create_jwt(id: ObjectId) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set."); // 👈 New!

    let expiration = Utc::now().checked_add_signed(chrono::Duration::seconds(60)).expect("Invalid timestamp").timestamp();
    
    let claims = Claims {
        subject_id: id,
        exp: expiration as usize
    }; 

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    // 👇 New!
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}