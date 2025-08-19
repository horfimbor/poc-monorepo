use crate::{PlanetRepository, built_info};
use anyhow::{Context, Error};
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_jwt::Claims;
use kurrentdb::Client;
use public_mono::civilisation::{MONO_CIVILISATION_STREAM, UUID_V8_KIND};
use redis::Client as RedisClient;
use rocket::Request;
use rocket::fs::{FileServer, relative};
use rocket::http::{Method, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;

pub mod planet;

pub async fn start_server(
    event_store_db: Client,
    planet_repo_state: PlanetRepository,
    dto_redis: RedisClient,
    port: Option<u16>,
) -> Result<(), Error> {
    let auth_port = if let Some(port) = port {
        port
    } else {
        env::var("APP_PORT")
            .context("APP_PORT is not defined")?
            .parse::<u16>()
            .context("APP_PORT cannot be parse in u16")?
    };

    let env_cors = env::var("CORS_HOST").context("CORS_HOST is not defined")?;
    let app_host = env_cors.split(";");
    let list: Vec<&str> = app_host.clone().collect();
    let allowed_origins = AllowedOrigins::some_exact(&list);

    dbg!(&allowed_origins);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .context("fail to create cors")?;

    let figment = rocket::Config::figment()
        .merge(("port", auth_port))
        .merge(("address", "0.0.0.0"));
    let _rocket = rocket::custom(figment)
        .manage(planet_repo_state)
        .manage(dto_redis)
        .manage(event_store_db)
        .mount("/", routes![redirect_index_js])
        .mount("/api/planet", planet::routes())
        .mount("/", FileServer::from(relative!("web")))
        .attach(cors)
        .register("/", catchers![general_not_found])
        .launch()
        .await;

    Ok(())
}

#[catch(404)]
fn general_not_found() -> RawHtml<&'static str> {
    RawHtml(
        r"
        <p>Hmm... This is not the droïd you are looking for, oupsi</p>
    ",
    )
}

fn get_jwt_claims(token: &str) -> Result<Claims, String> {
    let secret = env::var("JWT_SECRET_KEY").map_err(|_| "JWT_SECRET_KEY is missing")?;
    let auth_host = env::var("AUTH_HOST").map_err(|_| "AUTH_HOST is missing")?;
    let app_id = env::var("APP_ID").map_err(|_| "APP_ID is missing")?;
    let claims = Claims::from_jwt(token, &secret, &app_id, &auth_host).map_err(|e| {
        println!("claims error : {e:?}");
        "Invalid claims"
    })?;
    Ok(claims)
}

pub struct AuthAccountClaim {
    pub claims: Claims,
    pub account_model_key: ModelKey,
}

#[derive(Debug)]
pub enum AccountClaimError {
    Claim,
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthAccountClaim {
    type Error = AccountClaimError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            None => Outcome::Error((Status::BadRequest, AccountClaimError::Missing)),
            Some(token) => match get_jwt_claims(token) {
                Ok(claims) => Outcome::Success(AuthAccountClaim {
                    account_model_key: ModelKey::new_uuid_v8(
                        MONO_CIVILISATION_STREAM,
                        UUID_V8_KIND,
                        &claims.account().to_string(),
                    ),
                    claims,
                }),
                Err(e) => {
                    dbg!(e);
                    Outcome::Error((Status::BadRequest, AccountClaimError::Claim))
                }
            },
        }
    }
}

#[get("/client/index.js")]
pub fn redirect_index_js() -> Redirect {
    let wasm_tag: &'static str = env!("WASM_TAG");
    if !wasm_tag.is_empty() {
        Redirect::temporary(format!("/client/index-{wasm_tag}.js"))
    } else {
        Redirect::temporary(format!(
            "/client/index-v{}.js",
            built_info::PKG_VERSION.replace('.', "-")
        ))
    }
}
