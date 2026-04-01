use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let css_href = format!("/{}/{}.css", options.site_pkg_dir, options.output_name);

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"YouTube.rs"</title>
                <meta
                    name="description"
                    content="YouTube clone built using Rust - Leptos for frontend, Axum for backend, SQLx for database interactions. Features include user authentication, video upload and streaming, comments, and likes."
                />

                // Favicon
                <link rel="icon" href="/favicon.ico" />

                // Theme bootstrap: apply persisted theme before first paint to avoid flicker.
                <script>
                    "(function(){try{var t=localStorage.getItem('yt-theme');if(t==='dark'){document.documentElement.classList.add('dark');}else{document.documentElement.classList.remove('dark');}}catch(_){}})();"
                </script>

                // CSS
                <link rel="preload" href=css_href.clone() r#as="style" fetchpriority="high" />
                <link rel="stylesheet" href=css_href />

                // OAuth providers
                <script src="https://accounts.google.com/gsi/client" async defer></script>

                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>

            <body>
                <App />
            </body>
        </html>
    }
}