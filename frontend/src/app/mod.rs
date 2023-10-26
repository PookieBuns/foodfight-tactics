use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| {
            set_count.set(3);
        }>

            "Click me: " {move || count.get()}
        </button>
    }
}

