use std::{net::Ipv4Addr, str::FromStr};

use leptos::{*, leptos_dom::debug_warn};
use leptos_meta::*;
use leptos_router::*;
use reqwest::header::{CONTENT_TYPE, ACCEPT};
use serde::{Deserialize, Serialize};
use rand::Rng;
use self::api_types::{status, relay};
mod api_types;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Led {
    led_1,
    led_2
}
// extern crate reqwest;
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Stylesheet id="ajax" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"/>

            // sets the document title
            <Title text="HankeHausen"/>
            // content for this welcome page
            <Router>
                <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage devices=vec![
                                api_types::Device{
                                    ip:Ipv4Addr::new(192,168,178,94),
                                    name: Some("Hühnerhaus".to_string()),
                                    relay_names:vec!["INDOOR".to_string(),"OUTDOOR".to_string()]
                                },
                                api_types::Device{
                                    ip:Ipv4Addr::new(192,168,178,69),
                                    name: Some("Steckdose".to_string()),
                                    relay_names:vec!["BUTTON".to_string()]
                                }
                        ]/> }/>
                </Routes>
                </main>
            </Router>
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<usize>,
    title: String,
    content: Option<String>,
    completed: bool,
}

async fn get_status(ip:Ipv4Addr)->Result<api_types::status,reqwest::Error>{
    let url = format!(
        "http://{}/status",
        ip
        );
    let client = reqwest::Client::new();
    let response: api_types::status = client
        .get(url).fetch_mode_no_cors()
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

#[server(GetState, "/api")]
pub async fn get_state(ip:Ipv4Addr) -> Result<api_types::status, ServerFnError> {
    // tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    debug_warn!("getting called");
    println!("getting called");
    let data = get_status(ip).await;
    if let Err(err) =  &data {
        panic!("http error: {:?}", err);
    }
    Ok(data.unwrap())
}
#[server(SetRelayState, "/api")]
pub async fn set_relay_state(ip: Ipv4Addr,index: usize, action: api_types::Turn) -> Result<api_types::relay, ServerFnError> {
    // tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    let url = format!(
        "http://{}/relay/{}?turn={}",
        ip,
        index,
        action
        );
    println!("{:?}",url);
    let client = reqwest::Client::new();
    // without json
    let response: api_types::relay = client
        .get(url).fetch_mode_no_cors()
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await.unwrap_or_else(|_|{
            relay{
        ison: false,
        is_valid: Some(true),
        has_timer: false,
        overpower: false,
        overtemperature: Some(false),
        timer_duration: 0,
        timer_started: 0,
        timer_remaining: 0,
        source: "bla".to_string()}
      });

    Ok(response)
}

#[component]
fn Button(cx: Scope,device:api_types::Device, index:usize, relay:relay) -> impl IntoView {
   let (state, set_state) = create_signal(cx, relay.ison);
   let test = api_types::Device{
                                    ip:Ipv4Addr::new(192,168,178,94),
                                    name: Some("Hühnerhaus".to_string()),
                                    relay_names:vec!["INDOOR".to_string(),"OUTDOOR".to_string()]
                                };
   let data = Box::new(test.get_ip());
   let fetch_state = create_local_resource_with_initial_value(cx,state,move |state| async move {

       let action = match state {
           true=>api_types::Turn::On,
           false=>api_types::Turn::Off
       };
      // set the button to off if the response was incorrect 
      let data = set_relay_state(device.ip,index,action).await;
      data.unwrap_or_else(|_|{
        relay{
        ison: false,
        is_valid: Some(true),
        has_timer: false,
        overpower: false,
        overtemperature: Some(false),
        timer_duration: 0,
        timer_started: 0,
        timer_remaining: 0,
        source: "bla".to_string()}
      }) 


   },
   Some(relay)
       );
   view! {cx,
        <div class="action">
        <Suspense fallback= move || {view!{cx,"loading..."}}>
            {
                // {button_view.clone()}
                match fetch_state.read(cx).unwrap().ison {
                    true => view!{cx,<button on:click=move |_|{set_state.update(|state| *state=!*state)} class="on" >{device.relay_names.get(index).unwrap()}</button>},
                    false =>view!{cx,<button on:click=move |_|{set_state.update(|state| *state=!*state)}>{device.relay_names.get(index).unwrap()}</button>}
                }
            }
        </Suspense>
        </div>

   // view! {cx,<button></button>} 
   }

}
#[component]
fn Device(cx: Scope,device:api_types::Device)-> impl IntoView {
    let (reload_status, set_reload_status) = create_signal(cx, false);
    let status = create_resource(cx,reload_status, move |_| async move {get_state(device.ip.clone()).await});
    let name = device.name.clone();
    let action_view = move || {
        status.with(cx,|_|
                    {
                        status.read(cx).unwrap().unwrap().clone().relays.into_iter().enumerate().map(|(index,relay)| {
                            view! {cx,<Button device=device.clone() index=index relay=relay.clone() />}
                        }).collect::<Vec<_>>()
                    }
                    )
    };
    
    // Main view
    view! {cx,
        <Suspense fallback=move || {view!{cx,"loading..."}}>
            <p>{name.clone()}</p>
            <div class="layout">
                {action_view.clone()}
            </div>
        </Suspense>
    }
    
}
#[component]
fn HomePage(cx:Scope, devices:Vec<api_types::Device>) -> impl IntoView {
    
    // Main view
    view! {cx,
        <div class="header">
            <p>"Hankehausen"</p>
        </div> 
        <div class="devices">
            <For 
                each={move || devices.clone().into_iter()}
                key={|device| device.clone()}
                view=move |cx, device| {
                    view! {cx,
                    <div class="device">
                        <Device device=device />
                    </div>
                    }
                }
            />
        </div>
    }
}
