#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Param {
    pub open: bool,
    pub r#type: String,
    pub r#where: String,
    pub vegan: bool,
    pub vegetarian: bool,
    pub what: String,
    pub wifi: bool,
    pub wo_hour: bool,
}

impl Param {
    pub fn from(
        params: &leptos_router::params::ParamsMap,
        query: &leptos_router::params::ParamsMap,
    ) -> Self {
        Self {
            open: query.get("open").is_some(),
            r#type: params.get("type").unwrap_or_default(),
            r#where: params.get("where").unwrap_or_default(),
            vegan: query.get("vegan").is_some(),
            vegetarian: query.get("vegetarian").is_some(),
            what: params.get("what").unwrap_or_default(),
            wifi: query.get("wifi").is_some(),
            wo_hour: query.get("wo_hour").is_some(),
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
            filter += &format!("[\"amenity\"=\"{}\"]", self.r#type.trim());
        }

        if !self.what.is_empty() {
            filter += &format!("[\"name\"~\".*{}.*\", i]", self.what.trim());
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

        if self.open {
            params.push("open");
        }
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
            open: false,
            r#type: "all".to_string(),
            r#where: String::new(),
            vegan: false,
            vegetarian: false,
            what: String::new(),
            wifi: false,
            wo_hour: false,
        }
    }
}
