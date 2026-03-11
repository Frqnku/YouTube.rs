use leptos::prelude::*;

#[component]
pub fn LineDivider() -> impl IntoView {
    view! {
        <div class="border-t border-border"></div>
    }
}

#[component]
pub fn DotDivider() -> impl IntoView {
    view! {
        <span class="mx-1 text-xs text-text-muted">"•"</span>
    }
}