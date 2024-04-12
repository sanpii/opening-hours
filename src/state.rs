#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct State {
    pub errors: Vec<String>,
    pub index: usize,
    pub nodes: Vec<Node>,
    pub progress: u32,
    pub searching: bool,
    pub location: crate::Location,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Param {
    pub r#where: String,
    pub r#type: String,
    pub what: String,
    pub wo_hour: bool,
    pub wifi: bool,
    pub vegetarian: bool,
    pub vegan: bool,
}

impl Param {
    pub fn from(params: &leptos_router::ParamsMap, query: &leptos_router::ParamsMap) -> Self {
        Self {
            r#where: params.get("where").cloned().unwrap_or_default(),
            r#type: params.get("type").cloned().unwrap_or_default(),
            what: params.get("what").cloned().unwrap_or_default(),

            wo_hour: query.get("wo_hour").is_some(),
            wifi: query.get("wifi").is_some(),
            vegetarian: query.get("vegetarian").is_some(),
            vegan: query.get("vegan").is_some(),
        }
    }

    pub fn as_filter(&self, r#box: &[String]) -> String {
        let mut filter = String::new();

        if !self.wo_hour {
            filter += "[\"opening_hours\"]";
        }

        if self.wifi {
            filter += "[\"internet_access\"=\"wlan\"]";
        }

        if self.vegetarian {
            filter += "[\"diet:vegetarian\"=\"yes\"]";
        }

        if self.vegan {
            filter += "[\"diet:vegan\"=\"yes\"]";
        }

        if self.r#type != "all" {
            filter += &format!("[\"amenity\"=\"{}\"]", self.r#type);
        }

        if !self.what.is_empty() {
            filter += &format!("[\"name\"~\".*{}.*\", i]", self.what);
        }

        filter += &format!("({},{},{},{});", r#box[0], r#box[2], r#box[1], r#box[3]);

        filter
    }

    pub fn as_url(&self) -> String {
        let mut url = "/".to_string();

        if !self.r#where.is_empty() {
            url += &format!("{}/", self.r#where);
        }

        if !self.r#type.is_empty() {
            url += &format!("{}/", self.r#type);
        } else {
            url += "all/"
        }

        if !self.what.is_empty() {
            url += &self.what;
        }

        let mut params = Vec::new();

        if self.wo_hour {
            params.push("wo_hour");
        }
        if self.wifi {
            params.push("wifi");
        }
        if self.vegetarian {
            params.push("vegetarian");
        }
        if self.vegan {
            params.push("vegan");
        }

        format!("{url}?{}", params.join("&"))
    }
}

impl Default for Param {
    fn default() -> Self {
        Self {
            r#where: String::new(),
            r#type: "all".to_string(),
            what: String::new(),
            wo_hour: false,
            wifi: false,
            vegetarian: false,
            vegan: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Node {
    pub amenity: Option<String>,
    pub favorite: bool,
    pub icon: Option<String>,
    pub id: u64,
    pub lat: f32,
    pub lon: f32,
    pub name: String,
    pub nodes: Vec<leptos_leaflet::Position>,
    pub phone: Option<String>,
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

    pub fn position(&self) -> leptos_leaflet::Position {
        leptos_leaflet::Position::new(self.lat as f64, self.lon as f64)
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
        ) -> Vec<leptos_leaflet::Position> {
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
            .filter(|x| x.tags.get("name").is_some() && x.tags.get("amenity").is_some())
            .map(|x| {
                let mut node = crate::state::Node::from(x);

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

impl From<&crate::Element> for Node {
    fn from(value: &crate::Element) -> Self {
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
            vegetarian: value.tags.get("diet:vegetarian") == Some(&"yes".to_string()),
            vegan: value.tags.get("diet:vegan") == Some(&"yes".to_string()),
            wifi: value.tags.get("internet_access") == Some(&"wlan".to_string()),
            favorite: value.is_favorite(),
            tags: value.tags.clone(),
        }
    }
}
