use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use serde_json::Error;
use std::time::Duration;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use civilisation_shared::dto::CivilisationDto;
use civilisation_shared::event::CivilisationEvent;

#[allow(dead_code)]
pub struct MonoState {
    es: Option<EventSource>,
    dto: Result<CivilisationDto, String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DtoMessage {
    Dto(CivilisationDto),
    Event(CivilisationEvent),
    Error(String),
    Reconnect,
}

#[derive(WebComponent)]
#[component(MonoState)]
#[derive(Default, Properties, PartialEq)]
pub struct MonoStateProps {
    pub endpoint: String,
    pub jwt: String,
}

impl MonoState {
    fn connect(&mut self, ctx: &Context<Self>) {
        if self.es.is_some() {
            return;
        }
        self.dto = Ok(CivilisationDto::default());

        let endpoint = ctx.props().endpoint.clone();
        let jwt = ctx.props().jwt.clone();

        let mut es = match EventSource::new(format!("{endpoint}/api/civilisation/{jwt}").as_str()) {
            Ok(es) => es,
            Err(_) => {
                self.dto = Err(format!(
                    "cannot open eventsource to {endpoint}/api/civilisation/<jwt>"
                ));
                return;
            }
        };

        let mut stream = match es.subscribe("message") {
            Ok(stream) => stream,
            Err(_) => {
                self.dto = Err("cannot subscribe to all messages".to_string());
                return;
            }
        };

        let link = ctx.link().clone();
        spawn_local(async move {
            while let Some(Ok((_, msg))) = stream.next().await {
                if let Some(json) = msg.data().as_string() {
                    let message: Result<DtoMessage, Error> = serde_json::from_str(json.as_str());

                    let link = link.clone();
                    match message {
                        Ok(m) => {
                            link.send_message(m);
                        }
                        Err(_) => {
                            link.send_message(DtoMessage::Error("stream closed".to_string()));
                        }
                    }
                }
            }
            link.send_message(DtoMessage::Error("EventSource closed".to_string()));
        });

        self.es = Some(es);
    }
}

impl Component for MonoState {
    type Message = DtoMessage;
    type Properties = MonoStateProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut state = Self {
            es: None,
            dto: Ok(CivilisationDto::default()),
        };

        state.connect(ctx);

        state
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DtoMessage::Dto(d) => {
                self.dto = Ok(d);
                true
            }
            DtoMessage::Event(e) => match &mut self.dto {
                Ok(dto) => {
                    dto.play_event(&e);
                    true
                }
                Err(_) => false,
            },
            DtoMessage::Error(e) => {
                self.dto = Err(e);
                self.es = None;

                let link = ctx.link().clone();

                spawn_local(async move {
                    sleep(Duration::from_secs(5)).await;
                    link.send_message(DtoMessage::Reconnect);
                });
                true
            }
            DtoMessage::Reconnect => {
                self.connect(ctx);
                if self.dto.is_err() {
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        sleep(Duration::from_secs(5)).await;
                        link.send_message(DtoMessage::Reconnect);
                    });
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = move || -> Html {
            match &self.dto {
                Ok(dto) => {
                    let nation_part = match dto.nation() {
                        None => {
                            html! {
                                <div>
                                    {"No nation name yet"}
                                </div>
                            }
                        }
                        Some(nation) => {
                            html! {
                                <div>
                                    <b>{&nation.name}</b><p>{&nation.description}</p>
                                </div>
                            }
                        }
                    };

                    let script = r#"
                        import init, { run } from 'http://mono.localhost:8001/mono/index.js';
                        async function main() {
                            await init();
                            run();
                        }
                        main();

                    "#;

                    let world_part = html!(<>{
                        dto.worlds().iter().map(|world|{
                            html!(
                                <fieldset>
                                    <@{world.balise.clone()}
                                        // endpoint={ctx.props().endpoint.clone()}
                                        endpoint={"http://mono.localhost:8001"}
                                        jwt={ctx.props().jwt.clone()}
                                        id={world.id.clone()}
                                    />
                                </fieldset>
                            )
                        }).collect::<Html>()

                    }</>);

                    html! {
                        <>
                            <horfimbor-account-input endpoint={ctx.props().endpoint.clone()} jwt={ctx.props().jwt.clone()}></horfimbor-account-input>
                            <hr/>
                            {nation_part}
                            <hr/>
                            <script type="module">{script}</script>
                            {world_part}
                        </>
                    }
                }
                Err(e) => {
                    html! {
                        <h2 style="float:right">
                            {e}
                        </h2>
                    }
                }
            }
        };

        html! {
            {state()}
        }
    }
}
