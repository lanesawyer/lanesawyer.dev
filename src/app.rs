use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::{
    components::header::Header,
    pages::{About, Home, NotFound, Projects},
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />
            <main>
                <Routes fallback=NotFound >
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/projects") view=Projects />
                    <Route path=path!("/about") view=About />
                </Routes>
            </main>
        </Router>
    }
}
