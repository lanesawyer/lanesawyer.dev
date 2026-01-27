use leptos::prelude::*;
use web_sys::window;

pub struct NavLink {
    pub label: &'static str,
    pub path: &'static str,
}

const NAV_LINKS: [NavLink; 4] = [
    NavLink {
        label: "Home",
        path: "/",
    },
    NavLink {
        label: "Projects",
        path: "/projects",
    },
    NavLink {
        label: "About",
        path: "/about",
    },
    NavLink {
        label: "Blog",
        path: "https://blog.lanesawyer.dev",
    },
];

const SOCIAL_LINKS: [NavLink; 3] = [
    NavLink {
        label: "LinkedIn",
        path: "https://www.linkedin.com/in/lanesawyer/",
    },
    NavLink {
        label: "Mastodon",
        path: "https://mastodon.social/@lanesawyer",
    },
    NavLink {
        label: "Storygraph",
        path: "https://app.thestorygraph.com/profile/vari",
    },
];

#[component]
pub fn Header() -> impl IntoView {
    // Initialize color mode on mount
    Effect::new(move |_| {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let color_mode = storage.get_item("color-mode").ok().flatten();

                // Check if user has set preference or if OS prefers light mode
                let prefers_light = window
                    .match_media("(prefers-color-scheme: light)")
                    .ok()
                    .flatten()
                    .map(|media| media.matches())
                    .unwrap_or(false);

                if color_mode.as_deref() == Some("light") || (prefers_light && color_mode.is_none())
                {
                    if let Some(document) = window.document() {
                        if let Some(html) = document.document_element() {
                            let _ = html.set_attribute("color-mode", "light");
                        }
                    }
                }
            }
        }
    });

    let toggle_color_mode = move |is_light_button: bool| {
        let Some(window) = window() else { return };
        let Some(document) = window.document() else {
            return;
        };
        let Some(html) = document.document_element() else {
            return;
        };
        let Ok(Some(storage)) = window.local_storage() else {
            return;
        };

        let mode = if is_light_button { "light" } else { "dark" };
        let _ = html.set_attribute("color-mode", mode);
        let _ = storage.set_item("color-mode", mode);
    };

    view! {
        <header>
            <nav>
                <span class="title">/Lane</span>
                <ul class="nav-item">
                    {NAV_LINKS
                        .iter()
                        .map(|link| {
                            view! {
                                <li>
                                    <a href=link.path>{link.label}</a>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </ul>
                <ul class="nav-item right">
                    {SOCIAL_LINKS
                        .iter()
                        .map(|link| {
                            view! {
                                <li>
                                    <a href=link.path>{link.label}</a>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </ul>
                <button
                    class="color-mode__btn light--hidden"
                    aria-label="Toggle light mode"
                    on:click=move |_| toggle_color_mode(true)
                >
                    <svg viewBox="0 0 24 24">
                        <circle cx="12" cy="12" r="5"></circle>
                        <line x1="12" y1="1" x2="12" y2="3"></line>
                        <line x1="12" y1="21" x2="12" y2="23"></line>
                        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                        <line x1="1" y1="12" x2="3" y2="12"></line>
                        <line x1="21" y1="12" x2="23" y2="12"></line>
                        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                        <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                    </svg>
                </button>
                <button
                    class="color-mode__btn dark--hidden"
                    aria-label="Toggle dark mode"
                    on:click=move |_| toggle_color_mode(false)
                >
                    <svg viewBox="0 0 24 24">
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                </button>
            </nav>
        </header>
    }
}
