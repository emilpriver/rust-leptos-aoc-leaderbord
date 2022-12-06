use leptos::*;


use crate::api;

#[component]
pub fn Leaderbord(cx: Scope) -> Element {
    let scores = create_resource(
        cx,
        move || false,
        move |_| async move { api::fetch_leaderbord::<api::AdventOfCodeResponse>().await },
    );

    log!("{:?}", scores.read());

    view! {
        cx,
        <div class="news-view">
            {move || match scores.read() {
                None => Some(view! { cx,  <p>"loading"</p> }),
                Some(None) => Some(view! { cx,  <p>"Error loading stories."</p> }),
                Some(Some(stories)) => {
                    log!("{:?}", stories);

                    Some(view! { cx,
                        <ul>
                        </ul>
                        })
                    }
                }
            }
        </div>
    }
}
