use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use rocket::request::FromRequest;
use rocket::State;
use rocket::{http::Status, request::Outcome, Request};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Key {
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keys {
    keys: Vec<Key>,
}

impl Keys {
    pub fn get(&self, kid: &str) -> Result<DecodingKey, Box<dyn Error>> {
        for key in &self.keys {
            if key.kid == *kid {
                return Ok(DecodingKey::from_rsa_components(&key.n, &key.e)?);
            }
        }
        Err("missing".into())
    }
}

pub fn parse(json: &str) -> Result<Keys, Box<dyn Error>> {
    let keys: Keys = serde_json::from_str(json)?;
    Ok(keys)
}

pub fn fetch() -> Result<Keys, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://www.googleapis.com/oauth2/v3/certs")
        .send()?
        .text()?;
    let keys: Keys = parse(&resp)?;
    Ok(keys)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub name: String,
    pub exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Outcome::Success(goog_pub_keys) = request.guard::<&State<Keys>>().await else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let Some(auth_header) = request.headers().get_one("Authorization") else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let parts: Vec<&str> = auth_header.split(' ').collect();
        if parts.len() != 2 || parts[0] != "Bearer" {
            return Outcome::Error((Status::Unauthorized, ()));
        }
        let bearer = parts[1];
        let Ok(header) = jsonwebtoken::decode_header(bearer) else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let Some(kid) = header.kid else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let Ok(pub_key) = goog_pub_keys.get(&kid) else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&[
            "981002175662-g8jr2n89bptsn8n9ds1fn5edfheojr7i.apps.googleusercontent.com",
        ]);
        validation.set_issuer(&["https://accounts.google.com"]);
        let Ok(token) = jsonwebtoken::decode::<Claims>(bearer, &pub_key, &validation) else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        Outcome::Success(Claims {
            email: token.claims.email,
            name: token.claims.name,
            exp: token.claims.exp,
        })
    }
}
