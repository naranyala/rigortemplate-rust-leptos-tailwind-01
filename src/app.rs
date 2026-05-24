use leptos::prelude::*;
use leptos_meta::*;
use crate::components::{sidebar::Sidebar, topnav::TopNav};
use crate::pages::{home::Home, demo::Demo, demo_detail::DemoDetail, hooks::Hooks};
use crate::demos::accordion::{AccordionDemo, SOURCE as ACCORDION_SRC};
use crate::demos::sliding_panel::{SlidingPanelDemo, SOURCE as PANEL_SRC};
use crate::store::{GlobalStore, Page};
use crate::error;

#[component]
pub fn App() -> AnyView {
    provide_meta_context();

    let store = GlobalStore::new();
    let current_page = store.current_page;
    provide_context(store);
    error::log_info("GlobalStore initialized and provided to context");

    view! {
        <div class="flex h-screen bg-slate-900 text-slate-100">
            <Sidebar />
            <div class="flex-1 flex flex-col min-w-0">
                <TopNav />
                <main class="flex-1 overflow-y-auto">
                    {move || match current_page.get() {
                        Page::Home => view! { <Home /> }.into_any(),
                        Page::Demo => view! { <Demo /> }.into_any(),
                        Page::Accordion => view! { 
                            <DemoDetail label="Accordion Demo" source=ACCORDION_SRC>
                                <AccordionDemo />
                            </DemoDetail>
                        }.into_any(),
                        Page::SlidingPanel => view! { 
                            <DemoDetail label="Sliding Panel Demo" source=PANEL_SRC>
                                <SlidingPanelDemo />
                            </DemoDetail>
                        }.into_any(),
                        Page::Hooks => view! { <Hooks /> }.into_any(),
                    }}
                </main>
            </div>
        </div>
    }.into_any()
}
