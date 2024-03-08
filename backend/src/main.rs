#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::serde::json::Json;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Time {
    time: String,
}

#[get("/time")]
fn time() -> Json<Time> {
    Json(Time {
        time: chrono::Utc::now().to_rfc3339(),
    })
}

#[launch]
fn rocket() -> _ {
    env::set_var("ROCKET_PORT", env::var("PORT").unwrap_or("8000".into()));
    rocket::build()
        .mount("/api", routes![time])
        .mount("/", FileServer::from("./static"))
}
