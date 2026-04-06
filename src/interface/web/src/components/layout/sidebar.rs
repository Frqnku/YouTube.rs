use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::{
    api::{_dtos::subscription::ChannelDto, subscription::get_subscriptions},
    components::ui::{LineDivider, icons::{Icon, IconKind}},
    context::CurrentUserContext,
};

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
pub fn Sidebar(sidebar_open: RwSignal<bool>) -> impl IntoView {
    let selected_page = RwSignal::new(SidebarItem::Home);
    let is_hydrated = RwSignal::new(false);

    let location = use_location();

    let current_user_ctx = use_context::<CurrentUserContext>();
    let is_authenticated = Signal::derive(move || {
        current_user_ctx
            .as_ref()
            .and_then(|ctx| ctx.current_user.get())
            .is_some()
    });

    let subscriptions_resource = LocalResource::new(move || {
        let authed = is_authenticated.get();

        async move {
            if !authed {
                Ok(vec![])
            } else {
                get_subscriptions().await
            }
        }
    });

    Effect::new(move |_| {
        is_hydrated.set(true);
    });

    Effect::new(move |_| {
        let path = location.pathname.get();
        selected_page.set(SidebarItem::from_path(&path));
    });

    view! {
        <Show when=move || is_hydrated.get()>
            <Show when=move || sidebar_open.get()>
                <div class="fixed inset-0 top-14 z-20 bg-black/40 md:hidden" on:click=move |_| sidebar_open.set(false)></div>
            </Show>

            <aside class=move || {
                if sidebar_open.get() {
                    "fixed left-0 top-14 z-60 w-60 shrink-0 bg-bg shadow-xl transition-transform duration-200 ease-out translate-x-0 md:sticky md:top-14 md:block md:h-[calc(100dvh-3.5rem)] md:self-start md:shadow-none"
                } else {
                    "fixed left-0 top-14 z-60 w-60 shrink-0 bg-bg shadow-xl transition-transform duration-200 ease-out -translate-x-full md:hidden"
                }
            }>
                <nav class="flex h-[calc(100dvh-3.5rem)] flex-col px-3 py-4">
                    <div>
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

                        <Show when=move || is_authenticated.get()>
                            <LineDivider margin="my-3".to_string()/>
                            <p class="my-2 px-3 font-semibold text-text-primary">"Subscriptions"</p>
                            <Transition>
                                {move || {
                                    let channels = subscriptions_resource
                                        .get()
                                        .and_then(Result::ok)
                                        .unwrap_or_default();

                                    if channels.is_empty() {
                                        view! {
                                            <p class="px-3 text-sm text-text-secondary">"No subscriptions"</p>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <For
                                                each=move || channels.clone()
                                                key=|c| c.id.clone()
                                                children=|channel: ChannelDto| {
                                                    let href = format!("/channel?id={}", channel.id);
                                                    view! {
                                                        <a href=href class="sidebar-item mt-1 gap-6 text-text-primary">
                                                            <img
                                                                src=channel.profile_picture.unwrap_or_default()
                                                                alt=channel.name.clone()
                                                                class="h-6 w-6 rounded-full bg-bg-tertiary object-cover"
                                                            />
                                                            <span class="truncate">{channel.name}</span>
                                                        </a>
                                                    }
                                                }
                                            />
                                        }.into_any()
                                    }
                                }}
                            </Transition>
                        </Show>
                    </div>

                    <div class="mt-auto pt-3">
                        <LineDivider margin="mb-3".to_string()/>
                        <a
                            href="https://www.linkedin.com/in/ugo-tiberto-729a55279/"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="sidebar-item"
                        >
                            <Icon kind=IconKind::LinkedIn />
                            <span>"Let's connect!"</span>
                        </a>
                        <a
                            href="https://github.com/Frqnku/YouTube.rs"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="sidebar-item mt-1"
                        >
                            <Icon kind=IconKind::Github class="h-5 w-5" />
                            <span>"Source code"</span>
                        </a>
                    </div>
                </nav>
            </aside>
        </Show>
    }
}
