#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use std::env;


mod google;
#[derive(Serialize)]
struct Time {
    time: String,
}

#[get("/time")]
fn time(
    _claims: google::Claims
) -> Result<Json<Time>, Status> {
    Ok(Json(Time {
        time: chrono::Utc::now().to_rfc3339(),
    }))
}

#[launch]
fn rocket() -> _ {
    env::set_var("ROCKET_PORT", env::var("PORT").unwrap_or("8000".into()));
    let certs = google::fetch().unwrap();
    rocket::build()
        .manage(certs)
        .mount("/api", routes![time])
        .mount("/", FileServer::from("./static"))
}
