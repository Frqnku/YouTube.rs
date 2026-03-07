use leptos::prelude::*;

use crate::components::ui::icons::{Icon, IconKind};

#[component]
pub fn HeaderSearchBar() -> impl IntoView {
    view! {
        <div class="mx-3 max-w-2xl flex-1 items-center flex">
            <input type="text" class="input-yt w-full rounded-r-none" placeholder="Search" />
            <button class="h-10 rounded-r-full border border-l-0 border-border bg-bg-secondary px-6 text-text-secondary hover:bg-bg-tertiary transition">
                <Icon kind=IconKind::Search />
            </button>
        </div>
    }
}