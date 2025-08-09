use crate::web::{AuthAccountClaim, get_jwt_claims};
use crate::{PlanetDtoRepository, PlanetRepository};
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::helper::get_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::Repository;
use planet_shared::command::PlanetCommand;
use planet_shared::event::PlanetEvent;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::{Route, State};

pub fn routes() -> Vec<Route> {
    routes![mono_command, stream_dto]
}

#[post("/<model_id>", format = "json", data = "<command>")]
pub async fn mono_command(
    state_repository: &State<PlanetRepository>,
    command: Json<PlanetCommand>,
    claim: AuthAccountClaim,
    model_id: &str,
) -> Result<(), String> {
    let key = ModelKey::try_from(model_id).map_err(|_| "invalid id")?;

    let model = state_repository
        .get_model(&key)
        .await
        .map_err(|e| e.to_string())?;

    dbg!(model.state().owner());
    dbg!(claim.claims.user());

    // if model.state().owner() != claim.claims.user() {
    //     return Err("not your planet".to_string());
    // }

    state_repository
        .add_command(&key, command.0, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[get("/<model_id>/<jwt>")]
pub async fn stream_dto(
    dto_repository: &State<PlanetDtoRepository>,
    model_id: &str,
    jwt: &str,
) -> Result<EventStream![], String> {
    let _ = get_jwt_claims(jwt)?; // TODO move into FromRequest

    let key = ModelKey::try_from(model_id).map_err(|_| "invalid id")?;

    let dto = dto_repository
        .get_model(&key)
        .await
        .map_err(|_| "cannot find the dto".to_string())?;

    if dto.position().is_none() {
        return Err("planet not found".to_string());
    }

    let mut subscription = get_subscription(
        dto_repository.event_db(),
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

                match original_event.as_json::<PlanetEvent>(){
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
