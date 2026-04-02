use leptos::prelude::*;

use crate::{
    components::{
        ui::{icons::IconKind, LineDivider},
        layout::header::{
            buttons::{menu_items::{BranchMenuItem, LogoutButton, MenuLink}},
            dropdown::ActiveSubmenu,
        },
    },
};

#[component]
fn AuthenticatedUserHeader(user_name: String, user_profile_picture: String) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3 px-4 py-3">
            <img
                src=user_profile_picture
                alt="User profile picture"
                referrerpolicy="no-referrer"
                class="h-9 w-9 rounded-full object-cover"
            />
            <p class="truncate text-sm font-medium text-text">{user_name}</p>
        </div>
    }
}

#[component]
pub fn RootDropdownMenu(
    active_more_submenu: RwSignal<Option<ActiveSubmenu>>,
    should_load_locations: RwSignal<bool>,
    on_close: Callback<()>,
    is_authenticated: bool,
    user_name: String,
    user_profile_picture: String,
) -> impl IntoView {
    let open_appearance = Callback::new(move |_| active_more_submenu.set(Some(ActiveSubmenu::Appearance)));
    let open_locations = Callback::new(move |_| {
        active_more_submenu.set(Some(ActiveSubmenu::Locations));
        should_load_locations.set(true);
    });

    view! {
        {if is_authenticated {
            view! {
                <AuthenticatedUserHeader
                    user_name=user_name
                    user_profile_picture=user_profile_picture
                />
                <LineDivider />
                <LogoutButton on_close=on_close />
                <LineDivider />
            }
                .into_any()
        } else {
            view! { <></> }.into_any()
        }}

        <BranchMenuItem label="Appearance" icon=IconKind::Moon on_select=open_appearance />
        <BranchMenuItem label="Location" icon=IconKind::Globe on_select=open_locations />

        <MenuLink label="Let's connect!" icon=IconKind::LinkedIn href="https://www.linkedin.com/in/ugo-tiberto-729a55279/" new_tab=true on_select=on_close />
    }
}