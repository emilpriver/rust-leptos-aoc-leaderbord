use crate::api;
use leptos::*;

#[component]
pub fn Leaderbord(cx: Scope) -> Element {
    let scores = create_resource(
        cx,
        || true,
        |_| async move { api::fetch_leaderbord().await },
    );

    view! {
        cx,
        <div class="scores-view view">
            <Transition
                fallback=view! { cx,  <p>"Loading..."</p> }
            >
                {move || match scores.read() {
                    None => Some(view! { cx,  <p>"loading"</p> }),
                    Some(None) => Some(view! { cx,  <p>"Error loading leaderbord."</p> }),
                    Some(Some(members)) => {
                        Some(view! { cx,
                            <ul>
                                <For each=move || members.clone() key=|x| x.len()>{
                                    move |cx: Scope, scores: &Vec<api::Score>| {
                                        view! {cx, <Group group=scores.clone() />}
                                    }
                                }</For>
                            </ul>
                        })
                    }
                }}
            </Transition>
        </div>
    }
}

#[component]
fn Group(cx: Scope, group: Vec<api::Score>) -> Element {
    view! {
        cx,
        <li>
            <ul>
                <For each=move || group.clone() key=|x| x.id>{
                move |cx: Scope, score: &api::Score| {
                    view! {
                        cx,
                        <li>
                            {format!("{:?} - {:?}", score.name, score.stars)}
                        </li>
                        }
                    }
                }
                </For>
            </ul>
        </li>
    }
}
