// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{html::button, prelude::*};

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(app);
}

// define a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            buttons(cx, "red".to_string())
        }
    })
}
