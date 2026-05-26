use leptos::prelude::*;
use crate::core::store::{GlobalStore, Theme, Page};

#[component]
pub fn TopNav() -> AnyView {
    let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
    let theme = store.theme;
    let current_page = store.current_page;

    let toggle_sidebar = move |_: leptos::ev::MouseEvent| store.sidebar_open.update(|v| *v = !*v);
    let _go_home = move |_: leptos::ev::MouseEvent| current_page.set(Page::Home);
    let toggle_theme = move |_: leptos::ev::MouseEvent| {
        theme.update(|t| {
            *t = match t {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
        });
    };

    let page_label = move || match current_page.get() {
        Page::Home => "Dashboard",
        Page::Demo => "Component Library",
        Page::Accordion => "Accordion Demo",
        Page::SlidingPanel => "Sliding Panel Demo",
        Page::Hooks => "Hooks Library",
        _ => "Project",
    };

    view! {
        <div class="layout-topnav flex items-center justify-between h-14 px-5 sticky top-0 z-20 bg-surface shadow-sm shadow-black/[0.03]">
            <div class="flex items-center gap-3">
                <button on:click=toggle_sidebar class="lg:hidden flex items-center justify-center w-8 h-8 rounded-lg cursor-pointer transition-colors -ml-1 text-label hover:text-heading hover:bg-muted">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                </button>
                <div class="flex items-center gap-2 text-sm font-medium text-label rounded-lg px-3 py-1.5 bg-muted border border-border">
                    <span>"Workspace"</span>
                    <span class="text-label">"/"</span>
                    <span class="text-body">{move || page_label()}</span>
                </div>
            </div>
            <div class="flex items-center gap-3">
                <button 
                    on:click=toggle_theme 
                    class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors cursor-pointer border bg-muted border-border text-body hover:bg-border"
                >
                    {move || if theme.get() == Theme::Dark {
                        view! { <span>"Light Mode"</span> }.into_any()
                    } else {
                        view! { <span>"Dark Mode"</span> }.into_any()
                    }}
                </button>
            </div>
        </div>
    }.into_any()
}
