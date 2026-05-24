use leptos::prelude::*;
use leptos_meta::*;
use crate::components::layout::{MainLayout, LayoutType};
use crate::pages::{home::Home, demo::Demo, demo_detail::DemoDetail, hooks::Hooks};
use crate::demos::accordion::{AccordionDemo, SOURCE as ACCORDION_SRC};
use crate::demos::sliding_panel::{SlidingPanelDemo, SOURCE as PANEL_SRC};
use crate::demos::stats::{StatsDemo, SOURCE as STATS_SRC};
use crate::demos::tabs::{TabsDemo, SOURCE as TABS_SRC};
use crate::demos::modal::{ModalDemo, SOURCE as MODAL_SRC};
use crate::store::{GlobalStore, Page};
use crate::stdlib::components::notification_toast::NotificationToast;
use crate::error;

#[component]
pub fn App() -> AnyView {
    provide_meta_context();

    let store = GlobalStore::new();
    let current_page = store.current_page;
    provide_context(store);
    error::log_info("GlobalStore initialized and provided to context");

    view! {
        <MainLayout layout_type=LayoutType::Default>
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
                Page::Tabs => view! { 
                    <DemoDetail label="Tabs System Demo" source=TABS_SRC>
                        <TabsDemo />
                    </DemoDetail>
                }.into_any(),
                Page::Stepper => view! { <div>"Stepper Demo Coming Soon"</div> }.into_any(),
                Page::Stats => view! { 
                    <DemoDetail label="KPI Stats Cards Demo" source=STATS_SRC>
                        <StatsDemo />
                    </DemoDetail>
                }.into_any(),
                Page::Autocomplete => view! { <div>"Autocomplete Demo Coming Soon"</div> }.into_any(),
                Page::Upload => view! { <div>"Upload Demo Coming Soon"</div> }.into_any(),
                Page::Modal => view! {
                    <DemoDetail label="Modal/Dialog Demo" source=MODAL_SRC>
                        <ModalDemo />
                    </DemoDetail>
                }.into_any(),
            }}
        </MainLayout>
        <NotificationToast />
    }.into_any()
}
