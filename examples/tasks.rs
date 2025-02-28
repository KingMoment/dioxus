//! Example: README.md showcase
//!
//! The example from the README.md.

use dioxus::prelude::*;
use std::time::Duration;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let count = use_state(&cx, || 0);

    use_future(&cx, move || {
        let count = UseState::for_async(&count);
        // for_async![count];
        async move {
            // loop {
            //     tokio::time::sleep(Duration::from_millis(1000)).await;
            //     count += 1;
            // }
        }
    });

    cx.render(rsx! {
        div {
            h1 { "Current count: {count}" }
            button {
                onclick: move |_| count.set(0),
                "Reset the count"
            }
        }
    })
}
