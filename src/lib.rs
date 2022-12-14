#![feature(slice_group_by)]
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod api;
mod routes;
use routes::leaderbord::*;
use routes::nav::*;

const PKG_PATH: &str = "/pkg/advent_of_code_dashboard";


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

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            console_error_panic_hook::set_once();
            leptos::hydrate(body().unwrap(), move |cx| {
                view! { cx, <App /> }
            });
        }
    } else {
        use worker::*;
        use dotenv::dotenv;
        use worker::ResponseBody;

        fn log_request(req: &Request) {
            console_log!(
                "{} - [{}], located at: {:?}, within: {}",
                Date::now().to_string(),
                req.path(),
                req.cf().coordinates().unwrap_or_default(),
                req.cf().region().unwrap_or_else(|| "unknown region".into())
            );
        }

        #[event(fetch)]
        pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
            log_request(&req);
            
            dotenv().ok();

            // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
            // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
            // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
            let router = Router::new();

            // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
            // functionality and a `RouteContext` which you can use to  and get route parameters and
            // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
            router
                // serve JS to load Wasm
                // this section is kind of a mess; ideally it could point to static files rather than inlining them and serving like this
                .get(&format!("{PKG_PATH}.js"), |_, _| {
                    let mut headers = Headers::new();
                    headers.set("Content-Type", "text/javascript")?;
                    let body = ResponseBody::Body(include_str!("../pkg/advent_of_code_dashboard.js").as_bytes().to_vec());
                    Ok(
                        Response::from_body(body)?
                            .with_headers(headers)
                    )
                })
                .get("/pkg/snippets/leptos_dom-68e8edfe5e6c8b92/inline0.js", |_, _| {
                    let mut headers = Headers::new();
                    headers.set("Content-Type", "text/javascript")?;
                    let body = ResponseBody::Body(include_str!("../pkg/snippets/leptos_dom-68e8edfe5e6c8b92/inline0.js").as_bytes().to_vec());
                    Ok(
                        Response::from_body(body)?
                            .with_headers(headers)
                    )
                })
                .get(&format!("{PKG_PATH}_bg.wasm"), |_, _| {
                    let mut headers = Headers::new();
                    headers.set("Content-Type", "application/wasm")?;
                    let body = ResponseBody::Body(include_bytes!("../pkg/advent_of_code_dashboard_bg.wasm").to_vec());
                    Ok(
                        Response::from_body(body)?
                            .with_headers(headers)
                    )
                })
                .get("/worker-version", |_, ctx| {
                    let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
                    Response::ok(version)
                })
                .get("/", |_, _| Response::from_html(render_html_page(&render_to_string(|cx| view! { cx, <App/> }))))
                .run(req, env)
                .await
        }
    }
}


fn render_html_page(body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="modulepreload" href="{PKG_PATH}.js">
                <link rel="preload" href="{PKG_PATH}_bg.wasm" as="fetch" type="application/wasm" crossorigin="">
                <script type="module">import init, {{ hydrate }} from '{PKG_PATH}.js'; init('{PKG_PATH}_bg.wasm').then(hydrate);</script>
            </head>
            <body>
                {body}
            </body>
        </html>"#
    )
}
