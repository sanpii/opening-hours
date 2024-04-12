use leptos::{SignalGet, SignalGetUntracked, SignalUpdate};

#[leptos::component]
pub(crate) fn Form<F>(
    param: leptos::ReadSignal<crate::state::Param>,
    on_search: F,
) -> impl leptos::IntoView
where
    F: Fn(crate::state::Param) + 'static,
{
    let param = leptos::create_rw_signal(param.get_untracked());

    leptos::view! {
        <form>
            <div class="input-group mb-3">
                <input
                    type="text"
                    value=move || param.get().r#where
                    placeholder="Où ?"
                    class="form-control"
                    required
                    on:input=move |ev| {
                        param.update(|p| p.r#where = leptos::event_target_value(&ev));
                    }
                />
            </div>

            <div class="input-group mb-3">
                <select class="form-control" on:change=move |ev| {
                    param.update(|p| p.r#type = leptos::event_target_value(&ev));
                }>
                    <option value="all">Quoi ?</option>
                    <leptos::For each=move || TYPES key=|value| value.to_string() let:ty>
                        <option value=ty selected=move || param.get().r#type == ty>{ ty }</option>
                    </leptos::For>
                </select>
            </div>

            <div class="input-group mb-3">
                <input
                    type="text"
                    value=move || param.get().what
                    placeholder="Nom ?"
                    class="form-control"
                    on:input=move |ev| {
                        param.update(|p| p.what = leptos::event_target_value(&ev));
                    }
                />
            </div>

            <div class="checkbox mb-3">
                <Checkbox
                    value=move || param.get().wo_hour
                    on_toggle=move |value| param.update(|p| p.wo_hour = value)
                >Sans horaire</Checkbox>
                <Checkbox
                    value=move || param.get().wifi
                    on_toggle=move |value| param.update(|p| p.wifi = value)
                >Avec wifi</Checkbox>
                <leptos::Show when=move || param.get().r#type == "restaurant">
                    <Checkbox
                        value=move || param.get().vegetarian
                        on_toggle=move |value| param.update(|p| p.vegetarian = value)
                    >Végétarien</Checkbox>
                    <Checkbox
                        value=move || param.get().vegan
                        on_toggle=move |value| param.update(|p| p.vegan = value)
                    >Vegan</Checkbox>
                </leptos::Show>
            </div>

            <button class="btn btn-primary" on:click=move |ev| {
                ev.prevent_default();
                on_search(param.get());
            }>Rechercher</button>
        </form>
    }
}

#[leptos::component]
fn Checkbox<I, O>(children: leptos::ChildrenFn, value: I, on_toggle: O) -> impl leptos::IntoView
where
    I: Fn() -> bool + 'static,
    O: Fn(bool) + 'static,
{
    leptos::view! {
        <div class="form-check form-check-inline">
            <input
                type="checkbox"
                class="form-check-input"
                value=value
                on:input=move |ev| on_toggle(leptos::event_target_checked(&ev))
            />
            <label class="form-check-label">{ children }</label>
        </div>
    }
}

static TYPES: [&str; 100] = [
    "animal_boarding",
    "animal_shelter",
    "arts_centre",
    "atm",
    "baby_hatch",
    "bank",
    "bar",
    "bbq",
    "bench",
    "bicycle parking",
    "bicycle rental",
    "bicycle_repair_station",
    "biergarten",
    "boat_sharing",
    "brothel",
    "bureau de change",
    "bus_station",
    "cafe",
    "car rental",
    "car sharing",
    "car wash",
    "casino",
    "charging_station",
    "cinema",
    "clinic",
    "clock",
    "college",
    "community_centre",
    "courthouse",
    "coworking_space",
    "crematorium",
    "crypt",
    "dentist",
    "doctors",
    "dojo",
    "drinking_water",
    "embassy",
    "ev_charging",
    "fast food",
    "ferry_terminal",
    "firepit",
    "fire_station",
    "food court",
    "fountain",
    "fuel",
    "gambling",
    "game_feeding",
    "grave_yard",
    "grit_bin",
    "gym",
    "hospital",
    "hunting_stand",
    "ice_cream",
    "kindergarten",
    "kneipp_water_cure",
    "library",
    "marketplace",
    "motorcycle parking",
    "nightclub",
    "nursing_home",
    "parking",
    "parking_entrance",
    "parking_space",
    "pharmacy",
    "photo_booth",
    "place of worship",
    "planetarium",
    "police",
    "post_box",
    "post_office",
    "prison",
    "pub",
    "public_bookcase",
    "public_building",
    "ranger_station",
    "recycling",
    "register_office",
    "rescue_station",
    "restaurant",
    "sauna",
    "school",
    "shelter",
    "shower",
    "social_centre",
    "social_facility",
    "stripclub",
    "studio",
    "swingerclub",
    "taxi",
    "telephone",
    "theatre",
    "toilets",
    "townhall",
    "university",
    "vending_machine",
    "veterinary",
    "waste_basket",
    "waste_disposal",
    "watering_place",
    "water_point",
];
