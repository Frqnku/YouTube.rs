use leptos::prelude::*;

use crate::hooks::use_google_signin;

#[component]
pub fn SigninButton() -> impl IntoView {
    let on_signin = use_google_signin();

    view! {
        <button
            type="button"
            class="btn-secondary whitespace-nowrap text-xs md:text-sm"
            on:click=move |_| on_signin.run(())
        >
            Sign in
        </button>
    }
}