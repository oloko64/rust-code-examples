use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button class="one"
            on:click=move |_| {
                let val = ReadSignal::get(&count);
                if val == 3 {
                    WriteSignal::set(&set_count, 0);
                } else {
                    WriteSignal::set(&set_count, val + 1);
                }
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
           <App />
        }
    })
}
