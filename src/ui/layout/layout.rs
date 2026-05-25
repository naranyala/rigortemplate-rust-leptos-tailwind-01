use leptos::prelude::*;
use crate::ui::layout::{sidebar::Sidebar, topnav::TopNav};
use crate::core::store::{GlobalStore, Theme};

#[derive(Clone, Copy, PartialEq)]
pub enum LayoutType {
    Default,
    NoSidebar,
    NoTopNav,
    Fullscreen,
}

#[component]
pub fn MainLayout(
    #[prop(into)] layout_type: LayoutType,
    children: Children,
) -> impl IntoView {
    let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
    let theme = store.theme;

    view! {
        <div class="flex h-screen transition-colors duration-300 bg-surface text-body"
             class:dark=move || Theme::Dark == theme.get()>
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
