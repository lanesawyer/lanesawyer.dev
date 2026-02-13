use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::{
    components::{footer::Footer, header::Header},
    pages::{About, Home, NotFound, Projects},
};

#[component]
pub fn App() -> impl IntoView {
    let is_footer_visible = RwSignal::new(false);

    view! {
        <Router>
            <Header is_footer_visible=is_footer_visible />
            <main>
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/projects") view=Projects />
                    <Route path=path!("/about") view=About />
                </Routes>
            </main>
            <Footer is_visible=is_footer_visible />
        </Router>
    }
}
