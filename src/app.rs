use leptos::{*, leptos_dom::debug_warn};
use leptos_meta::*;
use leptos_router::*;
use reqwest::header::{CONTENT_TYPE, ACCEPT};
use serde::{Deserialize, Serialize};
use rand::Rng;

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

            // sets the document title
            <Title text="HankeHausen"/>
            // content for this welcome page
            <Router>
                <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
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

async fn get_data()->Result<Vec<Todo>,reqwest::Error>{
    let url = format!(
        "https://jsonplaceholder.typicode.com/todos?userId=1",
        );
    let client = reqwest::Client::new();
    let response: Vec<Todo> = client
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
pub async fn get_state(led: Led) -> Result<bool, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    // let data = get_data().await;
    let mut rng = rand::thread_rng();
    let random_bool: bool = rng.gen();
    let state =  match led {
        Led::led_1=>false,
        Led::led_2=>true
    };
    debug_warn!("state: {:?}",state);
    // Just for now: return random bool to represent the button states -> shelly api
    Ok(random_bool)
}
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the buttons
    let (led_1, set_led_1) = create_signal(cx, 0);
    let (led_2, set_led_2) = create_signal(cx, 0);

    // Define the behavior when pressing each button
    let led_1_pressed = move |_| {set_led_1.update(|led_1| if *led_1 == 1 {
        *led_1 = 0;
    }else{

        *led_1 = 1;
    })};
    let led_2_pressed = move |_| {set_led_2.update(|led_2| if *led_2 == 1 {
        *led_2 = 0;
    }else{

        *led_2 = 1;
    })};
    // Creates resources, do async stuff 
    let state_led_1 = create_resource(cx,led_1, |_| async {get_state(Led::led_1).await});
    let state_led_2 = create_resource(cx,led_2, |_| async {get_state(Led::led_2).await});

    // Playing around 
    let (reactive_list, set_reactive_list) = create_signal(cx, vec!("hello world","we can now generate","more list items"));

    let on_click = move |_| {debug_warn!("{}",reactive_list.get().len()); set_reactive_list.update(|reactive_list| reactive_list.push("hey from button"));};

    // Main view
    view! {cx,
        <div class="header">
            <p>"Hankehausen"</p>
        </div> 
        <div class="layout">
        // TODO: move the button rendering into one extra component. Create loading animation.
        <div class="action">
        <Suspense fallback=move || view!{cx,<p>"loading..."</p>}>
        {
            // This is not a good way to handle conditional rendering.
            let mut return_view = view!{cx,<button></button>};
            if let Some(value) = state_led_1.read(cx)
                .and_then(|res| res.ok())
                {
                    if(value){
                        return_view = view!{cx,<button on:click=led_1_pressed class="on">"INDOOR"</button>}
                    }
                    else{
                        return_view = view!{cx,<button on:click=led_1_pressed>"INDOOR"</button>}
                    }
                } else {
                    // Handle the case where the result is None or contains an error
                    return_view = view!{cx,<button>"Broken :("</button>}
                }
            return_view
        }
        </Suspense>
        </div>
        <div class="action">
        <Suspense fallback=move || view!{cx,<p>"loading..."</p>}>
        {
            let mut return_view = view!{cx,<button></button>};
            if let Some(value) = state_led_2.read(cx)
                .and_then(|res| res.ok())
                {
                    if(value){
                        return_view = view!{cx,<button on:click=led_2_pressed class="on">"OUTDOOR"</button>}
                    }
                    else{
                        return_view = view!{cx,<button on:click=led_2_pressed>"OUTDOOR"</button>}
                    }
                } else {
                    // Handle the case where the result is None or contains an error
                    return_view = view!{cx,<button on:click=led_2_pressed>"Broken :("</button>}
                }
            return_view
        }
        </Suspense>
        </div>

        </div>
        <button on:click=on_click>"press me"</button>
        <p>{move || reactive_list.get().into_iter().len()}</p> 
        <For 
        each={move || reactive_list.get().into_iter()}
        key={|el| *el}
        view=move |cx, value| {
            view! {cx,
            <p>{value}</p>
            }
        }
        />
    }
}
