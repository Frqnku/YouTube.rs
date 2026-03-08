use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::ui::icons::{Icon, IconKind};

#[derive(Debug, Clone, PartialEq, Eq)]
enum SidebarItem {
    Home,
    History,
    LikedVideos,
}

impl SidebarItem {
    fn from_path(path: &str) -> Self {
        match path {
            "/history" => SidebarItem::History,
            "/liked-videos" => SidebarItem::LikedVideos,
            _ => SidebarItem::Home,
        }
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let selected_page = RwSignal::new(SidebarItem::Home);
    let is_hydrated = RwSignal::new(false);

    let location = use_location();

    Effect::new(move |_| {
        is_hydrated.set(true);
    });

    Effect::new(move |_| {
        let path = location.pathname.get();
        selected_page.set(SidebarItem::from_path(&path));
    });

    view! {
        <aside class="hidden w-60 shrink-0 bg-bg md:block">
            <Show when=move || is_hydrated.get()>
                <nav class="sticky top-14 px-3 py-4">
                    <a href="/" class="sidebar-item"
                        class:sidebar-item-active=move || matches!(selected_page.get(), SidebarItem::Home)>
                        <Show
                            when=move || matches!(selected_page.get(), SidebarItem::Home)
                            fallback=move || view! { <Icon kind=IconKind::Home /> }
                        >
                            <Icon kind=IconKind::HomeSelected />
                        </Show>
                        <span>"Home"</span>
                    </a>

                    <a href="/history" class="sidebar-item mt-1"
                        class:sidebar-item-active=move || matches!(selected_page.get(), SidebarItem::History)>
                        <Icon kind=IconKind::History />
                        <span>"History"</span>
                    </a>

                    <a href="/liked-videos" class="sidebar-item mt-1"
                        class:sidebar-item-active=move || matches!(selected_page.get(), SidebarItem::LikedVideos)>
                        <Show
                            when=move || matches!(selected_page.get(), SidebarItem::LikedVideos)
                            fallback=move || view! { <Icon kind=IconKind::ThumbsUp /> }
                        >
                            <Icon kind=IconKind::ThumbsUpSelected />
                        </Show>
                        <span>"Liked videos"</span>
                    </a>
                </nav>
            </Show>
        </aside>
    }
}
