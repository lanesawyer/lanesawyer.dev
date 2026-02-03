use leptos::prelude::*;

use crate::components::icons::{
    github::GitHub, linkedin::LinkedIn, mastodon::Mastodon, storygraph::Storygraph,
};

pub struct NavLink {
    pub label: &'static str,
    pub path: &'static str,
    pub icon: Option<Box<dyn Fn() -> AnyView>>,
}

pub const NAV_LINKS: [NavLink; 4] = [
    NavLink {
        label: "Home",
        path: "/",
        icon: None,
    },
    NavLink {
        label: "Projects",
        path: "/projects",
        icon: None,
    },
    NavLink {
        label: "About",
        path: "/about",
        icon: None,
    },
    NavLink {
        label: "Blog",
        path: "https://blog.lanesawyer.dev",
        icon: None,
    },
];

pub fn social_links() -> Vec<NavLink> {
    vec![
        NavLink {
            label: "LinkedIn",
            path: "https://www.linkedin.com/in/lanesawyer/",
            icon: Some(Box::new(|| view! { <LinkedIn /> }.into_any())),
        },
        NavLink {
            label: "Mastodon",
            path: "https://mastodon.social/@lanesawyer",
            icon: Some(Box::new(|| view! { <Mastodon /> }.into_any())),
        },
        NavLink {
            label: "Storygraph",
            path: "https://app.thestorygraph.com/profile/vari",
            icon: Some(Box::new(|| view! { <Storygraph /> }.into_any())),
        },
        NavLink {
            label: "GitHub",
            path: "https://github.com/lanesawyer",
            icon: Some(Box::new(|| view! { <GitHub /> }.into_any())),
        },
    ]
}
