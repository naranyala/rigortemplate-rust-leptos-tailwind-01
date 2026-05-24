use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos_template::app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos_template::error::log_info("Leptos application starting...");
    mount_to_body(|| view! { <App /> })
}
