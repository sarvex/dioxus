use dioxus::{events::onmouseover, prelude::*};

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            background: "red",
            onmouseenter: |_| {
                println!("mouse entered");
            },
            "onmouseenter": "console.log('mouse entered')",
            onmouseleave: |_| {
                println!("mouse leave");
            },
            "onmouseleave": "console.log('mouse left')",
            onmouseover: move |_| {
                println!("mouse over");
            },
            onmouseout: move |_| {
                println!("mouse out");
            },
            "hi"
        }
    })
}
