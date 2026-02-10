use crate::CivilisationRepository;
use crate::web::{AuthAccountClaim, get_jwt_claims};
use civilisation_shared::command::CivilisationCommand;
use civilisation_shared::event::CivilisationEvent;
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::helper::get_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::Repository;
use public_mono::civilisation::{MONO_CIVILISATION_STREAM, UUID_V8_KIND};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::{Route, State};

pub fn routes() -> Vec<Route> {
    routes![mono_command, stream_dto]
}

#[post("/", format = "json", data = "<command>")]
pub async fn mono_command(
    state_repository: &State<CivilisationRepository>,
    command: Json<CivilisationCommand>,
    claim: AuthAccountClaim,
) -> Result<(), String> {
    let model = state_repository
        .get_model(&claim.account_model_key)
        .await
        .map_err(|e| e.to_string())?;

    // TODO check why this to_string is needed :thinking:
    if claim.claims.user().to_string() != model.state().owner().to_string() {
        return Err("not your account".to_string());
    }

    state_repository
        .add_command(&claim.account_model_key, command.0, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[get("/<jwt>")]
pub async fn stream_dto(
    repository: &State<CivilisationRepository>,
    jwt: &str,
) -> Result<EventStream![], String> {
    let claims = get_jwt_claims(jwt)?;

    let key = ModelKey::new_uuid_v8(
        MONO_CIVILISATION_STREAM,
        UUID_V8_KIND,
        &claims.account().to_string(),
    );

    let dto = repository
        .get_model(&key)
        .await
        .map_err(|err| format!("cannot find the dto {key} : {err}"))?;

    if dto.position().is_none() {
        return Err("account not found".to_string());
    }

    let mut subscription =
        get_subscription(repository.event_db(), &Stream::Model(key), dto.position()).await;

    Ok(EventStream! {
        yield Event::json(&dto.state().shared());
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

                match original_event.as_json::<CivilisationEvent>(){
                    Ok(event) =>{
                        if let CivilisationEvent::Shared(event) = event {
                            yield Event::json(&event);
                        }
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
