use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::{storage::use_local_storage, use_document, use_media_query};

use crate::config::{NAV_LINKS, social_links};

#[component]
pub fn Header() -> impl IntoView {
    // Initialize color mode on mount
    let (color_mode, set_color_mode, _) =
        use_local_storage::<String, FromToStringCodec>("color-mode");

    let is_light_preferred: Signal<bool> = use_media_query("(prefers-color-scheme: light)");
    let document = use_document();

    Effect::new({
        // Need to clone before moving into the closure
        let document = document.clone();
        move |_| {
            let color_mode = color_mode.get();
            let document_element = document.document_element();

            if color_mode == "light" || (is_light_preferred.get() && color_mode.is_empty()) {
                if let Some(html) = document_element {
                    let _ = html.set_attribute("color-mode", "light");
                }
            }
        }
    });

    let toggle_color_mode = move |is_light_button: bool| {
        let document_element = document.document_element();
        let Some(html) = document_element else {
            return;
        };

        let mode = if is_light_button { "light" } else { "dark" };
        let _ = html.set_attribute("color-mode", mode);
        set_color_mode.update(|color_mode| *color_mode = mode.to_string());
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
                    {social_links()
                        .iter()
                        .map(|link| {
                            view! {
                                <li>
                                    <a href=link.path target="_blank" title=link.label aria-label=link.label>{link.icon.as_ref().map(|icon| icon())}</a>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </ul>
                <button
                    class="color-mode__btn light--hidden"
                    aria-label="Toggle light mode"
                    on:click={
                        let toggle_color_mode = toggle_color_mode.clone();
                        move |_| toggle_color_mode(true)
                    }
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
                    on:click={
                        let toggle_color_mode = toggle_color_mode.clone();
                        move |_| toggle_color_mode(false)
                    }
                >
                    <svg viewBox="0 0 24 24">
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                </button>
            </nav>
        </header>
    }
}
