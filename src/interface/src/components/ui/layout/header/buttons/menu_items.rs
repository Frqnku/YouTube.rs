use leptos::prelude::*;

use crate::{api::user::auth::Logout, components::ui::icons::{Icon, IconKind}};

#[component]
pub fn LogoutButton(on_close: Callback<()>) -> impl IntoView {
    let logout_action = ServerAction::<Logout>::new();

    Effect::new(move |_| {
        if let Some(Ok(())) = logout_action.value().get() {
            on_close.run(());
            if let Some(window) = web_sys::window() {
                if let Ok(current_href) = window.location().href() {
                    let _ = window.location().set_href(&current_href);
                }
            }
        }
    });

    let on_logout = Callback::new(move |_| {
        logout_action.dispatch(Logout {});
    });

    view! {
        <button
            type="button"
            class="flex w-full items-center justify-between px-4 py-2 text-left text-base text-text transition hover:bg-bg-tertiary"
            on:click=move |_| on_logout.run(())
        >
            <span class="flex items-center gap-2">
                <Icon kind=IconKind::Logout />
                <span>"Logout"</span>
            </span>
        </button>
    }
}

#[component]
pub fn LeafMenuItem(label: &'static str, on_select: Callback<()>) -> impl IntoView {
    view! {
        <button
            type="button"
            class="block w-full px-4 py-2 text-left text-base text-text transition hover:bg-bg-tertiary"
            on:click=move |_| on_select.run(())
        >
            {label}
        </button>
    }
}

#[component]
pub fn BranchMenuItem(
    label: &'static str,
    icon: IconKind,
    on_select: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="flex w-full items-center justify-between px-4 py-2 text-left text-base text-text transition hover:bg-bg-tertiary"
            on:click=move |_| on_select.run(())
        >
            <span class="flex items-center gap-2">
                <Icon kind=icon />
                <span>{label}</span>
            </span>
            <Icon kind=IconKind::ChevronRight />
        </button>
    }
}

#[component]
pub fn BackMenuItem(on_select: Callback<()>) -> impl IntoView {
    view! {
        <button
            type="button"
            class="flex w-full items-center gap-2 px-4 py-2 text-left text-base text-text transition hover:bg-bg-tertiary"
            on:click=move |_| on_select.run(())
        >
            <Icon kind=IconKind::ChevronLeft />
            <span>"Back"</span>
        </button>
    }
}

#[component]
pub fn MenuLink(label: &'static str, icon: IconKind, href: &'static str, new_tab: bool, on_select: Callback<()>) -> impl IntoView {
    view! {
        <a
            href=href
            target=move || if new_tab { Some("_blank") } else { None }
            rel=move || if new_tab { Some("noopener noreferrer") } else { None }
            class="flex w-full items-center gap-2 px-4 py-2 text-left text-base text-text transition hover:bg-bg-tertiary"
            on:click=move |_| on_select.run(())
        >
            <Icon kind=icon />
            <span>{label}</span>
        </a>
    }
}
