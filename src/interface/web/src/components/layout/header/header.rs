use leptos::prelude::*;

use crate::components::layout::header::{HeaderLeft, HeaderRight, HeaderSearchBar};

#[component]
pub fn Header(sidebar_open: RwSignal<bool>) -> impl IntoView {
    let selected_country_code = RwSignal::new("US".to_string());
    let mobile_search_open = RwSignal::new(false);

    view! {
        <nav class="sticky top-0 z-40 bg-bg/95 backdrop-blur">
            <div class="mx-auto flex h-14 items-center justify-between px-4 md:px-6">
                <HeaderLeft
                    country_code=Signal::derive(move || selected_country_code.get())
                    sidebar_open=sidebar_open
                    mobile_search_open=mobile_search_open
                />
                <HeaderSearchBar mobile_search_open=mobile_search_open />
                <HeaderRight
                    selected_country_code=selected_country_code
                    mobile_search_open=mobile_search_open
                />
            </div>
        </nav>
    }
}
