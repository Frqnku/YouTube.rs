use leptos::prelude::*;

use crate::app::{ThemeContext, ThemeMode};
use crate::components::ui::Loader;
use crate::components::ui::icons::{Icon, IconKind};
use crate::components::layout::header::{buttons::menu_items::BackMenuItem, dropdown::LocationOption};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveSubmenu {
    Appearance,
    Locations,
}

#[component]
fn AppearanceSubmenu(on_close: Callback<()>) -> impl IntoView {
    let theme_context = use_context::<ThemeContext>();

    let on_select_light_mode = {
        let on_close = on_close;
        Callback::new(move |_| {
            if let Some(theme) = theme_context {
                theme.set_mode.run(ThemeMode::Light);
            }
            on_close.run(());
        })
    };

    let on_select_dark_mode = {
        let on_close = on_close;
        Callback::new(move |_| {
            if let Some(theme) = theme_context {
                theme.set_mode.run(ThemeMode::Dark);
            }
            on_close.run(());
        })
    };

    let is_light_selected = move || {
        theme_context
            .map(|ctx| ctx.mode.get() == ThemeMode::Light)
            .unwrap_or(true)
    };
    let is_dark_selected = move || {
        theme_context
            .map(|ctx| ctx.mode.get() == ThemeMode::Dark)
            .unwrap_or(false)
    };

    view! {
        <button
            type="button"
            class="flex w-full items-center gap-2 px-4 py-2 text-left text-base text-text transition hover:bg-bg-tertiary"
            on:click=move |_| on_select_light_mode.run(())
        >
            <div class="mr-2 flex h-5 w-5 items-center justify-center">
                <Show when=is_light_selected >
                    <Icon kind=IconKind::Check />
                </Show>
            </div>
            <span>"Light mode"</span>
        </button>
        <button
            type="button"
            class="flex w-full items-center gap-2 px-4 py-2 text-left text-base text-text transition hover:bg-bg-tertiary"
            on:click=move |_| on_select_dark_mode.run(())
        >
            <div class="mr-2 flex h-5 w-5 items-center justify-center">
                <Show when=is_dark_selected>
                    <Icon kind=IconKind::Check />
                </Show>
            </div>
            <span>"Dark mode"</span>
        </button>
    }
}

#[component]
fn LocationsSubmenu(
    locations_resource: LocalResource<Result<Vec<LocationOption>, String>>,
    selected_country_code: RwSignal<String>,
    on_select_location: Callback<String>,
) -> impl IntoView {
    view! {
        {move || {
            match locations_resource.get() {
                None => view! {
                    <Loader />
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
                        <For
                            each=move || locations.clone()
                            key=|location| location.iso2.clone()
                            children=move |location| {
                                let on_select = on_select_location;
                                let selected_country_code = selected_country_code;
                                let location_iso2_for_click = location.iso2.clone();
                                let location_iso2_for_check = location.iso2;
                                view! {
                                    <button
                                        type="button"
                                        class="flex w-full items-center gap-2 px-4 py-2 text-left text-base text-text transition hover:bg-bg-tertiary"
                                        on:click=move |_| on_select.run(location_iso2_for_click.clone())
                                    >
                                        <div class="mr-2 flex h-5 w-5 items-center justify-center">
                                            <Show when=move || {
                                                selected_country_code
                                                    .get()
                                                    .eq_ignore_ascii_case(&location_iso2_for_check)
                                            }>
                                                <Icon kind=IconKind::Check />
                                            </Show>
                                        </div>
                                        <span>{location.country}</span>
                                    </button>
                                }
                            }
                        />
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
    selected_country_code: RwSignal<String>,
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
                Some(ActiveSubmenu::Locations) => {
                    view! {
                        <LocationsSubmenu
                            locations_resource=locations_resource
                            selected_country_code=selected_country_code
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