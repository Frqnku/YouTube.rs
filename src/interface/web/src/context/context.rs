use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use serde::{Deserialize, Serialize};

use crate::context::theme::{ThemeContext, ThemeMode, initial_theme_mode};

// Context for the currently authenticated user
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: String,
    pub name: String,
    pub profile_picture: Option<String>,
}

#[derive(Clone, Copy)]
pub struct CurrentUserContext {
    pub current_user: RwSignal<Option<CurrentUser>>,
}

// Context for client request metadata like IP address
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientRequestMeta {
    pub ip_address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncomingCookieHeader {
    pub cookie_header: Option<String>,
}

#[derive(Clone, Copy)]
pub struct ClientRequestMetaContext {
    pub client_meta: RwSignal<Option<ClientRequestMeta>>,
}

// Context for user subscriptions, can be expanded with more fields as needed
#[derive(Clone, Copy)]
pub struct SubscriptionsContext {
    pub refetch_trigger: RwSignal<u32>,
}

// Main application context that can be provided at the root of the component tree
#[derive(Clone)]
pub struct AppContext {
    pub current_user: RwSignal<Option<CurrentUser>>,
    pub client_meta: RwSignal<Option<ClientRequestMeta>>,
    pub theme_mode: ThemeContext,
    pub subscriptions: SubscriptionsContext,
}

impl AppContext {
    pub fn new(
        current_user: RwSignal<Option<CurrentUser>>,
        client_meta: RwSignal<Option<ClientRequestMeta>>,
        theme_mode: ThemeContext,
        subscriptions: SubscriptionsContext,
    ) -> Self {
        Self {
            current_user,
            client_meta,
            theme_mode,
            subscriptions,
        }
    }
}

// Helper trait to provide all contexts at once for both web and server entry points
pub trait ProvideContextExt {
    fn provide_contexts(&self);
}

impl ProvideContextExt for AppContext {
    fn provide_contexts(&self) {
        provide_context(CurrentUserContext {
            current_user: self.current_user.clone(),
        });
        provide_context(ClientRequestMetaContext {
            client_meta: self.client_meta.clone(),
        });
        provide_context(self.theme_mode);
        provide_context(self.subscriptions);
    }
}

// Function to set up all contexts, can be called from both web and server entry points
pub fn setup_app_contexts() -> (
    RwSignal<Option<CurrentUser>>,
    RwSignal<Option<ClientRequestMeta>>,
    RwSignal<ThemeMode>,
) {
    provide_meta_context();

    let current_client_signal = RwSignal::new(use_context::<ClientRequestMeta>());
    let current_user_signal = RwSignal::new(use_context::<CurrentUser>());
    let theme_mode = RwSignal::new(initial_theme_mode());

    AppContext::new(
        current_user_signal,
        current_client_signal,
        ThemeContext {
            mode: theme_mode,
            set_mode: Callback::new(move |mode: ThemeMode| {
                theme_mode.set(mode);
            }),
        },
        SubscriptionsContext {
            refetch_trigger: RwSignal::new(0u32),
        },
    )
    .provide_contexts();

    (current_user_signal, current_client_signal, theme_mode)
}