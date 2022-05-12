use dioxus::desktop::use_window;
use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let window = use_window(&cx);

    let level = use_state(&cx, || 1.0);
    cx.render(rsx! {
        input {
            r#type: "number",
            value: "{level}",
            oninput: |e| {
                let num = e.value.parse::<f64>().unwrap_or(1.0);
                level.set(num);
                window.set_zoom_level(num);
            }
        }
    })
}
