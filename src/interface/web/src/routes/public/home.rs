use leptos::prelude::*;

#[derive(Clone, Copy)]
struct VideoPreview {
    title: &'static str,
    channel: &'static str,
    meta: &'static str,
    duration: &'static str,
}

const VIDEOS: [VideoPreview; 8] = [
    VideoPreview {
        title: "Rust Full Course for Beginners",
        channel: "Rust Academy",
        meta: "1.2M views • 2 days ago",
        duration: "42:10",
    },
    VideoPreview {
        title: "Build a YouTube Clone with Leptos",
        channel: "Rust Coding",
        meta: "24K views • 6 hours ago",
        duration: "18:34",
    },
    VideoPreview {
        title: "Axum API + SQLx in Practice",
        channel: "Backend Factory",
        meta: "89K views • 1 week ago",
        duration: "27:18",
    },
    VideoPreview {
        title: "Tailwind CSS v4 Quick Guide",
        channel: "Design Dev",
        meta: "61K views • 4 days ago",
        duration: "12:05",
    },
    VideoPreview {
        title: "DDD Architecture in Rust",
        channel: "Clean Code",
        meta: "9.3K views • 5 days ago",
        duration: "31:47",
    },
    VideoPreview {
        title: "Async Rust: Tokio Explained",
        channel: "Rust Performance",
        meta: "73K views • 3 weeks ago",
        duration: "22:11",
    },
    VideoPreview {
        title: "Leptos Router Deep Dive",
        channel: "WebAssembly Lab",
        meta: "17K views • 1 day ago",
        duration: "16:52",
    },
    VideoPreview {
        title: "Build & Deploy app Rust fullstack",
        channel: "Prod Engineer",
        meta: "45K views • 2 weeks ago",
        duration: "29:03",
    },
];

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-[calc(100dvh-3.5rem)] bg-bg px-4 py-5 md:px-6">
            <div class="mb-5 flex flex-wrap items-center gap-2">
                <button class="btn-secondary bg-text text-bg">"All"</button>
                <button class="btn-secondary">"Rust"</button>
                <button class="btn-secondary">"Leptos"</button>
                <button class="btn-secondary">"Programming"</button>
                <button class="btn-secondary">"Architecture"</button>
                <button class="btn-secondary">"New"</button>
            </div>

            <section class="grid grid-cols-1 gap-6 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4">
                {VIDEOS
                    .into_iter()
                    .map(|video| {
                        view! {
                            <article class="group">
                                <a href="#" class="block">
                                    <div class="relative aspect-video overflow-hidden rounded-xl bg-bg-secondary">
                                        <div class="h-full w-full bg-bg-tertiary transition group-hover:scale-[1.02]" />
                                        <span class="absolute bottom-2 right-2 rounded bg-black/80 px-1.5 py-0.5 text-xs font-medium text-white">
                                            {video.duration}
                                        </span>
                                    </div>

                                    <div class="mt-3 flex gap-3">
                                        <div class="mt-1 h-9 w-9 shrink-0 rounded-full bg-bg-tertiary" />
                                        <div class="min-w-0">
                                            <h3 class="line-clamp-2 text-sm font-medium text-text">{video.title}</h3>
                                            <p class="mt-1 text-sm text-text-secondary">{video.channel}</p>
                                            <p class="text-sm text-text-muted">{video.meta}</p>
                                        </div>
                                    </div>
                                </a>
                            </article>
                        }
                    })
                    .collect_view()}
            </section>
        </div>
    }
}
