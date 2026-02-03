use leptos::prelude::*;

use crate::config::{NAV_LINKS, social_links};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <nav>
                <h3>Navigation</h3>
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
                <h3>Socials</h3>
                {social_links()
                    .iter()
                    .map(|link| {
                        view! {
                            <li>
                                <a href=link.path target="_blank">{link.icon.as_ref().map(|icon| icon())}{link.label}</a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </nav>
        </footer>
    }
}
