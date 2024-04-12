use leptos::{SignalGet, SignalGetUntracked, SignalSet, SignalUpdate};

#[leptos::component]
pub(crate) fn App() -> impl leptos::IntoView {
    leptos::view! {
        <header>
            <nav class="navbar">
                <a class="navbar-brand" href="/">Horaires</a>
            </nav>
        </header>

        <leptos_router::Router>
            <leptos_router::Routes>
                <leptos_router::Route path="about" view=About />
                <leptos_router::Route path="/:where?/:type?/:what?" view=Index />
                <leptos_router::Route path="/*any" view=NotFound />
            </leptos_router::Routes>
        </leptos_router::Router>

        <footer>
            <a href="/about">À propos</a>
        </footer>
    }
}

#[leptos::component]
pub(crate) fn About() -> impl leptos::IntoView {
    let html = r#"
        <p>
            Ce site utilise les données d’<a href="http://www.openstreetmap.org/">OpenStreetMap</a>
            pour afficher les horaires d’ouverture.
        </p>

        <p>
            Si vous souhaitez ajouter ou corriger des horaires, il suffit de renseigner le
            tag <a href="https://wiki.openstreetmap.org/wiki/FR:Key:opening_hours">opening_hours</a>,
            <a href="https://wiki.openstreetmap.org/wiki/FR:Key:amenity">amenity</a> (pour
            le type d’établissement) et <a href="https://wiki.openstreetmap.org/wiki/FR:Key:name">name</a>.
        </p>

        <p>
            Concernant le code du site, il est également libre et peux être récupéré sur
            <a href="https://github.com/sanpii/opening-hours">github</a>.
        </p>

        <p>
            Donnée &copy; contributeurs <a href="http://openstreetmap.org">OpenStreetMap</a>,
            <a href="http://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>,
            Imagerie © <a href="http://openstreetmap.org">OpenStreetMap</a>.
        </p>
    "#;

    leptos::view! {
        <div inner_html=html />
    }
}

#[leptos::component]
pub(crate) fn NotFound() -> impl leptos::IntoView {
    leptos::view! {
        <div>{ "Not found" }</div>
    }
}

#[leptos::component]
pub(crate) fn Index() -> impl leptos::IntoView {
    let state = leptos::create_rw_signal(crate::State::default());

    let params = leptos_router::use_params_map();
    let query = leptos_router::use_query_map();

    let param = leptos::create_rw_signal(crate::state::Param::from(
        &params.get_untracked(),
        &query.get_untracked(),
    ));
    let _ = leptos::watch(
        move || (params.get(), query.get()),
        move |(params, query), _, _| param.set(crate::state::Param::from(params, query)),
        false,
    );

    leptos::create_local_resource(move || (param.get(), state), do_search);

    let have_no_result = move || {
        !state.get().searching
            && !param.get().r#where.is_empty()
            && state.get().errors.is_empty()
            && state.get().nodes.is_empty()
    };

    let have_more = move || state.get().index * 20 < state.get().nodes.len();

    leptos::view! {
        <crate::Form param=param.read_only() on_search=move |p| {
            let navigate = leptos_router::use_navigate();
            navigate(&p.as_url(), Default::default());
            param.set(p);
        } />

        <leptos::ErrorBoundary fallback=move |_| leptos::view! { <p>Error</p> }>
            <leptos::Show when=move || state.get().searching>
                <div class="progress">
                    <div class="progress-bar" style:width=move || format!("{}%", state.get().progress)></div>
                </div>
            </leptos::Show>
            <leptos::Show when=have_no_result>
                <div class="alert alert-warning">Aucun résultat.</div>
            </leptos::Show>
            <leptos::Show when=move || !state.get().nodes.is_empty()>
                <crate::Search state=state.read_only() />
                <leptos::Show when=have_more>
                    <div>
                        <button on:click=move |_| state.update(|s| s.index += 1) class="btn btn-primary center-block">Load more</button>
                    </div>
                </leptos::Show>
            </leptos::Show>
        </leptos::ErrorBoundary>
    }
}

async fn do_search(
    (param, state): (crate::state::Param, leptos::RwSignal<crate::State>),
) -> leptos::error::Result<()> {
    if param.r#where.is_empty() {
        return Ok(());
    }

    push(state);

    let location = location(&param.r#where).await?;

    push(state);

    let nodes = update_nodes(&param, &location.boundingbox).await?;

    push(state);

    let nodes = transform_nodes(nodes);

    push(state);

    state.update(|s| {
        s.location = location;
        s.searching = false;
        s.nodes = nodes;
        s.index = 1;
    });

    Ok(())
}

fn push(state: leptos::RwSignal<crate::State>) {
    state.update(|s| {
        s.searching = true;
        s.progress += 25;
    });
}

async fn location(r#where: &str) -> leptos::error::Result<crate::Location> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search.php?q={}&format=jsonv2",
        r#where
    );

    let location = reqwest::get(&url).await?.json::<Vec<_>>().await?.remove(0);

    Ok(location)
}

async fn update_nodes(
    param: &crate::state::Param,
    r#box: &[String],
) -> leptos::error::Result<crate::Overpass> {
    let filter = param.as_filter(r#box);
    let request = format!("[out:json][timeout:25]; (way{filter} >; node{filter}); out+body;");

    let url = format!("https://overpass-api.de/api/interpreter?data={request}");

    let nodes = reqwest::get(&url).await?.json::<crate::Overpass>().await?;

    Ok(nodes)
}
fn transform_nodes(nodes: crate::Overpass) -> Vec<crate::state::Node> {
    let mut nodes: Vec<crate::state::Node> = nodes.into();
    nodes.sort();

    nodes
}
