use leptos::prelude::*;
use leptos_meta::*;
use crate::ui::layout::{MainLayout, LayoutType};
use crate::ui::pages::{
    home::Home, 
    demo::Demo, 
    demo_detail::DemoDetail, 
    hooks::{
        HookToggle, HookCounter, HookWindowSize, HookMousePosition, 
        HookMediaQuery, HookOnlineStatus, HookClickOutside, HookKeyboardShortcut
    }, 
    demos::{
        accordion::{AccordionDemo, SOURCE as ACCORDION_SRC}, 
        sliding_panel::{SlidingPanelDemo, SOURCE as PANEL_SRC}, 
        stats::{StatsDemo, SOURCE as STATS_SRC}, 
        tabs::{TabsDemo, SOURCE as TABS_SRC}, 
        modal::{ModalDemo, SOURCE as MODAL_SRC}
    }
};
use crate::ui::components::{
    badge::{BaseBadge, SOURCE as BADGE_SRC},
    button::{Btn, ButtonVariant, SOURCE as BUTTON_SRC},
    card::{BaseCard, SOURCE as CARD_SRC},
    input::{TheInput, SOURCE as INPUT_SRC},
    notification_toast::{NotificationToast, SOURCE as TOAST_SRC},
};
use crate::shared::hooks::{
    state::SOURCE as STATE_SRC,
    window::SOURCE as WINDOW_SRC,
    mouse::SOURCE as MOUSE_SRC,
    media::SOURCE as MEDIA_SRC,
    browser::SOURCE as BROWSER_SRC,
    dom::SOURCE as DOM_SRC,
    keyboard::SOURCE as KEYBOARD_SRC,
};
use crate::core::store::{GlobalStore, Page};
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
                Page::HookToggle => view! { <DemoDetail label="Toggle Hook" source=STATE_SRC> <HookToggle /> </DemoDetail> }.into_any(),
                Page::HookCounter => view! { <DemoDetail label="Counter Hook" source=STATE_SRC> <HookCounter /> </DemoDetail> }.into_any(),
                Page::HookWindowSize => view! { <DemoDetail label="Window Size Hook" source=WINDOW_SRC> <HookWindowSize /> </DemoDetail> }.into_any(),
                Page::HookMousePosition => view! { <DemoDetail label="Mouse Position Hook" source=MOUSE_SRC> <HookMousePosition /> </DemoDetail> }.into_any(),
                Page::HookMediaQuery => view! { <DemoDetail label="Media Query Hook" source=MEDIA_SRC> <HookMediaQuery /> </DemoDetail> }.into_any(),
                Page::HookOnlineStatus => view! { <DemoDetail label="Online Status Hook" source=BROWSER_SRC> <HookOnlineStatus /> </DemoDetail> }.into_any(),
                Page::HookClickOutside => view! { <DemoDetail label="Click Outside Hook" source=DOM_SRC> <HookClickOutside /> </DemoDetail> }.into_any(),
                Page::HookKeyboardShortcut => view! { <DemoDetail label="Keyboard Shortcut Hook" source=KEYBOARD_SRC> <HookKeyboardShortcut /> </DemoDetail> }.into_any(),
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
                Page::Badge => view! { <DemoDetail label="Badge Demo" source=BADGE_SRC> <BaseBadge text="Badge Example" color="bg-accent/20 text-accent-text" /> </DemoDetail> }.into_any(),
                Page::Button => view! { <DemoDetail label="Button Demo" source=BUTTON_SRC> <Btn variant=ButtonVariant::Primary>"Button Example"</Btn> </DemoDetail> }.into_any(),
                Page::Card => view! { <DemoDetail label="Card Demo" source=CARD_SRC> <BaseCard title="Card Example"><p class="text-sm text-body">"Example content."</p></BaseCard> </DemoDetail> }.into_any(),
                Page::Input => view! { <DemoDetail label="Input Demo" source=INPUT_SRC> <TheInput label="Example Label" placeholder="Enter text..." value=RwSignal::new(String::new()) /> </DemoDetail> }.into_any(),
                Page::NotificationToast => view! { <DemoDetail label="Toast Demo" source=TOAST_SRC> <NotificationToast /> </DemoDetail> }.into_any(),
            }}
        </MainLayout>
        <NotificationToast />
    }.into_any()
}
