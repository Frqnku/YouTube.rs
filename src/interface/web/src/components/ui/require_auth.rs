use leptos::prelude::*;

use crate::components::{layout::header::buttons::SigninButton, ui::icons::{Icon, IconKind}};

#[component]
pub fn RequireAuth(
    icon_kind: IconKind,
    title: String,
    message: String,
) -> impl IntoView {
    view! {
        <div class="min-h-[calc(100dvh-3.5rem)] flex flex-col items-center justify-center gap-4 p-8">
            <Icon kind=icon_kind class="h-20 w-20" />
            <h2 class="text-2xl font-semibold text-text">{title}</h2>
            <p class="text-lg text-text-secondary">{message}</p>
            <SigninButton />
        </div>
    }
}