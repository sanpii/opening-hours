use leptos::prelude::*;

#[leptos::component]
pub(crate) fn App() -> impl leptos::IntoView {
    use leptos_router::components::{Route, Router, Routes};

    leptos::view! {
        <header>
            <nav class="navbar">
                <a class="navbar-brand" href="/">Horaires</a>
            </nav>
        </header>

        <Router>
            <Routes fallback=|| "This page could not be found.">
                <Route path=leptos_router::path!("about") view=About />
                <Route path=leptos_router::path!("/:where?/:type?/:what?") view=Index />
                <Route path=leptos_router::path!("/*any") view=NotFound />
            </Routes>
        </Router>

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
    let state = RwSignal::new(crate::State::default());

    let params = leptos_router::hooks::use_params_map();
    let query = leptos_router::hooks::use_query_map();

    let param = RwSignal::new(crate::Param::from(
        &params.get_untracked(),
        &query.get_untracked(),
    ));
    let _ = Effect::watch(
        move || (params.get(), query.get()),
        move |(params, query), _, _| param.set(crate::Param::from(params, query)),
        false,
    );

    LocalResource::new(move || do_search(param.get(), state));

    let have_no_result = move || {
        !state.get().searching
            && !param.get().r#where.is_empty()
            && state.get().errors.is_empty()
            && state.get().nodes.is_empty()
    };

    let have_more = move || state.get().index * 20 < state.get().nodes.len();
    let navigate = leptos_router::hooks::use_navigate();

    leptos::view! {
        <crate::Form param=param.read_only() on_search=move |p| {
            navigate(&p.as_url(), Default::default());
            param.set(p);
        } />

        <leptos::error::ErrorBoundary fallback=move |_| leptos::view! { <p>Error</p> }>
            <Show when=move || state.get().searching>
                <div class="progress">
                    <div class="progress-bar" style:width=move || format!("{}%", state.get().progress)></div>
                </div>
            </Show>
            <Show when=have_no_result>
                <div class="alert alert-warning">Aucun résultat.</div>
            </Show>
            <Show when=move || !state.get().nodes.is_empty()>
                <crate::Search state=state.read_only() />
                <Show when=have_more>
                    <div>
                        <button on:click=move |_| state.update(|s| s.index += 1) class="btn btn-primary center-block">Load more</button>
                    </div>
                </Show>
            </Show>
        </leptos::error::ErrorBoundary>
    }
}

async fn do_search(param: crate::Param, state: RwSignal<crate::State>) -> crate::Result {
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

fn push(state: RwSignal<crate::State>) {
    state.update(|s| {
        s.searching = true;
        s.progress += 25;
    });
}

async fn location(r#where: &str) -> crate::Result<crate::Location> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search.php?q={}&format=jsonv2",
        r#where
    );

    let location = reqwest::get(&url).await?.json::<Vec<_>>().await?.remove(0);

    Ok(location)
}

async fn update_nodes(param: &crate::Param, r#box: &[String]) -> crate::Result<crate::Overpass> {
    let filter = param.as_filter(r#box);
    let request = format!("[out:json][timeout:25]; (way{filter} >; node{filter}); out+body;");

    let url = format!("https://overpass-api.de/api/interpreter?data={request}");

    let nodes = reqwest::get(&url).await?.json::<crate::Overpass>().await?;

    Ok(nodes)
}
fn transform_nodes(nodes: crate::Overpass) -> Vec<crate::Node> {
    let mut nodes: Vec<crate::Node> = nodes.into();
    nodes.sort();

    nodes
}
