use leptos::prelude::*;
use crate::ui::components::tabs::{Tabs, TabItem};

#[component]
pub fn TabsDemo() -> impl IntoView {
    let items = vec![
        std::sync::Arc::new(TabItem {
            id: "account".to_string(),
            label: "Account".to_string(),
            content: Box::new(|| view! { 
                <div class="space-y-4">
                    <h4 class="text-lg font-semibold">Account Settings</h4>
                    <p class="text-label">Manage your account preferences and profile information.</p>
                    <div class="grid grid-cols-1 gap-4">
                        <div class="p-4 border rounded-lg border-border">"Username: user_leptos"</div>
                        <div class="p-4 border rounded-lg border-border">"Email: user@example.com"</div>
                    </div>
                </div>
            }.into_any()),
        }),
        std::sync::Arc::new(TabItem {
            id: "security".to_string(),
            label: "Security".to_string(),
            content: Box::new(|| view! { 
                <div class="space-y-4">
                    <h4 class="text-lg font-semibold">Security & Privacy</h4>
                    <p class="text-label">Update your password and enable two-factor authentication.</p>
                    <button class="px-4 py-2 bg-accent text-white rounded-lg hover:bg-accent-hover transition-colors">
                        "Change Password"
                    </button>
                </div>
            }.into_any()),
        }),
        std::sync::Arc::new(TabItem {
            id: "notifications".to_string(),
            label: "Notifications".to_string(),
            content: Box::new(|| view! { 
                <div class="space-y-4">
                    <h4 class="text-lg font-semibold">Notification Preferences</h4>
                    <p class="text-label">Control how you receive notifications from the system.</p>
                    <div class="flex items-center gap-2">
                        <input type="checkbox" checked />
                        <span>"Email Notifications"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <input type="checkbox" checked />
                        <span>"Push Notifications"</span>
                    </div>
                </div>
            }.into_any()),
        }),
    ];

    view! {
        <div class="max-w-2xl mx-auto bg-raised rounded-xl shadow-sm border border-border overflow-hidden">
            <Tabs items=items default_tab="account".to_string() />
        </div>
    }
}

pub const SOURCE: &str = include_str!("tabs.rs");
