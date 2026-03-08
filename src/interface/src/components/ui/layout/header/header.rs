use leptos::prelude::*;

use crate::components::ui::layout::header::{HeaderLeft, HeaderRight, HeaderSearchBar};

#[component]
pub fn Header() -> impl IntoView {
    let selected_country_code = RwSignal::new("US".to_string());

    view! {
        <nav class="sticky top-0 z-40 bg-bg/95 backdrop-blur supports-[backdrop-filter]:bg-bg/80">
            <div class="mx-auto flex h-14 items-center justify-between px-4 md:px-6">
                <HeaderLeft country_code=Signal::derive(move || selected_country_code.get()) />
                <HeaderSearchBar />
                <HeaderRight
                    selected_country_code=selected_country_code
                />
            </div>
        </nav>
    }
}
