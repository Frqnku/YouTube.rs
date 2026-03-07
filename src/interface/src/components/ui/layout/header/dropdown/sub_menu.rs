use leptos::prelude::*;

use crate::components::ui::layout::header::{buttons::menu_items::{BackMenuItem, LeafMenuItem}, dropdown::LocationOption};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveSubmenu {
    Appearance,
    Language,
    Locations,
}

#[component]
fn AppearanceSubmenu(on_close: Callback<()>) -> impl IntoView {
    view! {
        <LeafMenuItem label="Light mode" on_select=on_close />
        <LeafMenuItem label="Dark mode" on_select=on_close />
    }
}

#[component]
fn LanguageSubmenu(on_close: Callback<()>) -> impl IntoView {
    view! {
        <LeafMenuItem label="English" on_select=on_close />
        <LeafMenuItem label="French" on_select=on_close />
    }
}

#[component]
fn LocationsSubmenu(
    locations_resource: LocalResource<Result<Vec<LocationOption>, String>>,
    on_select_location: Callback<String>,
) -> impl IntoView {
    view! {
        {move || {
            match locations_resource.get() {
                None => view! {
                    <div class="flex items-center justify-center px-4 py-4">
                        <span class="h-5 w-5 animate-spin rounded-full border-2 border-border border-t-text"></span>
                    </div>
                }
                .into_any(),
                Some(Err(error_message)) => view! {
                    <p class="px-4 py-3 text-base text-text-secondary">{error_message}</p>
                }
                .into_any(),
                Some(Ok(locations)) if locations.is_empty() => view! {
                    <p class="px-4 py-3 text-base text-text-secondary">"No locations found"</p>
                }
                .into_any(),
                Some(Ok(locations)) => view! {
                    <div class="max-h-72 overflow-y-auto py-1">
                        {locations
                            .into_iter()
                            .map(|location| {
                                let on_select = on_select_location;
                                view! {
                                    <button
                                        type="button"
                                        class="block w-full px-4 py-2 text-left text-base text-text transition hover:bg-bg-tertiary"
                                        on:click=move |_| on_select.run(location.iso2.clone())
                                    >
                                        {location.country}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                }
                .into_any(),
            }
        }}
    }
}

#[component]
pub fn SubmenuContainer(
    active_more_submenu: RwSignal<Option<ActiveSubmenu>>,
    locations_resource: LocalResource<Result<Vec<LocationOption>, String>>,
    on_select_location: Callback<String>,
    on_close: Callback<()>,
) -> impl IntoView {
    let go_back = Callback::new(move |_| active_more_submenu.set(None));

    view! {
        <BackMenuItem on_select=go_back />
        {move || {
            match active_more_submenu.get() {
                Some(ActiveSubmenu::Appearance) => {
                    view! { <AppearanceSubmenu on_close=on_close /> }.into_any()
                }
                Some(ActiveSubmenu::Language) => {
                    view! { <LanguageSubmenu on_close=on_close /> }.into_any()
                }
                Some(ActiveSubmenu::Locations) => {
                    view! {
                        <LocationsSubmenu
                            locations_resource=locations_resource
                            on_select_location=on_select_location
                        />
                    }
                        .into_any()
                }
                None => view! { <></> }.into_any(),
            }
        }}
    }
}