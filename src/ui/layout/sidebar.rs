use leptos::prelude::*;
use crate::core::store::{GlobalStore, Page};
use crate::core::nav::{NavItem, COMPONENTS_ITEMS, HOOKS_ITEMS};
use crate::core::utils::cn;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

fn matches(query: &str, label: &str, matcher: &SkimMatcherV2) -> bool {
    if query.is_empty() {
        return true;
    }
    matcher.fuzzy_match(label, query).is_some()
}

fn nav_section(
    title: &'static str,
    items: &[NavItem],
    current_page: RwSignal<Page>,
    open_store: RwSignal<bool>,
    query: &str,
) -> AnyView {
    let matcher = SkimMatcherV2::default();
    let filtered: Vec<&NavItem> = items.iter()
        .filter(|item| matches(query, item.label, &matcher))
        .collect();

    if filtered.is_empty() {
        return view! { <div></div> }.into_any();
    }

    view! {
        <div>
            <p class="px-3 mb-2 text-[10px] uppercase font-bold tracking-widest text-label">{title}</p>
            <div class="space-y-1">
                {filtered.into_iter().map(|item| {
                    let current_page = current_page.clone();
                    let open_store = open_store.clone();
                    let page = item.page;
                    let label = item.label;
                    view! {
                        <a
                            class=move || cn(&[
                                Some("flex items-center px-3 py-2 rounded-lg cursor-pointer text-sm font-medium transition-colors"),
                                if current_page.get() == page { Some("bg-accent/15 text-accent-text") } else { Some("text-body hover:text-heading hover:bg-muted") },
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
    }.into_any()
}

fn nav(
    current_page: RwSignal<Page>,
    open_store: RwSignal<bool>,
    search_query: RwSignal<String>,
) -> AnyView {
    let nav = move || {
        let q = search_query.get();
        view! {
            <>
                {nav_section("Components", &COMPONENTS_ITEMS, current_page, open_store, &q)}
                {nav_section("Hooks", &HOOKS_ITEMS, current_page, open_store, &q)}
            </>
        }.into_any()
    };

    view! {
        <div class="layout-sidebar flex flex-col h-full w-64 bg-surface shadow-lg shadow-black/[0.04]">
            <div class="flex items-center gap-2.5 px-4 h-14 border-b border-border shrink-0 bg-surface">
                <div class="w-7 h-7 bg-accent rounded-lg flex items-center justify-center text-white font-black text-sm shadow-sm shadow-accent/30 shrink-0">"L"</div>
                <span class="text-sm font-semibold tracking-tight text-heading">"Leptos"</span>
            </div>

            <div class="px-2.5 pt-3 pb-1">
                <input
                    type="text"
                    placeholder="Search..."
                    prop:value=search_query
                    on:input=move |ev| search_query.set(event_target_value(&ev))
                    class="w-full px-2 py-1.5 text-xs rounded-lg border transition-colors outline-none focus:ring-2 focus:ring-accent-ring/40 bg-muted border-border text-body placeholder-placeholder"
                />
            </div>

            <nav class="flex-1 px-2.5 py-2 space-y-6 overflow-y-auto">
                {move || {
                    let q = search_query.get();
                    view! {
                        <>
                            {nav_section("Components", COMPONENTS_ITEMS, current_page, open_store, &q)}
                            {nav_section("Hooks", HOOKS_ITEMS, current_page, open_store, &q)}
                        </>
                    }.into_any()
                }}
            </nav>
        </div>
    }.into_any()
}

#[component]
pub fn Sidebar() -> AnyView {
    let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
    let current_page = store.current_page;
    let open_store = store.sidebar_open;
    let search_query = RwSignal::new(String::new());

    let close = move |_| open_store.set(false);
    let open = open_store;

    view! {
        <>
            <div class="hidden lg:block shrink-0">
                {nav(current_page, open_store, search_query)}
            </div>

            {move || if open.get() {
                view! {
                    <>
                        <div class="fixed inset-0 bg-black/50 z-30 lg:hidden" on:click=close></div>
                        <div class="fixed inset-y-0 left-0 z-40 lg:hidden">
                            <div class="relative h-full">
                                <div class="absolute top-3 right-3 z-50">
                                    <button on:click=close class="flex items-center justify-center w-8 h-8 rounded-lg cursor-pointer transition-colors text-label hover:text-heading hover:bg-muted">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5"><path d="M6.28 5.22a.75.75 0 0 0-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 1 0 1.06 1.06L10 11.06l3.72 3.72a.75.75 0 1 0 1.06-1.06L11.06 10l3.72-3.72a.75.75 0 0 0-1.06-1.06L10 8.94 6.28 5.22Z"/></svg>
                                    </button>
                                </div>
                                {nav(current_page, open_store, search_query)}
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
