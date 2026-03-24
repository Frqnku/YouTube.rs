use leptos::prelude::*;
use crate::{
    api::user::auth::{get_current_client_meta, get_current_user},
    context::{ClientRequestMeta, CurrentUser}
};

pub fn setup_user_resources(
    current_user_signal: RwSignal<Option<CurrentUser>>,
    current_client_signal: RwSignal<Option<ClientRequestMeta>>,
) {
    let current_client_resource = Resource::new(
        move || (),
        move |_| async move { get_current_client_meta().await.ok().flatten() },
    );

    let current_user_resource = Resource::new(
        move || (),
        move |_| async move { get_current_user().await.ok().flatten() },
    );

    Effect::new(move |_| {
        if let Some(current_user) = current_user_resource.get() {
            current_user_signal.set(current_user);
        }
        if let Some(current_client) = current_client_resource.get() {
            current_client_signal.set(current_client);
        }
    });
}