use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav(cx: Scope) -> Element {
    view! { cx,
        <header class="header">
            <nav class="inner">
                <A href="/">
                    <strong>"Leaderbord"</strong>
                </A>
            </nav>
        </header>
    }
}
