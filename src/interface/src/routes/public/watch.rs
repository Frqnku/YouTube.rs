use leptos::prelude::*;

#[component]
pub fn WatchPage() -> impl IntoView {
    view! {
        <main class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-6 md:px-6">
            <section class="mx-auto grid w-full max-w-7xl gap-6 xl:grid-cols-[minmax(0,1fr)_360px]">
                <div class="space-y-4">
                    <div class="overflow-hidden rounded-xl border border-border bg-bg-secondary">
                        <div class="aspect-video w-full bg-black">
                            <video class="h-full w-full" controls preload="metadata" playsinline>
                                <source src="/videos/rickroll.mp4" type="video/mp4" />
                            </video>
                        </div>
                    </div>

                    <div class="space-y-4">
                        <h1 class="text-xl font-semibold tracking-tight text-text md:text-2xl">
                            "Demo video title - style YouTube"
                        </h1>

                        <div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                            <div class="flex items-center gap-3">
                                <div class="h-10 w-10 rounded-full bg-bg-tertiary" />
                                <div>
                                    <p class="font-medium text-text">"@my-channel"</p>
                                    <p class="text-sm text-text-secondary">"12 k abonnés"</p>
                                </div>
                            </div>

                            <div class="flex flex-wrap gap-2">
                                <button class="btn-primary">"S'abonner"</button>
                                <button class="btn-secondary">"J'aime"</button>
                                <button class="btn-secondary">"Partager"</button>
                            </div>
                        </div>

                        <div class="rounded-xl border border-border bg-bg-secondary p-4 text-sm leading-relaxed text-text-secondary">
                            <p class="font-medium text-text">"34 102 vues - il y a 2 jours"</p>
                            <p class="mt-2">
                                "Ajoute ici la description de la video. Ce bloc reprend l'esprit de YouTube avec un fond legerement plus clair et des infos condensées."
                            </p>
                        </div>
                    </div>
                </div>

                <aside class="space-y-3">
                    <h2 class="text-sm font-semibold uppercase tracking-wide text-text-secondary">
                        "A suivre"
                    </h2>
                    <div class="space-y-2">
                        <a class="flex gap-3 rounded-xl border border-border bg-bg-secondary p-2 transition hover:bg-bg-tertiary" href="#">
                            <div class="aspect-video w-40 shrink-0 rounded-lg bg-bg-tertiary" />
                            <div class="min-w-0">
                                <p class="line-clamp-2 text-sm font-medium text-text">"Build a YouTube Clone with Leptos"</p>
                                <p class="mt-1 text-xs text-text-secondary">"Rust Coding"</p>
                                <p class="text-xs text-text-muted">"24K views - 6 hours ago"</p>
                            </div>
                        </a>
                        <a class="flex gap-3 rounded-xl border border-border bg-bg-secondary p-2 transition hover:bg-bg-tertiary" href="#">
                            <div class="aspect-video w-40 shrink-0 rounded-lg bg-bg-tertiary" />
                            <div class="min-w-0">
                                <p class="line-clamp-2 text-sm font-medium text-text">"Leptos Router Deep Dive"</p>
                                <p class="mt-1 text-xs text-text-secondary">"WebAssembly Lab"</p>
                                <p class="text-xs text-text-muted">"17K views - 1 day ago"</p>
                            </div>
                        </a>
                    </div>
                </aside>
            </section>
        </main>
    }
}