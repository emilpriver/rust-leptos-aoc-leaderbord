use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod api;
mod routes;
use routes::leaderbord::*;
use routes::nav::*;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());

    view! {
        cx,
        <div>
            <Stylesheet href="/static/style.css"/>
            <Meta name="description" content="Advent of code - Leaderbord"/>
            <Router>
                <Nav />
                <main>
                    <Routes>
                        <Route path="/" element=|cx| view! { cx,  <Leaderbord/> }/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn main() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            leptos::hydrate(body().unwrap(), move |cx| {
                view! { cx, <App/> }
            });
        }
    }
}
