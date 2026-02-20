#![warn(warnings)]

mod app;
mod form;
mod location;
mod node;
mod overpass;
mod param;
mod search;
mod taginfo;

use app::App;
use form::Form;
use location::Location;
use node::Node;
use overpass::Overpass;
use param::Param;
use search::Search;
use taginfo::Taginfo;

pub(crate) type Result<T = ()> = std::result::Result<T, leptos::error::Error>;

#[derive(Debug)]
pub(crate) enum Error {
    LocationNotFound(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::LocationNotFound(location) => format!("Location not found: {location}"),
        };

        f.write_str(&s)
    }
}

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
    leptos::mount::mount_to_body(|| leptos::view! { <App /> })
}

fn favorites() -> Vec<u64> {
    use gloo::storage::Storage as _;

    gloo::storage::LocalStorage::get::<Vec<u64>>("favorites").unwrap_or_default()
}
