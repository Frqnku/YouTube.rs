use leptos::prelude::*;

use crate::components::ui::icons::{Icon, IconKind};

#[component]
fn SearchForm(
    input_ref: NodeRef<leptos::html::Input>,
    #[prop(into)]
    form_class: String,
    #[prop(into)]
    input_wrapper_class: String,
    #[prop(into)]
    submit_button_class: String,
    #[prop(default = Callback::new(|_| ()))]
    close_button: Callback<()>,
    #[prop(default = false)]
    show_close_button: bool,
    #[prop(default = false)]
    autofocus: bool,
) -> impl IntoView {
    let close_button_view = if show_close_button {
        Some(
            view! {
                <button
                    type="button"
                    aria-label="Close search"
                    title="Close search"
                    class="icon-btn shrink-0"
                    on:click=move |_| close_button.run(())
                >
                    <Icon kind=IconKind::ChevronLeft />
                </button>
            }
            .into_any(),
        )
    } else {
        None
    };

    view! {
        <form
            action="/results"
            method="get"
            role="search"
            class=form_class
            on:submit=move |event: leptos::ev::SubmitEvent| {
                let value = input_ref
                    .get()
                    .map(|input| input.value())
                    .unwrap_or_default();

                if value.trim().is_empty() {
                    event.prevent_default();
                }
            }
        >
            {close_button_view}

            <div class=input_wrapper_class>
                <input
                    node_ref=input_ref
                    type="text"
                    name="search"
                    autocomplete="off"
                    spellcheck="false"
                    role="combobox"
                    aria-autocomplete="list"
                    placeholder="Search"
                    autofocus=autofocus
                    class="h-full w-full border-0 bg-transparent px-0 py-px text-base font-normal leading-[1.375rem] text-inherit placeholder:text-text-tertiary outline-none"
                />
            </div>

            <button
                type="submit"
                aria-label="Search"
                title="Search"
                class=submit_button_class
            >
                <Icon kind=IconKind::Search />
            </button>
        </form>
    }
}

#[component]
pub fn HeaderSearchBar(mobile_search_open: RwSignal<bool>) -> impl IntoView {
    let desktop_search_input_ref = NodeRef::<leptos::html::Input>::new();
    let mobile_search_input_ref = NodeRef::<leptos::html::Input>::new();

    let close_mobile_search = {
        let mobile_search_input_ref = mobile_search_input_ref.clone();
        Callback::new(move |_| {
            if let Some(input) = mobile_search_input_ref.get() {
                input.set_value("");
            }
            mobile_search_open.set(false);
        })
    };

    view! {
        <div class=move || {
            if mobile_search_open.get() {
                "flex min-w-0 flex-1 flex-row items-center gap-2"
            } else {
                "flex min-w-0 flex-[0_1_640px] flex-row items-center justify-end px-4"
            }
        }>
            <SearchForm
                input_ref=desktop_search_input_ref
                form_class="relative hidden h-10 flex-1 items-center px-1 text-text md:ml-4 md:flex"
                input_wrapper_class="flex h-10 w-full items-center rounded-l-full border border-border border-r-0 bg-bg py-0 pl-4 pr-1 shadow-inner shadow-black/5 transition focus-within:border-text-tertiary md:ml-8"
                submit_button_class="flex h-10 w-18 items-center justify-center rounded-r-full border border-border bg-bg-tertiary px-2 text-inherit transition hover:bg-bg-tertiary"
            />

            <Show
                when=move || mobile_search_open.get()
                fallback=move || {
                    view! {
                        <button
                            type="button"
                            aria-label="Search"
                            title="Search"
                            class="icon-btn md:hidden"
                            on:click=move |_| {
                                mobile_search_open.set(true);
                            }
                        >
                            <Icon kind=IconKind::Search />
                        </button>
                    }
                }
            >
                <SearchForm
                    input_ref=mobile_search_input_ref
                    form_class="flex h-10 w-full items-center gap-2 text-text md:hidden"
                    input_wrapper_class="flex h-10 w-full items-center rounded-full border border-border bg-bg py-0 pl-4 pr-1 shadow-inner shadow-black/5 transition focus-within:border-text-tertiary"
                    submit_button_class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-border bg-bg-tertiary text-inherit transition hover:bg-bg-tertiary"
                    close_button=close_mobile_search
                    show_close_button=true
                    autofocus=true
                />
            </Show>

        </div>
    }
}