use leptos::mount::mount_to_body;

mod app;
mod components;
mod pages;

use app::App;

fn main() {
    // Allows for better debugging in the browser console
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
