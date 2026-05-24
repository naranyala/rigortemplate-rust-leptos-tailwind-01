use leptos::prelude::*;
use crate::core::store::{GlobalStore, Page};

#[component]
pub fn TopNav() -> AnyView {
    let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
    let current_page = store.current_page;
    let toggle = move |_| { let n = !store.sidebar_open.get(); store.sidebar_open.set(n); };

    let page_label = move || match current_page.get() {
        Page::Home => "Dashboard",
        Page::Demo => "Component Library",
        Page::Accordion => "Accordion Demo",
        Page::SlidingPanel => "Sliding Panel Demo",
        Page::Hooks => "Hooks Library",
        _ => "Project",
    };

    view! {
        <div class="flex items-center justify-between h-14 px-5 bg-slate-900 border-b border-slate-800 sticky top-0 z-20">
            <div class="flex items-center gap-3">
                <button on:click=toggle class="lg:hidden flex items-center justify-center w-8 h-8 rounded-lg text-slate-400 hover:text-slate-100 hover:bg-slate-800 cursor-pointer transition-colors -ml-1">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                </button>
                <div class="flex items-center gap-2 text-sm text-slate-500 font-medium">
                    <span>"Workspace"</span>
                    <span class="text-slate-700">"/"</span>
                    <span class="text-slate-300">{move || page_label()}</span>
                </div>
            </div>
        </div>
    }.into_any()
}
