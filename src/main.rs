use leptos::*;

#[component]
fn ProgressBar(
    progress: ReadSignal<i32>
    ) -> impl IntoView {
    view! {
        <progress 
            max="50"
            
            value=progress
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button 
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                //set_count.set(3)
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            //{move || count.get();}
            //{move || count()}
        </button>
        <ProgressBar progress=count/>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
