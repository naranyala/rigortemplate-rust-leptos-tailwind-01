use crate::core::store::Page;

#[derive(Clone, Copy)]
pub struct NavItem {
    pub page: Page,
    pub label: &'static str,
}

pub const COMPONENTS_ITEMS: &[NavItem] = &[
    NavItem { page: Page::Accordion, label: "Accordion" },
    NavItem { page: Page::SlidingPanel, label: "Sliding Panel" },
    NavItem { page: Page::Tabs, label: "Tabs" },
    NavItem { page: Page::Modal, label: "Modal" },
    NavItem { page: Page::Stats, label: "Stats" },
    NavItem { page: Page::Badge, label: "Badge" },
    NavItem { page: Page::Button, label: "Button" },
    NavItem { page: Page::Card, label: "Card" },
    NavItem { page: Page::Input, label: "Input" },
    NavItem { page: Page::NotificationToast, label: "Toast" },
];

pub const HOOKS_ITEMS: &[NavItem] = &[
    NavItem { page: Page::HookToggle, label: "Toggle" },
    NavItem { page: Page::HookCounter, label: "Counter" },
    NavItem { page: Page::HookWindowSize, label: "Window Size" },
    NavItem { page: Page::HookMousePosition, label: "Mouse Pos" },
    NavItem { page: Page::HookMediaQuery, label: "Media Query" },
    NavItem { page: Page::HookOnlineStatus, label: "Online Status" },
    NavItem { page: Page::HookClickOutside, label: "Click Outside" },
    NavItem { page: Page::HookKeyboardShortcut, label: "Keyboard" },
];