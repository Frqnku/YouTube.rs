use leptos::prelude::*;

use crate::app::CurrentUserContext;
use crate::components::ui::buttons::LoginButton;

#[component]
pub fn Navbar() -> impl IntoView {
    let current_user_ctx = use_context::<CurrentUserContext>();
    let current_user = move || current_user_ctx.as_ref().and_then(|ctx| ctx.current_user.get());
    let current_user_name = move || current_user().map(|user| user.name).unwrap_or_default();
    let current_user_profile_picture = move || {
        current_user()
            .and_then(|user| user.profile_picture.clone())
            .unwrap_or_default()
    };
    let is_hydrated = RwSignal::new(false);

    Effect::new(move |_| {
        is_hydrated.set(true);
    });

    view! {
        <nav class="sticky top-0 z-40 border-b border-border bg-bg/95 backdrop-blur supports-[backdrop-filter]:bg-bg/80">
            <div class="mx-auto flex h-14 items-center justify-between px-4 md:px-6">
                <div class="flex items-center gap-3 md:gap-5">
                    <button class="icon-btn hidden md:inline-flex" title="Menu">
                        <svg class="h-6 w-6 text-text" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M3 6h18v2H3V6zm0 5h18v2H3v-2zm0 5h18v2H3v-2z" />
                        </svg>
                    </button>

                    <a href="/" class="flex items-center gap-2">
                        <span class="rounded-md bg-primary px-2 py-1 text-xs font-black tracking-wide text-white">"▶"</span>
                        <span class="text-lg font-semibold text-text">"YouTube"</span>
                    </a>
                </div>

                <div class="mx-3 hidden max-w-2xl flex-1 items-center md:flex">
                    <input
                        type="text"
                        class="input-yt w-full rounded-r-none"
                        placeholder="Search"
                    />
                    <button class="h-10 rounded-r-full border border-l-0 border-border bg-bg-secondary px-6 text-text-secondary hover:bg-bg-tertiary transition">
                        <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path d="M15.5 14h-.79l-.28-.27A6.5 6.5 0 1 0 14 15.5l.27.28v.79L20 21.49 21.49 20 15.5 14zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z" />
                        </svg>
                    </button>
                </div>

                <div class="flex items-center gap-2">
                    <button class="icon-btn md:hidden" title="Search">
                        <svg class="h-5 w-5 text-text" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M15.5 14h-.79l-.28-.27A6.5 6.5 0 1 0 14 15.5l.27.28v.79L20 21.49 21.49 20 15.5 14z" />
                        </svg>
                    </button>
                    <Show
                        when=move || is_hydrated.get() && current_user().is_some()
                        fallback=move || view! { <LoginButton /> }
                    >
                        <div class="flex items-center gap-2">
                            <div class="flex h-9 w-9 items-center justify-center rounded-full bg-bg-tertiary text-sm font-semibold text-text">
                                <img src={current_user_profile_picture} alt={format!("{}'s profile picture", current_user_name())} class="h-full w-full rounded-full object-cover" />
                            </div>
                        </div>
                    </Show>
                </div>
            </div>
        </nav>
    }
}
