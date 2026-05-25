use leptos::prelude::*;
use crate::core::store::{GlobalStore, Page};
use crate::core::utils::cn;

fn nav(
    current_page: RwSignal<Page>,
    open_store: RwSignal<bool>,
) -> AnyView {
    let components_items = vec![
        (Page::Accordion, "Accordion"),
        (Page::SlidingPanel, "Sliding Panel"),
        (Page::Tabs, "Tabs"),
        (Page::Modal, "Modal"),
        (Page::Stats, "Stats"),
        (Page::Badge, "Badge"),
        (Page::Button, "Button"),
        (Page::Card, "Card"),
        (Page::Input, "Input"),
        (Page::NotificationToast, "Toast"),
    ];

    let hooks_items = vec![
        (Page::HookToggle, "Toggle"),
        (Page::HookCounter, "Counter"),
        (Page::HookWindowSize, "Window Size"),
        (Page::HookMousePosition, "Mouse Pos"),
        (Page::HookMediaQuery, "Media Query"),
        (Page::HookOnlineStatus, "Online Status"),
        (Page::HookClickOutside, "Click Outside"),
        (Page::HookKeyboardShortcut, "Keyboard"),
    ];

    view! {
        <div class="flex flex-col h-full w-56 dark:bg-slate-900 bg-slate-50 dark:border-r-slate-800 border-r-slate-200 border-r">
            <div class="flex items-center gap-2.5 px-4 h-14 dark:border-b-slate-800 border-b-slate-200 border-b shrink-0 dark:bg-slate-900 bg-white">
                <div class="w-7 h-7 bg-indigo-500 rounded-lg flex items-center justify-center text-white font-black text-sm shadow-sm shadow-indigo-500/30 shrink-0">"L"</div>
                <span class="text-sm font-semibold tracking-tight dark:text-slate-200 text-slate-900">"Leptos"</span>
            </div>
            <nav class="flex-1 px-2.5 py-4 space-y-6 overflow-y-auto">
                <div>
                    <p class="px-3 mb-2 text-[10px] uppercase font-bold tracking-widest dark:text-slate-500 text-slate-400">"Components"</p>
                    <div class="space-y-1">
                        {components_items.into_iter().map(|(page, label)| {
                            let current_page = current_page.clone();
                            let open_store = open_store.clone();
                            view! {
                                <a
                                    class=move || cn(&[
                                        Some("flex items-center px-3 py-2 rounded-lg cursor-pointer text-sm font-medium transition-colors"),
                                        if current_page.get() == page { Some("bg-indigo-500/15 text-indigo-400") } else { Some("text-slate-600 hover:text-slate-900 hover:bg-slate-200 dark:text-slate-400 dark:hover:text-slate-100 dark:hover:bg-slate-800") },
                                    ])
                                    on:click=move |_| {
                                        current_page.set(page);
                                        open_store.set(false);
                                    }
                                >
                                    <span>{label}</span>
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
                <div>
                    <p class="px-3 mb-2 text-[10px] uppercase font-bold tracking-widest dark:text-slate-500 text-slate-400">"Hooks"</p>
                    <div class="space-y-1">
                        {hooks_items.into_iter().map(|(page, label)| {
                            let current_page = current_page.clone();
                            let open_store = open_store.clone();
                            view! {
                                <a
                                    class=move || cn(&[
                                        Some("flex items-center px-3 py-2 rounded-lg cursor-pointer text-sm font-medium transition-colors"),
                                        if current_page.get() == page { Some("bg-indigo-500/15 text-indigo-400") } else { Some("text-slate-600 hover:text-slate-900 hover:bg-slate-200 dark:text-slate-400 dark:hover:text-slate-100 dark:hover:bg-slate-800") },
                                    ])
                                    on:click=move |_| {
                                        current_page.set(page);
                                        open_store.set(false);
                                    }
                                >
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
                                    <button on:click=close class="flex items-center justify-center w-8 h-8 rounded-lg cursor-pointer transition-colors text-slate-500 hover:text-slate-700 hover:bg-slate-100 dark:text-slate-400 dark:hover:text-slate-100 dark:hover:bg-slate-800">
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
