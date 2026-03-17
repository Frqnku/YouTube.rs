use leptos::prelude::*;

#[component]
pub fn LineDivider(
    #[prop(optional)]
    margin: String,
) -> impl IntoView {
    view! {
        <div class=move || format!("border-t border-border {}", margin)></div>
    }
}

#[component]
pub fn DotDivider() -> impl IntoView {
    view! {
        <span class="mx-1 text-xs text-text-muted">"•"</span>
    }
}