use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonA setter=set_toggled/>
    }
}

#[component]
pub fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <label>
            "Type a number"
            <input type="number" on:inout=on_input/>
            <p>
                "You entered "
                <strong>{value}</strong>
            </p>
        </label>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
