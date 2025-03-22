#[macro_use] extern crate rocket;
use rocket::http::Status;
use crate::responses::responses::Response;
use rocket::serde::json::Json;
use serde_json::json;


mod responses;



#[get("/")]
fn index() -> Result<Json<Response>, Status> {
    //using macro to create a new response
    let response = new_response!(json!({"message": "Hello, world!"}));
    Ok(Json(response))
}
    

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .configure(rocket::Config {
            address: "0.0.0.0".parse().expect("invalid address"),
            port: 3000,
            ..Default::default()
        })
}