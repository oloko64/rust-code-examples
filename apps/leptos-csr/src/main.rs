use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let res = create_local_resource(|| (), |_| async { fetch_data().await });

    view! {
        class="container",
        <p>COUNTER:</p>
        <Suspense fallback=move || view! {<p>"Loading Github"</p> }>
            {
                move || {
                    // Normally you would use a <ErrorBoundary> here, but Result<String, String> have trait bound issues
                    res.get().map(|data| {
                        match data {
                            Ok(data) => view! {
                                <p>{data}</p>
                            },
                            Err(data) => view! {
                                <p>Err: {data}</p>
                            }
                        }
                    })
                }
            }
        </Suspense>
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

async fn fetch_data() -> Result<String, String> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let body = res.text().await.map_err(|e| e.to_string())?;
        Ok(body)
    } else {
        Err("Failed to fetch data".to_string())
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
