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

    Effect::new(move |_| {
        let is_dark = Theme::Dark == theme.get();
        if let Some(document) = window().document() {
            if let Some(html) = document.document_element() {
                let mut classes = html.get_attribute("class").unwrap_or_default();
                if is_dark {
                    if !classes.contains("dark") {
                        classes.push_str(" dark");
                        let _ = html.set_attribute("class", &classes);
                    }
                } else {
                    if classes.contains("dark") {
                        let new_classes = classes.replace(" dark", "").replace("dark ", "").replace("dark", "");
                        let _ = html.set_attribute("class", &new_classes);
                    }
                }
            }
        }
    });

    view! {
        <div class="flex h-screen transition-colors duration-300 bg-surface text-body">
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
