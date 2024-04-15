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
