use leptos::prelude::*;

use crate::{
    components::{
        layout::header::buttons::SigninButton,
        ui::icons::{Icon, IconKind}
    },
    hooks::use_google_signin
};

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

#[component]
pub fn SigninPromptModal(
    open: RwSignal<bool>,
    title: String,
    message: String,
) -> impl IntoView {
    let on_signin = use_google_signin();
    view! {
        <Show when=move || open.get()>
            <div
                class="fixed inset-0 z-40 flex items-center justify-center"
                on:click=move |_| open.set(false)
            >
                <section
                    role="dialog"
                    aria-modal="true"
                    aria-labelledby="signin-prompt-title"
                    class="w-full max-w-sm rounded-xl bg-bg-secondary p-6 text-text shadow-2xl"
                    on:click=move |event| event.stop_propagation()
                >
                    <header class="text-center">
                        <h2 id="signin-prompt-title" class="text-2xl font-semibold text-text">{title.clone()}</h2>
                    </header>

                    <p class="mt-3 text-center text-lg text-text-secondary">{message.clone()}</p>

                    <footer class="mt-6 flex justify-center">
                        <button
                            type="button"
                            class="btn-primary text-base px-10 py-2"
                            on:click=move |event| {
                                event.prevent_default();
                                on_signin.run(());
                            }
                        >
                            "Sign in"
                        </button>
                    </footer>
                </section>
            </div>
        </Show>
    }
}