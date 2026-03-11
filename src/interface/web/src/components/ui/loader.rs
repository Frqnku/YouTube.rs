use leptos::prelude::*;

#[component]
pub fn Loader() -> impl IntoView {
    view! {
        <div class="mx-auto mb-6 flex h-16 w-16 items-center justify-center">
            <div class="h-8 w-8 animate-spin rounded-full border-4 border-border border-t-primary"></div>
        </div>
    }
}
