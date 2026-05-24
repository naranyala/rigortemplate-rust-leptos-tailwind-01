use leptos::prelude::*;
use crate::ui::layout::{sidebar::Sidebar, topnav::TopNav};
use crate::core::store::GlobalStore;

#[derive(Clone, Copy, PartialEq)]
pub enum LayoutType {
    Default,     // Sidebar + TopNav + Main
    NoSidebar,   // TopNav + Main
    NoTopNav,    // Sidebar + Main
    Fullscreen,  // Main only
}

#[component]
pub fn MainLayout(
    #[prop(into)] layout_type: LayoutType,
    children: Children,
) -> impl IntoView {
    let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
    let theme = store.theme;

    view! {
        <div class=move || format!("flex h-screen text-slate-100 transition-colors duration-300 {}", 
            if theme.get() == crate::core::store::Theme::Dark { "bg-slate-900" } else { "bg-slate-100 text-slate-900" }
        )>
            {move || match layout_type {
                LayoutType::Default | LayoutType::NoTopNav => view! { <Sidebar /> }.into_any(),
                _ => view! { <div /> }.into_any(),
            }}
            
            <div class="flex-1 flex flex-col min-w-0">
                {move || match layout_type {
                    LayoutType::Default | LayoutType::NoSidebar => view! { <TopNav /> }.into_any(),
                    _ => view! { <div /> }.into_any(),
                }}
                <main class="flex-1 overflow-y-auto">
                    {children()}
                </main>
            </div>
        </div>
    }
}
