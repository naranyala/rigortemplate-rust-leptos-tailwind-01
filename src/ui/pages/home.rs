use leptos::prelude::*;
use crate::core::store::GlobalStore;

#[component]
pub fn Home() -> AnyView {
    let _store = match use_context::<GlobalStore>() {
        Some(s) => s,
        None => {
            return view! {
                <div class="p-6 md:p-10 max-w-7xl mx-auto">
                    <div class="p-4 rounded-xl bg-red-50 border border-red-200 dark:bg-red-900/30 dark:border-red-800">
                        <p class="text-sm font-medium text-red-700 dark:text-red-400">"Error"</p>
                        <p class="text-xs text-red-600 dark:text-red-300 mt-1">"Application state missing."</p>
                    </div>
                </div>
            }.into_any();
        }
    };

    view! {
        <div class="flex items-center justify-center h-[calc(100vh-3.5rem)] p-6">
            <div class="text-center space-y-2">
                <h1 class="text-4xl font-black tracking-tight text-heading">
                    "Welcome to RigorTemplate"
                </h1>
                <p class="text-sm text-label">
                    "A Leptos CSR dashboard with Tailwind CSS theming."
                </p>
            </div>
        </div>
    }.into_any()
}
