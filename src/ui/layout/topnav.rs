use leptos::prelude::*;
use crate::core::store::{GlobalStore, Theme, Page};

#[component]
pub fn TopNav() -> AnyView {
    let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
    let theme = store.theme;
    let current_page = store.current_page;

    let toggle_sidebar = move |_| store.sidebar_open.update(|v| *v = !*v);
    let go_home = move |_| current_page.set(Page::Home);
    let toggle_theme = move |_| {
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
                <div on:click=go_home class="flex items-center gap-2 text-sm font-medium text-label cursor-pointer">
                    <span>"Workspace"</span>
                    <span class="text-label">"/"</span>
                    <span class="text-body">{move || page_label()}</span>
                </div>
            </div>
            <div class="flex items-center gap-3">
                <button 
                    on:click=toggle_theme 
                    class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors cursor-pointer border bg-muted border-border text-body hover:bg-border"
                >
                    {move || if theme.get() == Theme::Dark {
                        view! { 
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-6.364-0.386l1.591-1.591M3 12h2.25m.386 6.364l1.591-1.591M12 12a3 3 0 1 0 0-6 3 0 0 0 0 6Z" />
                            </svg>
                            <span>"Light Mode"</span>
                        }.into_any()
                    } else {
                        view! { 
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.759 9.759 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.759 9.759 0 0 0 9.002-5.998z" />
                            </svg>
                            <span>"Dark Mode"</span>
                        }.into_any()
                    }}
                </button>
            </div>
        </div>
    }.into_any()
}
