#![warn(warnings)]

mod app;
mod form;
mod search;
mod state;

use app::App;
use form::Form;
use search::Search;
use state::State;

#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize)]
#[non_exhaustive]
struct Location {
    //pub place_id: u32,
    //pub licence: String,
    //pub osm_type: String,
    //pub osm_id: u32,
    //pub lat: String,
    //pub lon: String,
    //pub category: String,
    //pub r#type: String,
    //pub place_rank: u32,
    //pub importance: f32,
    //pub addresstype: String,
    //pub name: String,
    //pub display_name: String,
    pub boundingbox: Vec<String>,
}

impl From<Location> for leptos_leaflet::leaflet::LatLngBounds {
    fn from(value: Location) -> Self {
        let corner1 = leptos_leaflet::leaflet::LatLng::new(
            value.boundingbox[0].parse().unwrap(),
            value.boundingbox[2].parse().unwrap(),
        );
        let corner2 = leptos_leaflet::leaflet::LatLng::new(
            value.boundingbox[1].parse().unwrap(),
            value.boundingbox[3].parse().unwrap(),
        );
        leptos_leaflet::leaflet::LatLngBounds::new(&corner1, &corner2)
    }
}

impl From<Location> for leptos_leaflet::Position {
    fn from(value: Location) -> Self {
        let corner1 = leptos_leaflet::leaflet::LatLng::new(
            value.boundingbox[0].parse().unwrap(),
            value.boundingbox[2].parse().unwrap(),
        );
        let corner2 = leptos_leaflet::leaflet::LatLng::new(
            value.boundingbox[1].parse().unwrap(),
            value.boundingbox[3].parse().unwrap(),
        );

        Self::new(
            (corner1.lat() + corner2.lat()) / 2.,
            (corner1.lng() + corner2.lng()) / 2.,
        )
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    leptos::mount_to_body(|| leptos::view! { <App /> })
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[non_exhaustive]
pub(crate) struct Overpass {
    //pub version: f32,
    //pub generator: String,
    pub elements: Vec<Element>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct Element {
    pub id: u64,
    pub r#type: String,
    #[serde(default)]
    pub lat: f32,
    #[serde(default)]
    pub lon: f32,
    #[serde(default)]
    pub nodes: Vec<u64>,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

impl Element {
    pub fn name(&self) -> String {
        self.tags
            .get("name")
            .or_else(|| self.tags.get("amenity"))
            .cloned()
            .unwrap_or_else(|| "?".to_string())
    }

    pub fn state(&self) -> opening_hours::RuleKind {
        let Some(opening_hours) = self.tags.get("opening_hours") else {
            return opening_hours::RuleKind::Unknown;
        };

        let opening = match opening_hours::OpeningHours::parse(opening_hours) {
            Ok(opening) => opening,
            Err(_) => return opening_hours::RuleKind::Unknown,
        };
        let now = chrono::Local::now().naive_local();

        opening
            .state(now)
            .unwrap_or(opening_hours::RuleKind::Unknown)
    }

    pub fn icon(&self) -> Option<String> {
        let amenity = self.tags.get("amenity")?;

        let icon = if amenity == "bicycle_parking" {
            "oc-parking-bicycle".to_string()
        } else {
            format!("oc-{}", amenity.replace('_', "-"))
        };

        Some(icon)
    }

    pub fn website(&self) -> Option<String> {
        self.tags
            .get("website")
            .or_else(|| self.tags.get("contact:website"))
            .cloned()
    }

    pub fn is_favorite(&self) -> bool {
        use gloo::storage::Storage;

        let favorites = gloo::storage::LocalStorage::get::<Vec<u64>>("favorites");

        favorites.map(|x| x.contains(&self.id)).unwrap_or_default()
    }

    pub fn position(&self) -> leptos_leaflet::Position {
        leptos_leaflet::Position::new(self.lat as f64, self.lon as f64)
    }
}
