use leptos::prelude::*;

use crate::components::ui::{icons::IconKind, layout::header::{buttons::menu_items::{BranchMenuItem, MenuLink}, dropdown::ActiveSubmenu}};

#[component]
pub fn RootDropdownMenu(
    active_more_submenu: RwSignal<Option<ActiveSubmenu>>,
    should_load_locations: RwSignal<bool>,
    on_close: Callback<()>,
) -> impl IntoView {
    let open_appearance = Callback::new(move |_| active_more_submenu.set(Some(ActiveSubmenu::Appearance)));
    let open_language = Callback::new(move |_| active_more_submenu.set(Some(ActiveSubmenu::Language)));
    let open_locations = Callback::new(move |_| {
        active_more_submenu.set(Some(ActiveSubmenu::Locations));
        should_load_locations.set(true);
    });

    view! {
        <BranchMenuItem label="Appearance" icon=IconKind::Moon on_select=open_appearance />
        <BranchMenuItem label="Display language" icon=IconKind::Translate on_select=open_language />
        <BranchMenuItem label="Location" icon=IconKind::Globe on_select=open_locations />
        <div class="mx-2 border-t border-border"></div>
        <MenuLink label="Settings" icon=IconKind::Settings href="/settings" new_tab=false on_select=on_close />
        <div class="mx-2 border-t border-border"></div>
        <MenuLink label="Let's connect!" icon=IconKind::LinkedIn href="https://www.linkedin.com/in/ugo-tiberto-729a55279/" new_tab=true on_select=on_close />
    }
}