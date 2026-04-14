use crate::available_building::AvailableBuilding;
use horfimbor_client::EventStoreProps;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client_derive::WebComponent;
use planet_shared::dto::PlanetDto;
use planet_shared::event::SharedPlanetEvent;
use serde::Deserialize;
use weblog::console_debug;
use yew::prelude::*;
use crate::visual::PlanetVisual;

type PlanetState = EventStoreState<PlanetDto, SharedPlanetEvent, PlanetStateProps>;

#[derive(WebComponent)]
#[component(PlanetState)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct PlanetStateProps {
    pub endpoint: AttrValue,
    pub jwt: AttrValue,
    pub id: AttrValue,
}

impl EventStoreProps for PlanetStateProps {
    fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    fn path(&self) -> &str {
        "api/planet"
    }

    fn jwt(&self) -> &str {
        self.jwt.as_str()
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
}

impl AddEvent<SharedPlanetEvent, PlanetStateProps> for PlanetDto {
    fn play_event(&mut self, event: &SharedPlanetEvent) {
        self.play_event(event);
    }

    fn get_view(&self, props: PlanetStateProps) -> Html {
        let data = format!("{:?}", self);
        console_debug!(data);

        html! {
            <div>
                <PlanetVisual id={props.id.clone()}/>
                <AvailableBuilding state_props={props} available_building={self.available_building.clone()} />
                <h3>{"Construction"}</h3>
                <table>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Cost"}</th>
                        <th>{"Running cost"}</th>
                        <th>{"Production"}</th>
                        <th>{"Action"}</th>
                    </tr>

                    { for self.construction.iter().map(|(id, building)| html! {
                        <tr key={id.to_string()}>
                            <td>{&building.0.name}</td>
                         <td>
                        <ul>
                                { for building.0.construction.iter().map(|(resource, quantity)| html!{
                                    <li>{quantity}{" "}{resource}</li>
                                })}
                                <li>{building.0.construction_time}{" secondes"}</li>
                                </ul>
                            </td>
                            <td>
                                <ul>
                                { for building.0.running_cost.iter().map(|(resource, quantity)| html!{
                                    <li>{quantity}{" "}{resource}</li>
                                })}
                                </ul>
                            </td>
                            <td>
                                <ul>
                                { for building.0.production.iter().map(|(resource, production)| html!{
                                    <>
                                    <li>{production.quantity}{" "}{resource}</li>
                                    <li>{"( "}{production.stock}{" stock )"}</li>
                                    </>
                                })}
                                </ul>
                            </td>
                            <td>
                                {"cancel TODO"}
                                // <button id={id.to_string()} onclick={on_cancel.clone()}>{"build"}</button>
                            </td>
                        </tr>
                    }) }

                </table>
            <h3>{"Building"}</h3>
                <table>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Cost"}</th>
                        <th>{"Running cost"}</th>
                        <th>{"Production"}</th>
                        <th>{"Action"}</th>
                    </tr>

                    { for self.buildings.iter().map(|(id, building)| html! {
                        <tr key={id.to_string()}>
                            <td>{&building.name}</td>
                            <td>
                                <ul>
                                { for building.construction.iter().map(|(resource, quantity)| html!{
                                    <li>{quantity}{" "}{resource}</li>
                                })}
                                <li>{building.construction_time}{" secondes"}</li>
                                </ul>
                            </td>
                            <td>
                                <ul>
                                { for building.running_cost.iter().map(|(resource, quantity)| html!{
                                    <li>{quantity}{" "}{resource}</li>
                                })}
                                </ul>
                            </td>
                            <td>
                                <ul>
                                { for building.production.iter().map(|(resource, production)| html!{
                                    <>
                                    <li>{production.quantity}{" "}{resource}</li>
                                    <li>{"( "}{production.stock}{" stock )"}</li>
                                    </>
                                })}
                                </ul>
                            </td>
                            <td>
                                {"delete TODO"}
                                // <button id={id.to_string()} onclick={on_create.clone()}>{"build"}</button>
                            </td>
                        </tr>
                    }) }
                </table>
            </div>
        }
    }
}
