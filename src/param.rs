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
