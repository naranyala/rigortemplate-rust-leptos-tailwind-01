use leptos::prelude::*;
use leptos_meta::*;
use crate::ui::layout::{MainLayout, LayoutType};
use crate::ui::pages::{home::Home, demo::Demo, demo_detail::DemoDetail, hooks::{HookToggle, HookCounter, HookWindowSize, HookMousePosition, HookMediaQuery, HookOnlineStatus, HookClickOutside, HookKeyboardShortcut}, demos::{accordion::AccordionDemo, accordion::SOURCE as ACCORDION_SRC, sliding_panel::SlidingPanelDemo, sliding_panel::SOURCE as PANEL_SRC, stats::StatsDemo, stats::SOURCE as STATS_SRC, tabs::TabsDemo, tabs::SOURCE as TABS_SRC, modal::ModalDemo, modal::SOURCE as MODAL_SRC}};
use crate::core::store::{GlobalStore, Page};
use crate::ui::components::notification_toast::NotificationToast;
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
                Page::Hooks => view! { <div>"Hooks Library"</div> }.into_any(),
                Page::HookToggle => view! { <DemoDetail label="Toggle Hook" source="use_toggle"> <HookToggle /> </DemoDetail> }.into_any(),
                Page::HookCounter => view! { <DemoDetail label="Counter Hook" source="use_counter_with_step"> <HookCounter /> </DemoDetail> }.into_any(),
                Page::HookWindowSize => view! { <DemoDetail label="Window Size Hook" source="use_window_size"> <HookWindowSize /> </DemoDetail> }.into_any(),
                Page::HookMousePosition => view! { <DemoDetail label="Mouse Position Hook" source="use_mouse_position"> <HookMousePosition /> </DemoDetail> }.into_any(),
                Page::HookMediaQuery => view! { <DemoDetail label="Media Query Hook" source="use_media_query"> <HookMediaQuery /> </DemoDetail> }.into_any(),
                Page::HookOnlineStatus => view! { <DemoDetail label="Online Status Hook" source="use_online_status"> <HookOnlineStatus /> </DemoDetail> }.into_any(),
                Page::HookClickOutside => view! { <DemoDetail label="Click Outside Hook" source="use_click_outside"> <HookClickOutside /> </DemoDetail> }.into_any(),
                Page::HookKeyboardShortcut => view! { <DemoDetail label="Keyboard Shortcut Hook" source="use_keyboard_shortcut"> <HookKeyboardShortcut /> </DemoDetail> }.into_any(),
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
                Page::Badge => view! { <DemoDetail label="Badge Demo" source="component"> <div>"Badge Demo"</div> </DemoDetail> }.into_any(),
                Page::Button => view! { <DemoDetail label="Button Demo" source="component"> <div>"Button Demo"</div> </DemoDetail> }.into_any(),
                Page::Card => view! { <DemoDetail label="Card Demo" source="component"> <div>"Card Demo"</div> </DemoDetail> }.into_any(),
                Page::Input => view! { <DemoDetail label="Input Demo" source="component"> <div>"Input Demo"</div> </DemoDetail> }.into_any(),
                Page::NotificationToast => view! { <DemoDetail label="Toast Demo" source="component"> <div>"Toast Demo"</div> </DemoDetail> }.into_any(),
            }}
        </MainLayout>
        <NotificationToast />
    }.into_any()
}
