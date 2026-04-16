use crate::CivilizationAdminRepository;
use crate::web::AuthConfig;

use civilization_shared::command::CivilizationAdminCommand;
use civilization_shared::command::CivilizationAdminCommand::CreateServer;
use civilization_shared::event::CivilizationAdminEvent;
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::helper::get_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::repository::Repository;
use horfimbor_jwt::Role;
use horfimbor_jwt::rocket::{AuthClaim, GateAdmin, get_checked_claims};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::{Route, State};

pub fn routes() -> Vec<Route> {
    routes![admin_command, stream_admin]
}

#[post("/", format = "json", data = "<command>")]
pub async fn admin_command(
    state_repository: &State<CivilizationAdminRepository>,
    command: Json<CivilizationAdminCommand>,
    _claim: AuthClaim<GateAdmin>,
    auth_config: &State<AuthConfig>,
) -> Result<(), String> {
    let key = auth_config.get_application_key();
    state_repository
        .add_command(&key, command.0, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[get("/<jwt>")]
pub async fn stream_admin(
    state_repository: &State<CivilizationAdminRepository>,
    jwt: &str,
    auth_config: &State<AuthConfig>,
) -> Result<EventStream![], String> {
    //FIXME security ISSUE here
    let _claims = get_checked_claims(jwt, Role::Admin)?;

    let key = auth_config.get_application_key();

    let dto = state_repository
        .get_model(&key)
        .await
        .map_err(|_| "cannot find the admin dto".to_string())?;

    if dto.position().is_none() {
        state_repository
            .add_command(&key, CreateServer(auth_config.app_host.clone()), None)
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
        yield Event::json(&dto.state().dto());
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

                match original_event.as_json::<CivilizationAdminEvent>(){
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
