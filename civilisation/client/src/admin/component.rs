use crate::admin::CivilisationAdminProps;
use bounce::{Atom, BounceRoot, use_atom};
use civilisation_admin::CivilisationAdminCommand;
use horfimbor_client::EventStoreProps;
use horfimbor_client::input::send_command;
use url::Url;
use web_sys::{HtmlInputElement, InputEvent};
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::{Callback, Html, TargetCast, function_component, html};

#[derive(Eq, PartialEq, Atom, Default)]
struct LocalData {
    comp: String,
}

#[function_component(AddComponent)]
pub fn update_timer(props: &CivilisationAdminProps) -> Html {
    html!(
        <BounceRoot>
            <AddComponentBounce
                    endpoint={props.endpoint().to_owned()}
                    jwt={props.jwt().to_owned()}/>
        </BounceRoot>
    )
}

#[function_component(AddComponentBounce)]
pub fn update_timer(props: &CivilisationAdminProps) -> Html {
    let data = use_atom::<LocalData>();

    let oninput = {
        let data = data.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            data.set(LocalData {
                comp: input.value(),
            });
        })
    };

    let add_button = if let Ok(host) = Url::parse(&data.comp) {
        let props = props.clone();
        let on_send_clicked = Callback::from(move |_| {
            let host = host.clone();
            let cmd = CivilisationAdminCommand::AddComponent(host);
            let props = props.clone();
            spawn_local(async move {
                match send_command(&cmd, props).await {
                    Ok(resp) => {
                        if resp.ok() {
                            console_info!("Sent !");
                        }
                    }
                    Err(e) => {
                        console_error!(e);
                    }
                }
            });
        });
        html!(<button onclick={on_send_clicked}>{"Send"}</button>)
    } else {
        html!(<>{"invalid component"}</>)
    };

    html! {
        <>
            <label>{"New component"}
                <input type="text" {oninput} value={data.comp.clone()} />
            </label>
            <br/>
            {add_button}
        </>
    }
}
