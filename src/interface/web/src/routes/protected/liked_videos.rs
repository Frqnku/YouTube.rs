use leptos::prelude::*;

#[component]
pub fn LikedVideosPage() -> impl IntoView {
    view! {
        <div class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-6 md:px-8">
            <div class="mb-6 flex items-center justify-between">
                <div>
                    <h1 class="text-2xl font-semibold text-text">"Liked Videos"</h1>
                    <p class="mt-1 text-sm text-text-secondary">"All the videos you liked"</p>
                </div>
                <button class="btn-primary">"Play all"</button>
            </div>

            <section class="grid grid-cols-1 gap-5 sm:grid-cols-2 xl:grid-cols-3">
                {(1..=9)
                    .map(|idx| {
                        view! {
                            <article class="yt-card p-3">
                                <div class="relative aspect-video overflow-hidden rounded-lg bg-bg-tertiary">
                                    <span class="absolute bottom-2 right-2 rounded bg-black/80 px-1.5 py-0.5 text-xs text-white">
                                        "08:4" {idx % 10}
                                    </span>
                                </div>
                                <h2 class="mt-3 line-clamp-2 text-sm font-medium text-text">
                                    "Roadmap to build a YouTube clone in Rust (part " {idx} ")"
                                </h2>
                                <p class="mt-1 text-sm text-text-secondary">"YouTube Clone Lab"</p>
                                <p class="text-sm text-text-muted">"Added " {idx} " day(s) ago"</p>
                            </article>
                        }
                    })
                    .collect_view()}
            </section>
        </div>
    }
}