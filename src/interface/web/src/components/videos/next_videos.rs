use leptos::prelude::*;

#[component]
pub fn NextVideos() -> impl IntoView {
    view! {
        <aside class="space-y-3">
            <h2 class="text-sm font-semibold uppercase tracking-wide text-text-secondary">
                "Next videos"
            </h2>
            <div class="space-y-2">
                <a class="flex gap-3 rounded-xl bg-bg-secondary p-2 transition hover:bg-bg-tertiary" href="#">
                    <div class="aspect-video w-40 shrink-0 rounded-lg bg-bg-tertiary" />
                    <div class="min-w-0">
                        <p class="line-clamp-2 text-sm font-medium text-text">"Build a YouTube Clone with Leptos"</p>
                        <p class="mt-1 text-xs text-text-secondary">"Rust Coding"</p>
                        <p class="text-xs text-text-muted">"24K views - 6 hours ago"</p>
                    </div>
                </a>
                <a class="flex gap-3 rounded-xl bg-bg-secondary p-2 transition hover:bg-bg-tertiary" href="#">
                    <div class="aspect-video w-40 shrink-0 rounded-lg bg-bg-tertiary" />
                    <div class="min-w-0">
                        <p class="line-clamp-2 text-sm font-medium text-text">"Leptos Router Deep Dive"</p>
                        <p class="mt-1 text-xs text-text-secondary">"WebAssembly Lab"</p>
                        <p class="text-xs text-text-muted">"17K views - 1 day ago"</p>
                    </div>
                </a>
            </div>
        </aside>
    }
}