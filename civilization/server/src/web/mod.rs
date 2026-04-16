use crate::web::base::load_base_routes;
use crate::{CivilizationAdminRepository, CivilizationRepository};
use anyhow::{Context, Error};
use horfimbor_eventsource::model_key::ModelKey;
use kurrentdb::Client;
use public_mono::civilization::{MONO_CIVILIZATION_ADMIN_STREAM, UUID_ADMIN_V8_KIND};
use redis::Client as RedisClient;
use rocket::fs::{FileServer, relative};
use rocket::http::Method;
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_dyn_templates::Template;
use std::env;
use url::Url;

pub mod admin;
mod base;
pub mod civilization;

pub async fn start_server(
    event_store_db: Client,
    civilization_repo: CivilizationRepository,
    civilization_admin_repo: CivilizationAdminRepository,
    dto_redis: RedisClient,
    auth_config: AuthConfig,
) -> Result<(), Error> {
    let app_port = auth_config.app_host.port();

    let allowed_origins = AllowedOrigins::some_exact(&[auth_config.app_host.to_string()]);

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
        .merge(("port", app_port))
        .merge(("address", "0.0.0.0"))
        .merge(("template_dir", "civilization/server/templates"));
    let _rocket = rocket::custom(figment)
        .manage(civilization_repo)
        .manage(civilization_admin_repo)
        .manage(auth_config)
        .manage(dto_redis)
        .manage(event_store_db)
        .mount("/", load_base_routes())
        .mount("/api/civilization-admin/", admin::routes())
        .mount("/api/civilization", civilization::routes())
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
        r#"<body style="background-color: darkgray;">
            <p>Hmm... This is not the droïd you are looking for, oupsi</p>
            </body>
        "#,
    )
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub app_host: Url,
    pub app_key: String,
    pub auth_host: Url,
    pub auth_callback_host: Url,
}

impl AuthConfig {
    pub fn get_application_key(&self) -> ModelKey {
        ModelKey::new_uuid_v8(
            MONO_CIVILIZATION_ADMIN_STREAM,
            UUID_ADMIN_V8_KIND,
            self.app_host.as_ref(),
        )
    }
}
