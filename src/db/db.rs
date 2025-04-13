use mongodb::{Client};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::env;
use dotenvy::dotenv;

pub struct Database {
    pub client: Arc<Mutex<Client>>,
}

pub async fn init_db() -> Database {
    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let client = Client::with_uri_str(mongo_uri).await.expect("Failed to initialize MongoDB client");
    Database {
        client: Arc::new(Mutex::new(client)),
    }
}