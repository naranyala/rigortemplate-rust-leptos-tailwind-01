use leptos::prelude::*;
use crate::core::store::{use_store, NotificationLevel};

pub const SOURCE: &str = include_str!("notification_toast.rs");

#[component]
pub fn NotificationToast() -> impl IntoView {
    let store = use_store();
    let notifications = store.notifications;

    view! {
        <div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 w-80">
            <For
                each=move || notifications.get()
                key=|n| n.id
                children=move |n| {
                    let id = n.id;
                    let store = store.clone();
                    let level_class = match n.level {
                        NotificationLevel::Info => "bg-blue-600 text-blue-100",
                        NotificationLevel::Success => "bg-emerald-600 text-emerald-100",
                        NotificationLevel::Warning => "bg-amber-600 text-amber-100",
                        NotificationLevel::Error => "bg-rose-600 text-rose-100",
                    };

                    view! {
                        <div class=format!("p-4 rounded-lg shadow-lg transition-all animate-in slide-in-from-right-full {} flex justify-between items-center", level_class)>
                            <span class="text-sm font-medium">{n.message}</span>
                            <button 
                                class="ml-4 opacity-70 hover:opacity-100 transition-opacity"
                                on:click=move |_| store.remove_notification(id)
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}
