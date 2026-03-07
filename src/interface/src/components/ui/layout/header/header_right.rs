use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_use::on_click_outside;

use crate::{
    app::CurrentUserContext,
    components::ui::{
        icons::{Icon, IconKind},
        layout::header::{
            buttons::SigninButton,
            dropdown::{ActiveSubmenu, HeaderDropdown},
        },
    },
};

#[component]
pub fn HeaderRight(
    selected_country_code: RwSignal<String>,
) -> impl IntoView {
    let current_user_ctx = use_context::<CurrentUserContext>();
    let current_user = move || current_user_ctx.as_ref().and_then(|ctx| ctx.current_user.get());

    let is_authenticated = Signal::derive(move || current_user().is_some());

    let is_hydrated = RwSignal::new(false);
    let is_more_menu_open = RwSignal::new(false);
    let active_more_submenu = RwSignal::new(None::<ActiveSubmenu>);

    let location = use_location();

    Effect::new(move |_| {
        is_hydrated.set(true);
    });

    Effect::new(move |_| {
        let _pathname = location.pathname.get();
        is_more_menu_open.set(false);
        active_more_submenu.set(None);
    });

    view! {
        <div class="flex items-center gap-2">
            <Show
                when=move || is_hydrated.get()
                fallback=move || {
                    view! {
                        <div class="h-9 w-9 animate-pulse rounded-full bg-bg-tertiary"></div>
                    }
                }
            >
                <Show
                    when=move || is_authenticated.get()
                    fallback=move || {
                        view! {
                            <GuestActions
                                is_more_menu_open=is_more_menu_open
                                active_more_submenu=active_more_submenu
                                selected_country_code=selected_country_code
                            />
                        }
                    }
                >
                    <ProfileMenu
                        is_more_menu_open=is_more_menu_open
                        active_more_submenu=active_more_submenu
                        selected_country_code=selected_country_code
                    />
                </Show>
            </Show>
        </div>
    }
}

#[component]
fn GuestActions(
    is_more_menu_open: RwSignal<bool>,
    active_more_submenu: RwSignal<Option<ActiveSubmenu>>,
    selected_country_code: RwSignal<String>,
) -> impl IntoView {
    let guest_menu_ref = NodeRef::<leptos::html::Div>::new();

    let close_menu = Callback::new(move |_| {
        is_more_menu_open.set(false);
        active_more_submenu.set(None);
    });
    let on_select_location = Callback::new(move |iso2: String| {
        selected_country_code.set(iso2.to_uppercase());
        close_menu.run(());
    });

    let _close_dropdown_on_outside_click = on_click_outside(guest_menu_ref, move |_| {
        close_menu.run(());
    });

    view! {
        <div class="flex items-center gap-2">
            <div node_ref=guest_menu_ref class="relative">
                <button
                    type="button"
                    class="icon-btn"
                    title="Open menu"
                    on:click=move |_| {
                        is_more_menu_open.update(|open| *open = !*open);
                        if !is_more_menu_open.get() {
                            active_more_submenu.set(None);
                        }
                    }
                >
                    <Icon kind=IconKind::DotMenu />
                </button>

                <Show when=move || is_more_menu_open.get()>
                    <HeaderDropdown
                        active_more_submenu=active_more_submenu
                        on_select_location=on_select_location
                        on_close=close_menu
                        is_authenticated=false
                        user_name=String::new()
                        user_profile_picture=String::new()
                    />
                </Show>
            </div>

            <SigninButton />
        </div>
    }
}

#[component]
fn ProfileMenu(
    is_more_menu_open: RwSignal<bool>,
    active_more_submenu: RwSignal<Option<ActiveSubmenu>>,
    selected_country_code: RwSignal<String>,
) -> impl IntoView {
    let current_user_ctx = use_context::<CurrentUserContext>();
    let current_user = move || current_user_ctx.as_ref().and_then(|ctx| ctx.current_user.get());

    let profile_picture = Signal::derive(move || {
        current_user()
            .and_then(|user| user.profile_picture.clone())
            .unwrap_or_default()
    });
    let profile_alt = Signal::derive(move || {
        let user_name = current_user().map(|user| user.name).unwrap_or_default();
        format!("{}'s profile picture", user_name)
    });
    let profile_name = Signal::derive(move || current_user().map(|user| user.name).unwrap_or_default());

    let profile_menu_ref = NodeRef::<leptos::html::Div>::new();

    let close_menu = Callback::new(move |_| {
        is_more_menu_open.set(false);
        active_more_submenu.set(None);
    });
    let on_select_location = Callback::new(move |iso2: String| {
        selected_country_code.set(iso2.to_uppercase());
        close_menu.run(());
    });

    let _close_dropdown_on_outside_click = on_click_outside(profile_menu_ref, move |_| {
        close_menu.run(());
    });

    view! {
        <div node_ref=profile_menu_ref class="relative flex items-center gap-2">
            <button
                type="button"
                class="flex h-9 w-9 items-center justify-center overflow-hidden rounded-full bg-bg-tertiary text-sm font-semibold text-text"
                title="Open profile menu"
                on:click=move |_| {
                    is_more_menu_open.update(|open| *open = !*open);
                    if !is_more_menu_open.get() {
                        active_more_submenu.set(None);
                    }
                }
            >
                <img
                    src=move || profile_picture.get()
                    alt=move || profile_alt.get()
                    referrerpolicy="no-referrer"
                    class="h-full w-full rounded-full object-cover"
                />
            </button>

            <Show when=move || is_more_menu_open.get()>
                <HeaderDropdown
                    active_more_submenu=active_more_submenu
                    on_select_location=on_select_location
                    on_close=close_menu
                    is_authenticated=true
                    user_name=profile_name.get()
                    user_profile_picture=profile_picture.get()
                />
            </Show>
        </div>
    }
}