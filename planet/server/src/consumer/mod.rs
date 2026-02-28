use horfimbor_eventsource::model_key::ModelKey;
use public_mono::planet::{PLANET_ADMIN_STREAM, UUID_ADMIN_V8_KIND};
use url::Url;

pub mod civilisation;
pub mod civilisation_admin;
pub mod planet;

fn generate_admin_id(game_host: &Url, service_host: &Url) -> ModelKey {
     ModelKey::new_uuid_v8(
        PLANET_ADMIN_STREAM,
        UUID_ADMIN_V8_KIND,
        format!("{game_host},{service_host}").as_str(),
    )
}
