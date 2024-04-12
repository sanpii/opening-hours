use chrono::{Datelike, Timelike};
use leptos::{SignalGet, SignalGetUntracked, SignalUpdate, SignalWith};

#[leptos::component]
pub(crate) fn Search(state: leptos::ReadSignal<crate::State>) -> impl leptos::IntoView {
    let limit = move || state.get().nodes.len().min(state.get().index * 20);
    let nodes = leptos::create_memo(move |_| state.get().nodes);
    let location = leptos::create_memo(move |_| state.get().location);

    leptos::view! {
        <Map location=location nodes=nodes />

        <div>
            { limit }" résultats sur "{ move || state.get().nodes.len() }
        </div>

        <ul id="list">
            <leptos::For each=move || { state.get().nodes[..limit()].to_vec() } key=|node| node.id let:node>
                <Item node />
            </leptos::For>
        </ul>
    }
}

#[leptos::component]
pub(crate) fn Item(node: crate::state::Node) -> impl leptos::IntoView {
    let (node, node_set) = leptos::create_signal(node);

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
                    <leptos::Show when=move || node.get().favorite>
                        <img src="/img/star.png" title="Supprimer des favoris" />
                    </leptos::Show>
                    <leptos::Show when=move || !node.get().favorite>
                        <img src="/img/empty-star.png" title="Ajouter au favoris" />
                    </leptos::Show>
                </span>
                <leptos::Show when=move || node.get().wifi>
                    <span class="wifi float-end">
                        <img src="/img/wifi.png" title="Wifi disponible" />
                    </span>
                </leptos::Show>
                <span class="diet">
                    <leptos::Show when=move || node.get().vegan>
                        <span class="float-end" title="Végétalien">+</span>
                    </leptos::Show>
                    <leptos::Show when=move || { node.get().vegetarian || node.get().vegan }>
                        <span class="float-end" title="Végétarien">V</span>
                    </leptos::Show>
                </span>
            </div>
            <div class="detail">
                <Timeline node=node />
                <div>
                    <leptos::Show when=move || node.get().phone.is_some()>
                        <div>
                            <a href=move || format!("tel:{}", node.get().phone.unwrap())>
                                <span class="oc-telephone"></span>
                                <span class="label">{ node.get().phone }</span>
                            </a>
                        </div>
                    </leptos::Show>
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
pub(crate) fn Timeline(node: leptos::ReadSignal<crate::state::Node>) -> impl leptos::IntoView {
    let now = chrono::Local::now().naive_local();
    let date = now - chrono::Duration::days(now.weekday() as i64);

    leptos::view! {
        <div class="timeline">
            <div class="container legend">
                <div class="row">
                    <span class="day col-lg-1"></span>

                    <div class="col">
                        <leptos::For each=move || (0..24) key=|x| *x let:hour>
                            <span
                                class:label=move || hour % 5 == 0
                                class:font-weight-bold=move || hour == now.hour()
                                class:text-muted=move || hour != now.hour()
                            >{ format!("{hour:02}") }</span>
                        </leptos::For>
                    </div>
                </div>
            </div>
            <div class="container">
                <leptos::For each=move || (0..7) key=|x| *x let:day>
                    <Day date=date.date() + chrono::Duration::days(day) node=node.get() />
                </leptos::For>
            </div>
        </div>
    }
}

#[leptos::component]
pub(crate) fn Day(date: chrono::NaiveDate, node: crate::state::Node) -> impl leptos::IntoView {
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
    node: crate::state::Node,
) -> impl leptos::IntoView {
    let Some(oh) = node.opening_hours() else {
        return leptos::view! { <div></div> };
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

    for (id, range) in oh.iter_range(start, end).unwrap().enumerate() {
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
            <leptos::For each=move || parts.clone() key=|x| x.id let:part>
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
            </leptos::For>
        </div>
    }
}

#[leptos::component]
pub(crate) fn Map(
    location: leptos::Memo<crate::Location>,
    nodes: leptos::Memo<Vec<crate::state::Node>>,
) -> impl leptos::IntoView {
    let center = leptos_leaflet::Position::from(location.get_untracked());
    let (map, map_set) = leptos::create_signal(None::<leptos_leaflet::leaflet::Map>);
    leptos::create_effect(move |_| {
        if let Some(map) = map.get() {
            map.fit_bounds(&location.get().into());
        }
    });

    leptos::view! {
        <leptos_leaflet::MapContainer style="height: 280px" map=map_set center zoom=18. set_view=true>
            <leptos_leaflet::TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="" />
            <leptos::For each=move || nodes.get() key=|node| node.id let:node>
                <Node node />
            </leptos::For>
        </leptos_leaflet::MapContainer>
    }
}

#[leptos::component]
pub(crate) fn Node(node: crate::state::Node) -> impl leptos::IntoView {
    let node = leptos::create_memo(move |_| node.clone());

    leptos::view! {
        <leptos::Show when=move || node.with(|x| x.nodes.is_empty())
            fallback=move || leptos::view! {
                <leptos_leaflet::Polygon positions=node.with(|x| x.nodes.clone()) color=node.with(|x| x.color())>
                    <Popup node />
                </leptos_leaflet::Polygon>
            }
        >
            <leptos_leaflet::Circle center=node.with(|x| x.position()) radius=5. color=node.with(|x| x.color())>
                <Popup node />
            </leptos_leaflet::Circle>
        </leptos::Show>
    }
}

#[leptos::component]
pub(crate) fn Popup(node: leptos::Memo<crate::state::Node>) -> impl leptos::IntoView {
    leptos::view! {
        <leptos_leaflet::Popup>
            <div>
                <span class=move || node.get().icon></span>
                move || node.get().name
            </div>
        </leptos_leaflet::Popup>
    }
}
