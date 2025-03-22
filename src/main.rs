#[macro_use] extern crate rocket;    

mod api;
mod responses;
mod schema;
mod db;

use crate::db::db::init_db;

#[launch]
async fn rocket() -> _ {
    let db = init_db().await;

    rocket::build()
        .manage(db)
        .mount("/", api::api::api_routes())
        .configure(rocket::Config {
            address: "0.0.0.0".parse().expect("invalid address"),
            port: 3000,
            ..Default::default()
        })
}