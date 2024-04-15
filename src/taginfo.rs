#[derive(Clone, Debug, serde::Deserialize)]
#[non_exhaustive]
pub(crate) struct Taginfo {
    //pub url: String,
    //pub data_until: String,
    //pub page: usize,
    //pub rp: usize,
    //pub total: usize,
    pub data: Vec<Data>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[non_exhaustive]
pub(crate) struct Data {
    pub value: String,
    //pub count: usize,
    //pub fraction: f32,
    //pub in_wiki: bool,
    //pub description: String,
    //pub desclang: String,
    //pub descdir: String,
}

impl Data {
    pub fn icon(&self) -> Option<String> {
        let amenity = &self.value;

        let icon = match amenity.as_str() {
            "bicycle_parking" => "oc-parking-bicycle".to_string(),
            "bicycle_rental" => "oc-rental-bicycle".to_string(),
            "doctors" => "oc-doctor".to_string(),
            "parking" => "oc-parking-car".to_string(),
            "townhall" => "oc-town-hall".to_string(),
            amenity => format!("oc-{}", amenity.replace('_', "-")),
        };

        Some(icon)
    }
}
