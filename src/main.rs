use leptos::*;
use leptos::ev::MouseEvent;

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonB on_click=move |_| set_toggled.update(|value| *value= !*value)/>
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

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
