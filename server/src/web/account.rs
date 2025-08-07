use crate::web::{AccountClaim, get_jwt_claims};
use crate::{AccountDtoRepository, AccountRepository};
use account_shared::command::AccountCommand;
use account_shared::event::AccountEvent;
use common::account::{MONO_ACCOUNT_STREAM, UUID_V8_KIND};
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::helper::get_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::Repository;
use rocket::State;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;

#[post("/", format = "json", data = "<command>")]
pub async fn mono_command(
    state_repository: &State<AccountRepository>,
    command: Json<AccountCommand>,
    claim: AccountClaim,
) -> Result<(), String> {
    let model = state_repository
        .get_model(&claim.account_model_key)
        .await
        .map_err(|e| e.to_string())?;

    if model.state().owner() != claim.claims.user() {
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
    dto_repository: &State<AccountDtoRepository>,
    jwt: &str,
) -> Result<EventStream![], String> {
    let claims = get_jwt_claims(jwt)?;

    let key = ModelKey::new_uuid_v8(
        MONO_ACCOUNT_STREAM,
        UUID_V8_KIND,
        &claims.account().to_string(),
    );

    let dto = dto_repository
        .get_model(&key)
        .await
        .map_err(|_| "cannot find the dto".to_string())?;

    if dto.position().is_none() {
        return Err("account not found".to_string());
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

                match original_event.as_json::<AccountEvent>(){
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
