#![warn(warnings)]

mod app;
mod form;
mod location;
mod node;
mod overpass;
mod param;
mod search;

use app::App;
use form::Form;
use location::Location;
use node::Node;
use overpass::Overpass;
use param::Param;
use search::Search;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct State {
    pub errors: Vec<String>,
    pub index: usize,
    pub nodes: Vec<Node>,
    pub progress: u32,
    pub searching: bool,
    pub location: crate::Location,
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    leptos::mount_to_body(|| leptos::view! { <App /> })
}
