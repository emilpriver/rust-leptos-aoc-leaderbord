use leptos::*;
use leptos_router::*;

use crate::api;

#[component]
pub fn Leaderbord(cx: Scope) -> Element {
    let scores = create_resource(
        cx,
        move || false,
        move |_| async move { api::fetch_leaderbord::<api::AdventOfCodeResponse>().await },
    );

    view! {
        cx,
        <div class="news-view">
        </div>
    }
}
