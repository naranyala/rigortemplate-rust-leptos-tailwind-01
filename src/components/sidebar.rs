use leptos::prelude::*;
use crate::store::{GlobalStore, Page};

fn nav(
    current_page: RwSignal<Page>,
    open_store: RwSignal<bool>,
) -> AnyView {
    let components_items = vec![
        (Page::Home, "Dashboard", "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 0m-2-0v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"),
        (Page::Demo, "Library", "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"),
        (Page::Accordion, "Accordion", "M19 9l-7 7-7-7"),
        (Page::SlidingPanel, "Sliding Panel", "M5 15l7-7 7 7"),
    ];

    let hooks_items = vec![
        (Page::Hooks, "Hooks Library", "M13 10V3L4 14h7v7l//9-11h-7z"),
    ];

    view! {
        <div class="flex flex-col h-full w-56 bg-slate-900 border-r border-slate-800">
            <div class="flex items-center gap-2.5 px-4 h-14 border-b border-slate-800 shrink-0">
                <div class="w-7 h-7 bg-indigo-500 rounded-lg flex items-center justify-center text-white font-black text-sm shadow-sm shadow-indigo-500/30 shrink-0">"L"</div>
                <span class="text-sm font-semibold text-slate-200 tracking-tight">"Leptos"</span>
            </div>
            <nav class="flex-1 px-2.5 py-4 space-y-6 overflow-y-auto">
                <div>
                    <p class="px-3 mb-2 text-[10px] uppercase font-bold text-slate-500 tracking-widest">"Components"</p>
                    <div class="space-y-1">
                        {components_items.into_iter().map(|(page, label, icon)| {
                            let current_page = current_page.clone();
                            let open_store = open_store.clone();
                            view! {
                                <a
                                    class=move || format!("flex items-center gap-3 px-3 py-2 rounded-lg cursor-pointer text-sm font-medium transition-colors {}",
                                        if current_page.get() == page { "bg-indigo-500/15 text-indigo-400" } else { "text-slate-400 hover:text-slate-100 hover:bg-slate-800" })
                                    on:click=move |_| {
                                        current_page.set(page);
                                        open_store.set(false);
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5 shrink-0">
                                        <path stroke-linecap="round" stroke-linejoin="round" d=icon />
                                    </svg>
                                    <span>{label}</span>
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
                <div>
                    <p class="px-3 mb-2 text-[10px] uppercase font-bold text-slate-500 tracking-widest">"Hooks"</p>
                    <div class="space-y-1">
                        {hooks_items.into_iter().map(|(page, label, icon)| {
                            let current_page = current_page.clone();
                            let open_store = open_store.clone();
                            view! {
                                <a
                                    class=move || format!("flex items-center gap-3 px-3 py-2 rounded-lg cursor-pointer text-sm font-medium transition-colors {}",
                                        if current_page.get() == page { "bg-indigo-500/15 text-indigo-400" } else { "text-slate-400 hover:text-slate-100 hover:bg-slate-800" })
                                    on:click=move |_| {
                                        current_page.set(page);
                                        open_store.set(false);
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5 shrink-0">
                                        <path stroke-linecap="round" stroke-linejoin="round" d=icon />
                                    </svg>
                                    <span>{label}</span>
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </nav>
        </div>
    }.into_any()
}

#[component]
pub fn Sidebar() -> AnyView {
    let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
    let current_page = store.current_page;
    let open_store = store.sidebar_open;

    let close = move |_| open_store.set(false);
    let open = open_store;

    view! {
        <>
            <div class="hidden lg:block shrink-0">
                {nav(current_page, open_store)}
            </div>

            {move || if open.get() {
                view! {
                    <>
                        <div class="fixed inset-0 bg-black/50 z-30 lg:hidden" on:click=close></div>
                        <div class="fixed inset-y-0 left-0 z-40 lg:hidden">
                            <div class="relative h-full">
                                <div class="absolute top-3 right-3 z-50">
                                    <button on:click=close class="flex items-center justify-center w-8 h-8 rounded-lg text-slate-400 hover:text-slate-100 hover:bg-slate-800 cursor-pointer transition-colors">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5"><path d="M6.28 5.22a.75.75 0 0 0-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 1 0 1.06 1.06L10 11.06l3.72 3.72a.75.75 0 1 0 1.06-1.06L11.06 10l3.72-3.72a.75.75 0 0 0-1.06-1.06L10 8.94 6.28 5.22Z"/></svg>
                                    </button>
                                </div>
                                {nav(current_page, open_store)}
                            </div>
                        </div>
                    </>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </>
    }.into_any()
}
