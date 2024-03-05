use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal("B".to_string());
    view! {
        <select on:change=move |env| {
            let new_value = event_target_value(&env);
            set_value(new_value);
        }>
            <SelectOption value is="A"/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>
        </select>
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option
            value=is
            selected=move || value() == is
        >
            {is}
        </option>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
