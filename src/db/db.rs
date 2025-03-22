use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Database {
    pub client: Arc<Mutex<Client>>,
}

pub async fn init_db() -> Database {
    let client_uri = "mongodb://localhost:27017"; // Your MongoDB URI
    let client = Client::with_uri_str(client_uri).await.expect("Failed to initialize MongoDB client");
    Database {
        client: Arc::new(Mutex::new(client)),
    }
}