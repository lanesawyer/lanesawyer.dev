use crate::{
    components::css_var_changer::CssVarChanger,
    config::{NAV_LINKS, social_links},
};
use leptos::{html, prelude::*};
use leptos_use::{UseIntersectionObserverOptions, use_intersection_observer_with_options};

#[component]
pub fn Footer(is_visible: RwSignal<bool>) -> impl IntoView {
    let footer_ref = NodeRef::<html::Footer>::new();

    let _ = use_intersection_observer_with_options(
        footer_ref,
        move |entries, _observer| {
            if let Some(entry) = entries.first() {
                is_visible.set(entry.is_intersecting());
            }
        },
        UseIntersectionObserverOptions::default().thresholds(vec![0.0]),
    );

    view! {
        <footer node_ref=footer_ref>
            <div>
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
            </div>
        </footer>
    }
}
