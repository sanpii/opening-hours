#![recursion_limit="128"]

#[derive(Default)]
struct Model {
    console: yew::services::ConsoleService,
    state: State,
}

#[derive(Debug, Default, serde_derive::Deserialize, serde_derive::Serialize)]
struct State {
    place: String,
    kind: String,
    what: String,
    wo_hour: bool,
    wifi: bool,
    vegetarian: bool,
    vegan: bool,
}

enum Msg {
    Search,
}

impl Model
{
    fn search(&mut self) -> yew::ShouldRender
    {
        true
    }

    fn form(&self) -> yew::Html<Self>
    {
        yew::html! {
<div>
    <fieldset class="form-group",>
        <input type="text", value=&self.state.place, placeholder="Où ?", class="form-control", />
    </fieldset>

    <fieldset class="form-group",>
        <input type="text", value=&self.state.kind, placeholder="Quoi ?", class="form-control", />
    </fieldset>

    <fieldset class="form-group",>
        <input type="text", value=&self.state.what, placeholder="Nom ?", class="form-control", />
    </fieldset>

    <div class="checkbox",>
        <label>
            <input type="checkbox", checked=self.state.wo_hour, /> { "Sans horaire" }
        </label>
        <label>
            <input type="checkbox", checked=self.state.wifi, /> { "Avec wifi" }
        </label>

        { self.restaurant_filter() }
    </div>

    <button onclick=|_| Msg::Search, class="btn btn-primary",>{ "Rechercher" }</button>
</div>
        }
    }

    fn restaurant_filter(&self) -> yew::Html<Self>
    {
        if self.state.kind == "restaurant" {
            yew::html! {
<label>
    <input type="checkbox", checked=self.state.vegetarian, /> { "Végétarien" }
</label>
<label>
    <input type="checkbox", checked=self.state.vegan, /> { "Vegan" }
</label>
            }
        }
        else {
            Self::nothing()
        }
    }

    fn map(&self) -> yew::Html<Self>
    {
        if self.state.place.is_empty() {
            Self::nothing()
        }
        else {
            yew::html! {
                <>{ "Map" }</>
            }
        }
    }

    fn nothing() -> yew::Html<Self>
    {
        yew::html! {
            <></>
        }
    }
}

impl yew::Component for Model
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self
    {
        Default::default()
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender
    {
        match msg {
            Msg::Search => {
                self.console.log(&format!("{:?}", self.state));
                self.search()
            },
        }
    }
}

impl yew::Renderable<Model> for Model {
    fn view(&self) -> yew::Html<Self>
    {
        yew::html! {
            <>
                { self.form() }
                { self.map() }
            </>
        }
    }
}

fn main()
{
    use stdweb::web::IParentNode;

    let element = stdweb::web::document()
        .query_selector("#app")
        .unwrap()
        .unwrap();

    yew::initialize();
    let app = yew::App::<Model>::new();
    app.mount(element);
    yew::run_loop();
}
