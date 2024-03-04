use leptos::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
fn App() -> impl IntoView {
    //start with a set of three rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        }
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {

    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
