use leptos::prelude::*;

#[leptos::component]
pub(crate) fn Form<F>(param: ReadSignal<crate::Param>, on_search: F) -> impl leptos::IntoView
where
    F: Fn(crate::Param) + 'static,
{
    let param = RwSignal::new(param.get_untracked());
    let types = LocalResource::new(move || async move {
        let url = format!(
            "https://taginfo.openstreetmap.org/api/4/key/values?key=amenity&filter=all&lang=fr&sortname=count&sortorder=desc&rp=50&page=1&query={}",
            param.get().r#type,
        );
        let mut taginfo = super::request::<crate::Taginfo>(&url).await.unwrap();
        taginfo.data.sort_by(|a, b| a.value.cmp(&b.value));

        taginfo
    });

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
                        param.update(|p| p.r#where = event_target_value(&ev));
                    }
                />
            </div>

            <div class="input-group mb-3">
                <input
                    type="text"
                    value=move || param.get().r#type
                    placeholder="Quoi ?"
                    class="form-control"
                    list="types"
                    on:input=move |ev| {
                    param.update(|p| p.r#type = event_target_value(&ev));
                }
                />
                <datalist id="types">
                    {move || match types.get() {
                        None => View::new(()).into_any(),
                        Some(types) => leptos::view! {
                            <For each=move || types.data.clone() key=|ty| ty.value.clone() let:ty>
                                <option value=ty.clone().value>
                                    <span class=ty.icon().unwrap_or_default()></span>
                                    { ty.value.clone() }
                                </option>
                            </For>
                        }.into_any()
                    }}
                </datalist>
            </div>

            <div class="input-group mb-3">
                <input
                    type="text"
                    value=move || param.get().what
                    placeholder="Nom ?"
                    class="form-control"
                    on:input=move |ev| {
                        param.update(|p| p.what = event_target_value(&ev));
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
                <Show when=move || param.get().r#type == "restaurant">
                    <Checkbox
                        value=move || param.get().vegetarian
                        on_toggle=move |value| param.update(|p| p.vegetarian = value)
                    >Végétarien</Checkbox>
                    <Checkbox
                        value=move || param.get().vegan
                        on_toggle=move |value| param.update(|p| p.vegan = value)
                    >Vegan</Checkbox>
                </Show>
            </div>

            <button class="btn btn-primary" on:click=move |ev| {
                ev.prevent_default();
                on_search(param.get());
            }>Rechercher</button>
        </form>
    }
}

#[leptos::component]
fn Checkbox<I, O>(
    children: leptos::children::ChildrenFn,
    value: I,
    on_toggle: O,
) -> impl leptos::IntoView
where
    I: Fn() -> bool + 'static,
    O: Fn(bool) + 'static,
{
    leptos::view! {
        <div class="form-check form-check-inline form-switch">
            <input
                type="checkbox"
                class="form-check-input"
                value=value()
                on:input=move |ev| on_toggle(event_target_checked(&ev))
            />
            <label class="form-check-label">{ children() }</label>
        </div>
    }
}
