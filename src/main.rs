#[macro_use] extern crate rocket;    

mod api;
mod responses;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", api::api::api_routes())
        .configure(rocket::Config {
            address: "0.0.0.0".parse().expect("invalid address"),
            port: 3000,
            ..Default::default()
        })
}