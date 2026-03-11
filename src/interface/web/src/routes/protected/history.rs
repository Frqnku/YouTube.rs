use leptos::prelude::*;

#[component]
pub fn HistoryPage() -> impl IntoView {
    view! {
        <div class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-6 md:px-8">
            <h1 class="text-2xl font-semibold text-text">"History"</h1>
            <p class="mt-1 text-sm text-text-secondary">"Recently watched videos"</p>

            <div class="mt-6 space-y-4">
                {(1..=6)
                    .map(|idx| {
                        view! {
                            <article class="flex flex-col gap-3 rounded-xl bg-bg-secondary p-3 sm:flex-row">
                                <div class="relative aspect-video w-full shrink-0 overflow-hidden rounded-lg bg-bg-tertiary sm:w-72">
                                    <span class="absolute bottom-2 right-2 rounded bg-black/80 px-1.5 py-0.5 text-xs text-white">
                                        "12:3" {idx}
                                    </span>
                                </div>
                                <div class="min-w-0">
                                    <h2 class="text-base font-medium text-text">"Rust Session #" {idx}</h2>
                                    <p class="mt-1 text-sm text-text-secondary">"Rust Dev Channel"</p>
                                    <p class="mt-2 line-clamp-2 text-sm text-text-muted">
                                        "Hands-on tutorial on structuring a YouTube-inspired fullstack app with Leptos, Axum, and SQLx."
                                    </p>
                                </div>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}