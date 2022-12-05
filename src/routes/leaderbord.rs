use leptos::*;
use leptos_router::*;

use crate::api;

#[component]
pub fn Leaderbord(cx: Scope) -> Element {
    let params = use_params_map(cx);
    let scores = create_resource(
        cx,
        move || false,
        move |_| async move {
            api::fetch_leaderbord().await
        },
    );
    let (pending, set_pending) = create_signal(cx, false);

    view! {
        cx,
        <div class="news-view">
        </div>
    }
}
