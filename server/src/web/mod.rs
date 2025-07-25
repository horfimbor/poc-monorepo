use crate::web::base::load_base_routes;
use crate::web::controller::{mono_command, stream_dto};
use crate::{MonoDtoCache, MonoDtoRepository, MonoRepository};
use anyhow::{Context, Error};
use horfimbor_jwt::Claims;
use kurrentdb::Client;
use redis::Client as RedisClient;
use rocket::fs::{FileServer, relative};
use rocket::http::Method;
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_dyn_templates::Template;
use std::env;

mod base;
pub mod controller;

pub async fn start_server(
    event_store_db: Client,
    repo_state: MonoRepository,
    repo_dto: MonoDtoRepository,
    dto_cache: MonoDtoCache,
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
        .manage(repo_state)
        .manage(repo_dto)
        .manage(auth_config)
        .manage(dto_redis)
        .manage(dto_cache)
        .manage(event_store_db)
        .mount("/", load_base_routes())
        .mount("/api", routes![mono_command, stream_dto])
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

fn get_jwt_claims(response: &str) -> Result<Claims, String> {
    let secret = env::var("JWT_SECRET_KEY").map_err(|_| "JWT_SECRET_KEY is missing")?;
    let auth_host = env::var("AUTH_HOST").map_err(|_| "AUTH_HOST is missing")?;
    let app_id = env::var("APP_ID").map_err(|_| "APP_ID is missing")?;
    let claims = Claims::from_jwt(response, &secret, &app_id, &auth_host).map_err(|e| {
        println!("claims error : {e:?}");
        "Invalid claims"
    })?;
    Ok(claims)
}
