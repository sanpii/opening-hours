#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Node {
    pub amenity: Option<String>,
    pub favorite: bool,
    pub icon: Option<String>,
    pub id: u64,
    pub lat: f32,
    pub lon: f32,
    pub name: String,
    pub nodes: Vec<leptos_leaflet::prelude::Position>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub state: opening_hours::RuleKind,
    pub tags: std::collections::HashMap<String, String>,
    pub vegan: bool,
    pub vegetarian: bool,
    pub wifi: bool,
}

impl Node {
    pub fn color(&self) -> &'static str {
        match self.state {
            opening_hours::RuleKind::Open => "green",
            opening_hours::RuleKind::Closed => "red",
            opening_hours::RuleKind::Unknown => "black",
        }
    }

    pub fn position(&self) -> leptos_leaflet::prelude::Position {
        leptos_leaflet::prelude::Position::new(self.lat as f64, self.lon as f64)
    }

    pub fn opening_hours(&self) -> Option<opening_hours::OpeningHours> {
        self.tags
            .get("opening_hours")
            .and_then(|x| opening_hours::OpeningHours::parse(x).ok())
    }

    pub fn favorite(&mut self) {
        use gloo::storage::Storage;

        let mut favorites =
            gloo::storage::LocalStorage::get::<Vec<u64>>("favorites").unwrap_or_default();

        if let Some(x) = favorites.iter().position(|x| x == &self.id) {
            self.favorite = false;
            favorites.remove(x);
        } else {
            self.favorite = true;
            favorites.push(self.id);
        }

        gloo::storage::LocalStorage::set("favorites", favorites).ok();
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.favorite && !other.favorite {
            std::cmp::Ordering::Less
        } else if !self.favorite && other.favorite {
            std::cmp::Ordering::Greater
        } else {
            self.name.cmp(&other.name)
        }
    }
}

impl std::cmp::Eq for Node {}

impl From<crate::Overpass> for Vec<Node> {
    fn from(value: crate::Overpass) -> Self {
        fn replace_ref_by_node(
            value: &crate::Overpass,
            refs: &[u64],
        ) -> Vec<leptos_leaflet::prelude::Position> {
            let mut nodes = Vec::new();

            for r#ref in refs {
                let n = value.elements.iter().find(|x| &x.id == r#ref).unwrap();
                nodes.push(n.position());
            }

            nodes
        }

        value
            .elements
            .iter()
            .filter(|x| x.tags.contains_key("name") && x.tags.contains_key("amenity"))
            .map(|x| {
                let mut node = Node::from(x);

                if x.r#type == "way" {
                    node.nodes = replace_ref_by_node(&value, &x.nodes);
                    node.lat = node.nodes[0].lat as f32;
                    node.lon = node.nodes[0].lng as f32;
                }

                node
            })
            .collect()
    }
}

impl From<&crate::overpass::Element> for Node {
    fn from(value: &crate::overpass::Element) -> Self {
        Self {
            id: value.id,
            nodes: Vec::new(),
            lat: value.lat,
            lon: value.lon,
            name: value.name(),
            amenity: value.tags.get("amenity").cloned(),
            phone: value.tags.get("phone").cloned(),
            state: value.state(),
            icon: value.icon(),
            website: value.website(),
            vegetarian: value.tags.get("diet:vegetarian") == Some(&"yes".to_string()),
            vegan: value.tags.get("diet:vegan") == Some(&"yes".to_string()),
            wifi: value.tags.get("internet_access") == Some(&"wlan".to_string()),
            favorite: value.is_favorite(),
            tags: value.tags.clone(),
        }
    }
}
