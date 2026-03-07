use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use reqwest::Client;
#[cfg(target_arch = "wasm32")]
use serde::Deserialize;

use crate::components::ui::layout::header::dropdown::{RootDropdownMenu, SubmenuContainer, ActiveSubmenu};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocationOption {
    pub country: String,
    pub iso2: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, Deserialize)]
struct LocationApiCountry {
    country: String,
    iso2: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, Deserialize)]
struct LocationApiResponse {
    error: bool,
    data: Vec<LocationApiCountry>,
}

#[cfg(target_arch = "wasm32")]
async fn fetch_locations() -> Result<Vec<LocationOption>, String> {
    let response = Client::new()
        .get("https://countriesnow.space/api/v0.1/countries")
        .send()
        .await
        .map_err(|_| "Unable to fetch locations".to_string())?;

    if !response.status().is_success() {
        return Err("Unable to fetch locations".to_string());
    }

    let payload: LocationApiResponse = response
        .json()
        .await
        .map_err(|_| "Invalid locations response".to_string())?;

    if payload.error {
        return Err("Location API returned an error".to_string());
    }

    Ok(payload
        .data
        .into_iter()
        .map(|country| LocationOption {
            country: country.country,
            iso2: country.iso2,
        })
        .collect())
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_locations() -> Result<Vec<LocationOption>, String> {
    Err("Location list is only available in browser mode".to_string())
}

#[component]
pub fn HeaderDropdown(
    active_more_submenu: RwSignal<Option<ActiveSubmenu>>,
    on_select_location: Callback<String>,
    on_close: Callback<()>,
) -> impl IntoView {
    let should_load_locations = RwSignal::new(false);
    let locations_resource = LocalResource::new(move || {
        let should_load = should_load_locations.get();
        async move {
            if should_load {
                fetch_locations().await
            } else {
                Ok(Vec::new())
            }
        }
    });

    view! {
        <div class="absolute right-0 top-10 z-50 mt-2 w-64 overflow-hidden rounded-xl border border-border bg-bg-secondary shadow-lg">
            <Show
                when=move || active_more_submenu.get().is_none()
                fallback=move || {
                    view! {
                        <SubmenuContainer
                            active_more_submenu=active_more_submenu
                            locations_resource=locations_resource
                            on_select_location=on_select_location
                            on_close=on_close
                        />
                    }
                }
            >
                <RootDropdownMenu
                    active_more_submenu=active_more_submenu
                    should_load_locations=should_load_locations
                    on_close=on_close
                />
            </Show>
        </div>
    }
}