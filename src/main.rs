use leptos::*;
use leptos::ev::MouseEvent;
use gloo_timers::future::TimeoutFuture;

async fn load_data(value: i32) -> i32 {
    //fake a 1 sec delay
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
pub fn App() -> impl IntoView {
    let (count: ReadSignal<i32>, set_count: WriteSignal<i32>) = create_signal(0);

    // resource that takes two args after scope

    let async_data: Resource<i32, i32> = create_resource(
        source: count,
        fetcher: |value: i32| async move { load_data(value).await },
    );
    
    let stable: Resource<(), i32> = create_resource(source: || (), fetcher: |_| async move {load_data(1).await });

    let async_result: || -> String = move || {
        async_data Resource<i32, i32>
            .read() Option<i32>
            .map(|value: i32| format! ("Server returned {value:?}")) Option<String>
            .unwrap_or_else(|| "Loading...".into())
    };

    let loading: Signal<bool> = async_data.loading();
    let is_loading: || -> &str = move || if loading() {"Loading"} else { "Idle." };

    view! {
       <button
           on:click=move |_| {
               set_count.update(|n| *n += 1);
           }
       >
           "Click me"
       </button>
       <p>
            <code>"stable"</code>": " {move || stable.read()}
       </p>
       <p>
            <code>"count"</code>": " {count}
       </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
            {is_loading}
        </p> 
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
