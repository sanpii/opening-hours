use chrono::{Datelike, Timelike};
use leptos::prelude::*;

#[leptos::component]
pub(crate) fn Search(state: ReadSignal<crate::State>) -> impl leptos::IntoView {
    let limit = move || state.get().nodes.len().min(state.get().index * 20);
    let nodes = Memo::new(move |_| state.get().nodes);
    let location = Memo::new(move |_| state.get().location);

    leptos::view! {
        <Map location=location nodes=nodes />

        <div>
            { limit }" résultats sur "{ move || state.get().nodes.len() }
        </div>

        <ul id="list">
            <For each=move || { state.get().nodes[..limit()].to_vec() } key=|node| node.id let:node>
                <Item node />
            </For>
        </ul>
    }
}

#[leptos::component]
pub(crate) fn Item(node: crate::Node) -> impl leptos::IntoView {
    let (node, node_set) = signal(node);

    let class = move || match node.get().state {
        opening_hours::RuleKind::Open => "open",
        opening_hours::RuleKind::Closed => "closed",
        opening_hours::RuleKind::Unknown => "",
    };

    let favorite = move |_| {
        node.get().favorite();
        node_set.update(|n| n.favorite = !n.favorite);
    };

    leptos::view! {
        <li class=class id=move || node.get().id>
            <div>
                <span class=move || node.get().icon></span>
                { move || node.get().name }
                <span class="favorite float-end" on:click=favorite>
                    <Show when=move || node.get().favorite>
                        <img src="/img/star.png" title="Supprimer des favoris" />
                    </Show>
                    <Show when=move || !node.get().favorite>
                        <img src="/img/empty-star.png" title="Ajouter au favoris" />
                    </Show>
                </span>
                <Show when=move || node.get().wifi>
                    <span class="wifi float-end">
                        <img src="/img/wifi.png" title="Wifi disponible" />
                    </span>
                </Show>
                <span class="diet">
                    <Show when=move || node.get().vegan>
                        <span class="float-end" title="Végétalien">+</span>
                    </Show>
                    <Show when=move || { node.get().vegetarian || node.get().vegan }>
                        <span class="float-end" title="Végétarien">V</span>
                    </Show>
                </span>
            </div>
            <div class="detail">
                <Timeline node=node />
                <div>
                    <Show when=move || node.get().website.is_some()>
                        <div>
                            <a href=move || node.get().website.unwrap()>
                                <span class="oc-computer"></span>
                                <span class="label">{ node.get().website.map(|mut x| {
                                    if x.len() > 20 {
                                        x.truncate(20);
                                        x + "…"
                                    } else {
                                        x
                                    }
                                }) }</span>
                            </a>
                        </div>
                    </Show>
                    <Show when=move || node.get().phone.is_some()>
                        <div>
                            <a href=move || format!("tel:{}", node.get().phone.unwrap())>
                                <span class="oc-telephone"></span>
                                <span class="label">{ node.get().phone }</span>
                            </a>
                        </div>
                    </Show>
                    <div>
                        <a href=move || format!("geo:{},{}", node.get().lat, node.get().lon)>
                            <span class="oc-guidepost"></span>
                            <span class="label">{ move || format!("{}, {}", node.get().lat, node.get().lon) }</span>
                        </a>
                    </div>
                </div>
            </div>
        </li>
    }
}

#[leptos::component]
pub(crate) fn Timeline(node: ReadSignal<crate::Node>) -> impl leptos::IntoView {
    let now = chrono::Local::now().naive_local();
    let date = now - chrono::Duration::days(now.weekday() as i64);

    leptos::view! {
        <div class="timeline">
            <div class="container legend">
                <div class="row">
                    <span class="day col-lg-1"></span>

                    <div class="col">
                        <For each=move || (0..24) key=|x| *x let:hour>
                            <span
                                class:label=move || hour % 5 == 0
                                class:font-weight-bold=move || hour == now.hour()
                                class:text-body-secondary=move || hour != now.hour()
                            >{ format!("{hour:02}") }</span>
                        </For>
                    </div>
                </div>
            </div>
            <div class="container">
                <For each=move || (0..7) key=|x| *x let:day>
                    <Day date=date.date() + chrono::Duration::days(day) node=node.get() />
                </For>
            </div>
        </div>
    }
}

#[leptos::component]
pub(crate) fn Day(date: chrono::NaiveDate, node: crate::Node) -> impl leptos::IntoView {
    static DOW: [&str; 7] = [
        "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche",
    ];

    let now = chrono::Local::now().naive_local();
    let start = date.and_hms_opt(0, 0, 0).unwrap();
    let end = date.and_hms_opt(23, 59, 59).unwrap();
    let weekday = DOW[date.weekday() as usize];

    leptos::view! {
        <div class="row">
            <span
                class="day col-lg-1"
                class:font-weight-bold=move || date.weekday() == now.weekday()
                class:text-muted=move || date.weekday() != now.weekday()
            >
                { weekday.chars().next() }<span class="label">{ weekday.get(1..) }</span>
            </span>
            <div class="col"><Progress start end node /></div>
        </div>
    }
}

#[leptos::component]
pub(crate) fn Progress(
    start: chrono::NaiveDateTime,
    end: chrono::NaiveDateTime,
    node: crate::Node,
) -> impl leptos::IntoView {
    let Some(oh) = node.opening_hours() else {
        return leptos::view! { <div></div> }.into_any();
    };

    #[derive(Clone)]
    struct Part {
        id: usize,
        size: f32,
        legend: String,
        comment: String,
        state: opening_hours::RuleKind,
    }

    let mut parts = Vec::new();

    for (id, range) in oh.iter_range(start, end).enumerate() {
        let start = range.range.start.time();
        let end = range.range.end.time();

        parts.push(Part {
            id,
            state: range.kind,
            legend: if range.kind == opening_hours::RuleKind::Open {
                format!(
                    "{:02}:{:02} - {:02}:{:02}",
                    start.hour(),
                    start.minute(),
                    end.hour(),
                    end.minute()
                )
            } else {
                String::new()
            },
            comment: range.comments.join("\n"),
            size: (end - start).num_minutes() as f32 * 100. / 1440.,
        });
    }

    leptos::view! {
        <div class="progress">
            <For each=move || parts.clone() key=|x| x.id let:part>
                <div
                    role="progressbar"
                    style=move || format!("width:{}%", part.size)
                    aria-valuenow=part.size
                    aria-valuemin="0"
                    aria-valuemax="100"
                    class="progress-bar"
                    class:bg-success=move || part.state == opening_hours::RuleKind::Open
                    class:bg-danger=move || part.state == opening_hours::RuleKind::Closed
                    title=part.legend.clone()
                >
                    <span class:d-lg-none=move || part.size < 11.>{ part.legend.clone() }</span>
                </div>
            </For>
        </div>
    }
    .into_any()
}

#[leptos::component]
pub(crate) fn Map(
    location: Memo<crate::Location>,
    nodes: Memo<Vec<crate::Node>>,
) -> impl leptos::IntoView {
    let center = leptos_leaflet::prelude::Position::from(location.get_untracked());
    let map = RwSignal::new_local(None::<leptos_leaflet::leaflet::Map>);
    Effect::new(move |_| {
        if let Some(map) = map.get() {
            map.fit_bounds(&location.get().into());
        }
    });

    leptos::view! {
        <leptos_leaflet::prelude::MapContainer style="height: 280px" map=map.write_only() center zoom=18. set_view=true>
            <leptos_leaflet::prelude::TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="" />
            <For each=move || nodes.get() key=|node| node.id let:node>
                <Node node />
            </For>
        </leptos_leaflet::prelude::MapContainer>
    }
}

#[leptos::component]
pub(crate) fn Node(node: crate::Node) -> impl leptos::IntoView {
    let node = Memo::new(move |_| node.clone());

    leptos::view! {
        <Show when=move || node.with(|x| x.nodes.is_empty())
            fallback=move || leptos::view! {
                <leptos_leaflet::prelude::Polygon positions=node.with(|x| x.nodes.clone()) color=node.with(|x| x.color())>
                    <Popup node />
                </leptos_leaflet::prelude::Polygon>
            }
        >
            <leptos_leaflet::prelude::Circle center=node.with(|x| x.position()) radius=5. color=node.with(|x| x.color())>
                <Popup node />
            </leptos_leaflet::prelude::Circle>
        </Show>
    }
}

#[leptos::component]
pub(crate) fn Popup(node: Memo<crate::Node>) -> impl leptos::IntoView {
    leptos::view! {
        <leptos_leaflet::prelude::Popup>
            <div>
                <span class=move || node.get().icon></span>
                { move || node.get().name }
                <span class="float-end">
                    <a class="oc-clock" href=move || format!("#{}", node.get().id)></a>
                </span>
                <State node />
            </div>
        </leptos_leaflet::prelude::Popup>
    }
}

#[leptos::component]
pub(crate) fn State(node: Memo<crate::Node>) -> impl leptos::IntoView {
    let state = move || match node.get().state {
        opening_hours::RuleKind::Open => "Ouvert",
        opening_hours::RuleKind::Closed => "Fermé",
        opening_hours::RuleKind::Unknown => "",
    };

    leptos::view! {
        <div>
            <span class=move || node.get().state.as_str()> { state }</span>
            { move || node.get().next_change().map(|x| format!(" · {x}")).unwrap_or_default() }
        </div>
    }
}
