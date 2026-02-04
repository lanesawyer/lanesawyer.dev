use leptos::prelude::*;

use crate::{
    components::css_var_changer::CssVarChanger,
    config::{NAV_LINKS, social_links},
};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <nav>
                <div>
                    <h3>Navigation</h3>
                    <ul>
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
                </div>
                <div>
                    <h3>Socials</h3>
                    <ul>
                        {social_links()
                            .iter()
                            .map(|link| {
                                view! {
                                    <li>
                                        <a href=link.path target="_blank">
                                            {link.icon.as_ref().map(|icon| icon())}
                                            {link.label}
                                        </a>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </div>
            </nav>
            <CssVarChanger />
        </footer>
    }
}
