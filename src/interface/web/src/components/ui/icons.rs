use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconKind {
    BellActive,
    BellInactive,
    Check,
    ChevronLeft,
    ChevronRight,
    DotMenu,
    Edit,
    Github,
    Globe,
    History,
    Home,
    HomeSelected,
    LinkedIn,
    Logout,
    Menu,
    Moon,
    Search,
    ThumbsUp,
    ThumbsUpSelected,
    ThumbsDown,
    ThumbsDownSelected,
    TrashBin,
}

#[component]
pub fn Icon(
    kind: IconKind,
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    match kind {
        IconKind::BellActive => view! { <BellActiveIcon class=class /> }.into_any(),
        IconKind::BellInactive => view! { <BellInactiveIcon class=class /> }.into_any(),
        IconKind::Check => view! { <CheckIcon class=class /> }.into_any(),
        IconKind::ChevronLeft => view! { <ChevronLeftIcon class=class /> }.into_any(),
        IconKind::ChevronRight => view! { <ChevronRightIcon class=class /> }.into_any(),
        IconKind::DotMenu => view! { <DotMenuIcon class=class /> }.into_any(),
        IconKind::Edit => view! { <EditIcon class=class /> }.into_any(),
        IconKind::Github => view! { <GithubIcon class=class /> }.into_any(),
        IconKind::Globe => view! { <GlobeIcon class=class /> }.into_any(),
        IconKind::History => view! { <HistoryIcon class=class /> }.into_any(),
        IconKind::Home => view! { <HomeIcon class=class /> }.into_any(),
        IconKind::HomeSelected => view! { <HomeSelectedIcon class=class /> }.into_any(),
        IconKind::LinkedIn => view! { <LinkedInIcon class=class /> }.into_any(),
        IconKind::Logout => view! { <LogoutIcon class=class /> }.into_any(),
        IconKind::Menu => view! { <MenuIcon class=class /> }.into_any(),
        IconKind::Moon => view! { <MoonIcon class=class /> }.into_any(),
        IconKind::Search => view! { <SearchIcon class=class /> }.into_any(),
        IconKind::ThumbsUp => view! { <ThumbsUpIcon class=class /> }.into_any(),
        IconKind::ThumbsUpSelected => view! { <ThumbsUpSelectedIcon class=class /> }.into_any(),
        IconKind::ThumbsDown => view! { <ThumbsDownIcon class=class /> }.into_any(),
        IconKind::ThumbsDownSelected => view! { <ThumbsDownSelectedIcon class=class /> }.into_any(),
        IconKind::TrashBin => view! { <TrashBinIcon class=class /> }.into_any(),
    }
}

#[component]
fn BellActiveIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 448 512" height="24" width="24" xmlns="http://www.w3.org/2000/svg"><path d="M224 0c-17.7 0-32 14.3-32 32l0 19.2C119 66 64 130.6 64 208l0 18.8c0 47-17.3 92.4-48.5 127.6l-7.4 8.3c-8.4 9.4-10.4 22.9-5.3 34.4S19.4 416 32 416l384 0c12.6 0 24-7.4 29.2-18.9s3.1-25-5.3-34.4l-7.4-8.3C401.3 319.2 384 273.9 384 226.8l0-18.8c0-77.4-55-142-128-156.8L256 32c0-17.7-14.3-32-32-32zm45.3 493.3c12-12 18.7-28.3 18.7-45.3l-64 0-64 0c0 17 6.7 33.3 18.7 45.3s28.3 18.7 45.3 18.7s33.3-6.7 45.3-18.7z"></path></svg>
    }
}

#[component]
fn BellInactiveIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 448 512" height="24" width="24" xmlns="http://www.w3.org/2000/svg"><path d="M224 0c-17.7 0-32 14.3-32 32l0 19.2C119 66 64 130.6 64 208l0 18.8c0 47-17.3 92.4-48.5 127.6l-7.4 8.3c-8.4 9.4-10.4 22.9-5.3 34.4S19.4 416 32 416l384 0c12.6 0 24-7.4 29.2-18.9s3.1-25-5.3-34.4l-7.4-8.3C401.3 319.2 384 273.9 384 226.8l0-18.8c0-77.4-55-142-128-156.8L256 32c0-17.7-14.3-32-32-32zm45.3 493.3c12-12 18.7-28.3 18.7-45.3l-64 0-64 0c0 17 6.7 33.3 18.7 45.3s28.3 18.7 45.3 18.7s33.3-6.7 45.3-18.7z"></path></svg>
    }
}

#[component]
fn CheckIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"></path></svg>
    }
}

#[component]
fn ChevronLeftIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"></path></svg>
    }
}

#[component]
fn ChevronRightIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"></path></svg>
    }
}

#[component]
fn DotMenuIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle r="1" cy="12" cx="12"></circle><circle cx="12" r="1" cy="5"></circle><circle r="1" cy="19" cx="12"></circle></svg>
    }
}

#[component]
fn EditIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg stroke="currentColor" class=class fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M17.263 2.177a1.75 1.75 0 0 1 2.474 0l2.586 2.586a1.75 1.75 0 0 1 0 2.474L19.53 10.03l-.012.013L8.69 20.378a1.753 1.753 0 0 1-.699.409l-5.523 1.68a.748.748 0 0 1-.747-.188.748.748 0 0 1-.188-.747l1.673-5.5a1.75 1.75 0 0 1 .466-.756L14.476 4.963ZM4.708 16.361a.26.26 0 0 0-.067.108l-1.264 4.154 4.177-1.271a.253.253 0 0 0 .1-.059l10.273-9.806-2.94-2.939-10.279 9.813ZM19 8.44l2.263-2.262a.25.25 0 0 0 0-.354l-2.586-2.586a.25.25 0 0 0-.354 0L16.061 5.5Z"></path></svg>
    }
}

#[component]
fn GithubIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg stroke="currentColor" class=class fill="currentColor" stroke-width="0" viewBox="0 0 496 512" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"></path></svg>
    }
}

#[component]
fn GlobeIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" r="10" cy="12"></circle><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"></path><path d="M2 12h20"></path></svg>
    }
}

#[component]
fn HistoryIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path><path d="M3 3v5h5"></path><path d="M12 7v5l4 2"></path></svg>
    }
}

#[component]
fn HomeIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="m11.485 2.143-8 4.8-2 1.2a1 1 0 001.03 1.714L3 9.567V20a2 2 0 002 2h5v-8h4v8h5a2 2 0 002-2V9.567l.485.29a1 1 0 001.03-1.714l-2-1.2-8-4.8a1 1 0 00-1.03 0Z"></path></svg>
    }
}

#[component]
fn HomeSelectedIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="m11.485 2.143-8 4.8-2 1.2a1 1 0 001.03 1.714L3 9.567V20a2 2 0 002 2h5v-8h4v8h5a2 2 0 002-2V9.567l.485.29a1 1 0 001.03-1.714l-2-1.2-8-4.8a1 1 0 00-1.03 0Z"></path></svg>
    }
}

#[component]
fn LinkedInIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg class=class stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 448 512" width="20" height="20" xmlns="http://www.w3.org/2000/svg"><path d="M100.28 448H7.4V148.9h92.88zM53.79 108.1C24.09 108.1 0 83.5 0 53.8a53.79 53.79 0 0 1 107.58 0c0 29.7-24.1 54.3-53.79 54.3zM447.9 448h-92.68V302.4c0-34.7-.7-79.2-48.29-79.2-48.29 0-55.69 37.7-55.69 76.7V448h-92.78V148.9h89.08v40.8h1.3c12.4-23.5 42.69-48.3 87.88-48.3 94 0 111.28 61.9 111.28 142.3V448z"></path></svg>
    }
}

#[component]
fn LogoutIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path><polyline points="16 17 21 12 16 7"></polyline><line y2="12" y1="12" x1="21" x2="9"></line></svg>
    }
}

#[component]
fn MenuIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><line x2="20" y2="12" x1="4" y1="12"></line><line y1="6" x1="4" x2="20" y2="6"></line><line y2="18" x2="20" y1="18" x1="4"></line></svg>
    }
}

#[component]
fn MoonIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path></svg>
    }
}

#[component]
fn SearchIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path></svg>
    }
}

#[component]
fn ShareIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg stroke="currentColor" class=class fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="24" width="24" xmlns="http://www.w3.org/2000/svg"><path d="M383.822 344.427c-16.045 0-31.024 5.326-41.721 15.979l-152.957-88.42c1.071-5.328 2.142-9.593 2.142-14.919 0-5.328-1.071-9.593-2.142-14.919l150.826-87.35c11.762 10.653 26.741 17.041 43.852 17.041 35.295 0 64.178-28.766 64.178-63.92C448 72.767 419.117 44 383.822 44c-35.297 0-64.179 28.767-64.179 63.92 0 5.327 1.065 9.593 2.142 14.919l-150.821 87.35c-11.767-10.654-26.741-17.041-43.856-17.041-35.296 0-63.108 28.766-63.108 63.92 0 35.153 28.877 63.92 64.178 63.92 17.115 0 32.089-6.389 43.856-17.042l151.891 88.421c-1.076 4.255-2.141 8.521-2.141 13.847 0 34.094 27.806 61.787 62.037 61.787 34.229 0 62.036-27.693 62.036-61.787.001-34.094-27.805-61.787-62.035-61.787z"></path></svg>
    }
}

#[component]
fn ThumbsUpIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M7 10v12"></path><path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z"></path></svg>
    }
}

#[component]
fn ThumbsUpSelectedIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M7 10v12"></path><path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z"></path></svg>
    }
}

#[component]
fn ThumbsDownIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M17 14V2"></path><path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z"></path></svg>
    }
}

#[component]
fn ThumbsDownSelectedIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class=class width="24" height="24" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M17 14V2"></path><path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z"></path></svg>
    }
}

#[component]
fn TrashBinIcon(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <svg stroke="currentColor" class=class fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" height="24" width="24" xmlns="http://www.w3.org/2000/svg"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
    }
}