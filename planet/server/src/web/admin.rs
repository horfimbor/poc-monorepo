use crate::web::get_jwt_claims;
use crate::{PlanetAdminRepository, get_admin_id};
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::helper::get_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::repository::Repository;
use horfimbor_jwt::rocket::{AuthClaim, GateAdmin};
use planet_shared::command::SharedPlanetAdminCommand;
use planet_shared::event::SharedPlanetAdminEvent;
use planet_state::admin::PlanetAdminCommand;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::{Route, State};
use url::Url;

pub fn routes() -> Vec<Route> {
    routes![admin_command, stream_admin]
}

#[post("/", format = "json", data = "<command>")]
pub async fn admin_command(
    state_repository: &State<PlanetAdminRepository>,
    command: Json<SharedPlanetAdminCommand>,
    auth: AuthClaim<GateAdmin>,
    app_host: &State<Url>,
) -> Result<(), String> {
    let audience = auth
        .claims()
        .audience()
        .try_into()
        .map_err(|e| format!("audience is not a ModelKey: {e}"))?;

    let key = get_admin_id(&audience, app_host);

    state_repository
        .add_command(&key, PlanetAdminCommand::Shared(command.0), None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
#[get("/<jwt>")]
pub async fn stream_admin(
    state_repository: &State<PlanetAdminRepository>,
    jwt: &str,
    app_host: &State<Url>,
) -> Result<EventStream![], String> {
    //FIXME security ISSUE here
    let claims = get_jwt_claims(jwt)?;

    let audience = claims
        .audience()
        .try_into()
        .map_err(|e| format!("audience is not a ModelKey: {e}"))?;

    let key = get_admin_id(&audience, app_host);

    let dto = state_repository
        .get_model(&key)
        .await
        .map_err(|_| "cannot find the admin dto".to_string())?;

    // let host = Url::parse(&auth_config.app_host).map_err(|_| "cannot parse app_host")?;

    // if dto.position().is_none() {
    //     state_repository
    //         .add_command(&key, CreateServer(host), None)
    //         .await
    //         .map_err(|e| e.to_string())?;
    // }

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

                match original_event.as_json::<SharedPlanetAdminEvent>(){
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
