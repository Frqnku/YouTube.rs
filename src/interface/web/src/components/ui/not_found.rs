use leptos::prelude::*;

#[component]
pub fn NotFound(message: String) -> impl IntoView {
    view! {
        <div class="flex min-h-dvh flex-col items-center justify-center bg-bg px-4">
            <h1 class="text-4xl font-bold text-text">"404"</h1>
            <p class="mt-4 text-text-secondary">{message}</p>
            <a href="/" class="mt-6 text-primary hover:underline">
                "Go back home"
            </a>
        </div>
    }
}