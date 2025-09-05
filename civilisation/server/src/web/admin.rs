use crate::CivilisationAdminRepository;
use crate::web::{AuthAccountAdminClaim, AuthConfig, get_jwt_claims};
use civilisation_admin::CivilisationAdminCommand::CreateServer;
use civilisation_admin::{CivilisationAdminCommand, CivilisationAdminEvent};
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::helper::get_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::Repository;
use public_mono::civilisation::{MONO_CIVILISATION_ADMIN_STREAM, UUID_ADMIN_V8_KIND};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::{Route, State};
use url::Host;

pub fn routes() -> Vec<Route> {
    routes![admin_command, stream_admin]
}

fn get_application_key(config: &AuthConfig) -> ModelKey {
    ModelKey::new_uuid_v8(
        MONO_CIVILISATION_ADMIN_STREAM,
        UUID_ADMIN_V8_KIND,
        &config.app_host,
    )
}

#[post("/", format = "json", data = "<command>")]
pub async fn admin_command(
    state_repository: &State<CivilisationAdminRepository>,
    command: Json<CivilisationAdminCommand>,
    _claim: AuthAccountAdminClaim,
    auth_config: &State<AuthConfig>,
) -> Result<(), String> {
    let key = get_application_key(auth_config);
    state_repository
        .add_command(&key, command.0, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[get("/<jwt>")]
pub async fn stream_admin(
    state_repository: &State<CivilisationAdminRepository>,
    jwt: &str,
    auth_config: &State<AuthConfig>,
) -> Result<EventStream![], String> {
    let _claims = get_jwt_claims(jwt)?;

    let key = get_application_key(auth_config);

    let dto = state_repository
        .get_model(&key)
        .await
        .map_err(|_| "cannot find the dto".to_string())?;

    if dto.position().is_none() {
        state_repository
            .add_command(
                &key,
                CreateServer(Host::Domain(auth_config.app_host.clone())),
                None,
            )
            .await
            .map_err(|e| e.to_string())?;
    }

    let mut subscription = get_subscription(
        state_repository.event_db(),
        &Stream::Model(key),
        dto.position(),
    )
    .await;

    Ok(EventStream! {
        yield Event::json(&dto.state());
        loop {
            let event = if let Ok(event) = subscription.next().await{
                event
            }else{
                yield Event::data("cannot get event").event("error");
                break;
            };
            let original_event = event.get_original_event();
            let metadata: Metadata = if let Ok(metadata) =  serde_json::from_slice(original_event.custom_metadata.as_ref()){
                metadata
            }else{
                yield Event::data("cannot get metdata").event("error");
                break;
            };

            if metadata.is_event(){

                match original_event.as_json::<CivilisationAdminEvent>(){
                    Ok(event) =>{
                        yield Event::json(&event);
                    },
                    Err(_) => {
                        yield Event::data("cannot get original event").event("error");
                        break;
                    }
                };

            }
        }
    })
}
