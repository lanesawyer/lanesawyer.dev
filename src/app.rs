use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::header::Header;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />
            <main>
                <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
                    <Route path=path!("/") view=|| view! { <h1>"Home"</h1> } />
                    <Route path=path!("/projects") view=|| view! { <h1>"Projects"</h1> } />
                    <Route path=path!("/about") view=|| view! { <h1>"About"</h1> } />
                </Routes>
            </main>
        </Router>
    }
}
