use crate::{PlanetAdminRepository, PlanetRepository, built_info};
use anyhow::{Context, Error};
use horfimbor_jwt::Claims;
use kurrentdb::Client;
use redis::Client as RedisClient;
use rocket::fs::{FileServer, relative};
use rocket::http::Method;
use rocket::response::Redirect;
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;
use url::Url;

mod admin;
pub mod planet;

pub async fn start_server(
    event_store_db: Client,
    planet_repo_state: PlanetRepository,
    planet_repo_admin: PlanetAdminRepository,
    dto_redis: RedisClient,
    app_host: Url,
) -> Result<(), Error> {
    let auth_port = if let Some(port) = app_host.port() {
        port
    } else {
        env::var("APP_PORT")
            .context("APP_PORT is not defined")?
            .parse::<u16>()
            .context("APP_PORT cannot be parse in u16")?
    };

    let env_cors = env::var("CORS_HOST").context("CORS_HOST is not defined")?;
    let cors_host = env_cors.split(";");
    let list: Vec<&str> = cors_host.clone().collect();
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
        .manage(planet_repo_admin)
        .manage(dto_redis)
        .manage(event_store_db)
        .manage(app_host)
        .mount("/", routes![redirect_index_js])
        .mount("/api/planet-admin/", admin::routes())
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
