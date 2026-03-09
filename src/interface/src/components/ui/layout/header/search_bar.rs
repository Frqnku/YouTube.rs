use leptos::prelude::*;

use crate::components::ui::icons::{Icon, IconKind};

#[component]
pub fn HeaderSearchBar() -> impl IntoView {
    view! {
        <div class="flex min-w-0 flex-[0_1_640px] flex-row justify-end items-center px-4">
            <form
                action="/results"
                method="get"
                role="search"
                class="relative hidden h-10 flex-1 items-center px-1 text-text md:ml-4 md:flex"
            >
                <div class="flex h-10 w-full items-center rounded-l-full border border-border border-r-0 bg-bg py-0 pl-4 pr-1 shadow-inner shadow-black/5 transition focus-within:border-text-tertiary md:ml-8">
                    <input
                        type="text"
                        name="search_query"
                        autocomplete="off"
                        autocorrect="off"
                        spellcheck="false"
                        role="combobox"
                        aria-autocomplete="list"
                        placeholder="Search"
                        class="h-full w-full border-0 bg-transparent px-0 py-px text-base font-normal leading-[1.375rem] text-inherit placeholder:text-text-tertiary outline-none"
                    />
                </div>

                <button
                    type="submit"
                    aria-label="Search"
                    title="Search"
                    class="flex h-10 w-18 items-center justify-center rounded-r-full border border-border bg-bg-tertiary px-2 text-inherit transition hover:bg-bg-tertiary"
                >
                    <Icon kind=IconKind::Search />
                </button>
            </form>

            <button
                type="button"
                aria-label="Search"
                title="Search"
                class="icon-btn md:hidden"
            >
                <Icon kind=IconKind::Search />
            </button>

        </div>
    }
}