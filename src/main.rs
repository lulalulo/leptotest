use leptos::*;
use leptos::ev::MouseEvent;
use gloo_timers::future::TimeoutFuture;

async fn load_data(value: i32) -> i32 {
    //fake a 1 sec delay
    TimeoutFuture::new(millis: 1_000).await;
    value * 10
}

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <Layout/>
    }
}

/*
 * PARENT N CHILD COMMUNICATION
 * **/
#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}

/*
#[component]
pub fn ButtonB(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}
*/
#[component]
pub fn ButtonB<F>(on_click: F) -> impl IntoView 
where 
    F: Fn(MouseEvent) + 'static 
{
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}

#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>"Toggle"</button>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD/>
        </div>
    }
}

#[component]
pub fn ButtonD<F>() -> impl IntoView {
    todo!()
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
