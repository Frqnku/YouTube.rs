use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
    pub set_mode: Callback<ThemeMode>,
}

#[cfg(target_arch = "wasm32")]
const THEME_STORAGE_KEY: &str = "yt-theme";

pub fn initial_theme_mode() -> ThemeMode {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(mode) = read_persisted_theme_mode() {
            return mode;
        }
    }

    ThemeMode::Light
}

#[cfg(target_arch = "wasm32")]
fn read_persisted_theme_mode() -> Option<ThemeMode> {
    let storage = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())?;

    let value = storage.get_item(THEME_STORAGE_KEY).ok().flatten()?;
    match value.as_str() {
        "dark" => Some(ThemeMode::Dark),
        "light" => Some(ThemeMode::Light),
        _ => None,
    }
}

fn persist_theme_mode(mode: ThemeMode) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = web_sys::window().and_then(|window| window.local_storage().ok().flatten()) {
            let stored_value = if mode == ThemeMode::Dark { "dark" } else { "light" };
            let _ = storage.set_item(THEME_STORAGE_KEY, stored_value);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = mode;
}

fn apply_theme(mode: ThemeMode) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            if let Some(root) = document.document_element() {
                let class_name = if mode == ThemeMode::Dark { "dark" } else { "" };
                root.set_class_name(class_name);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = mode;
}

pub fn setup_theme_effect(theme_mode: RwSignal<ThemeMode>) {
    Effect::new(move |_| {
        let current_theme = theme_mode.get();
        apply_theme(current_theme);
        persist_theme_mode(current_theme);
    });
}