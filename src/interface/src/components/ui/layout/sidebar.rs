use leptos::prelude::*;
use leptos_router::hooks::use_location;

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

    let location = use_location();

    Effect::new(move |_| {
        let path = location.pathname.get();
        selected_page.set(SidebarItem::from_path(&path));
    });

    view! {
        <aside class="hidden w-56 shrink-0 border-r border-border bg-bg md:block">
            <nav class="sticky top-14 px-3 py-4">
                <a href="/" class="sidebar-item"
                    class:sidebar-item-active=move || matches!(selected_page.get(), SidebarItem::Home)>
                    <svg class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                        <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" />
                    </svg>
                    <span>"Home"</span>
                </a>

                <a href="/history" class="sidebar-item mt-1"
                    class:sidebar-item-active=move || matches!(selected_page.get(), SidebarItem::History)>
                    <svg class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                        <path d="M13 3a9 9 0 1 0 8.95 10h-2.02A7 7 0 1 1 13 5v3l4-4-4-4v3zm-1 5v6l5 3 .9-1.45-4.4-2.55V8H12z" />
                    </svg>
                    <span>"History"</span>
                </a>

                <a href="/liked-videos" class="sidebar-item mt-1"
                    class:sidebar-item-active=move || matches!(selected_page.get(), SidebarItem::LikedVideos)>
                    <svg class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                        <path d="M9 21h9a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2h-5l.95-4.57.03-.32a1 1 0 0 0-.29-.7L13 4 7.59 9.41A2 2 0 0 0 7 10.83V19a2 2 0 0 0 2 2zM5 10H3v11h2V10z" />
                    </svg>
                    <span>"Liked videos"</span>
                </a>
            </nav>
        </aside>
    }
}
