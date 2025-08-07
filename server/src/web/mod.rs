use crate::web::base::load_base_routes;
use crate::{
    AccountDtoCache, AccountDtoRepository, AccountRepository, PlanetDtoCache, PlanetDtoRepository,
    PlanetRepository,
};
use anyhow::{Context, Error};
use common::account::{MONO_ACCOUNT_STREAM, UUID_V8_KIND};
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_jwt::Claims;
use kurrentdb::Client;
use redis::Client as RedisClient;
use rocket::Request;
use rocket::fs::{FileServer, relative};
use rocket::http::{Method, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_dyn_templates::Template;
use std::env;

pub mod account;
mod base;
pub mod planet;

pub async fn start_server(
    event_store_db: Client,
    account_repo_state: AccountRepository,
    account_repo_dto: AccountDtoRepository,
    account_dto_cache: AccountDtoCache,
    planet_repo_state: PlanetRepository,
    planet_repo_dto: PlanetDtoRepository,
    planet_dto_cache: PlanetDtoCache,
    dto_redis: RedisClient,
) -> Result<(), Error> {
    let auth_port = env::var("APP_PORT")
        .context("APP_PORT is not defined")?
        .parse::<u16>()
        .context("APP_PORT cannot be parse in u16")?;
    let app_host = env::var("APP_HOST").context("APP_HOST is not defined")?;
    let app_key = env::var("APP_KEY").context("APP_KEY is not defined")?;
    let auth_host = env::var("AUTH_HOST").context("APP_HOST is not defined")?;
    let auth_callback_host =
        env::var("AUTH_CALLBACK_HOST").context("AUTH_CALLBACK_HOST is not defined")?;
    let auth_config = AuthConfig {
        app_host: app_host.clone(),
        app_key,
        auth_host,
        auth_callback_host,
    };

    let allowed_origins = AllowedOrigins::some_exact(&[app_host]);

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
        .merge(("address", "0.0.0.0"))
        .merge(("template_dir", "server/templates"));
    let _rocket = rocket::custom(figment)
        .manage(account_repo_state)
        .manage(account_repo_dto)
        .manage(account_dto_cache)
        .manage(planet_repo_state)
        .manage(planet_repo_dto)
        .manage(planet_dto_cache)
        .manage(auth_config)
        .manage(dto_redis)
        .manage(event_store_db)
        .mount("/", load_base_routes())
        .mount("/api/account", account::routes())
        .mount("/api/planet", planet::routes())
        .mount("/", FileServer::from(relative!("web")))
        .attach(cors)
        .attach(Template::fairing())
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

#[derive(Debug)]
pub struct AuthConfig {
    app_host: String,
    app_key: String,
    auth_host: String,
    auth_callback_host: String,
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
                        MONO_ACCOUNT_STREAM,
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
