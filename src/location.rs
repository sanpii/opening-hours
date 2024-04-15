#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize)]
#[non_exhaustive]
pub struct Location {
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
