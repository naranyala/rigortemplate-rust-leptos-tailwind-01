use leptos::prelude::*;
use crate::core::nav::{COMPONENTS_ITEMS, HOOKS_ITEMS};

#[component]
pub fn Home() -> AnyView {
    let component_count = COMPONENTS_ITEMS.len();
    let hook_count = HOOKS_ITEMS.len();
    let total_demos = Memo::new(move |_| component_count + hook_count);

    view! {
        <div class="p-6">
            <h1 class="text-3xl font-bold text-heading mb-6">
                "Welcome to RigorTemplate"
            </h1>
            <p class="text-sm text-label mb-8">
                "A Leptos CSR dashboard with Tailwind CSS theming."
            </p>
            
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="bg-surface dark:bg-surface/50 border border-border rounded-xl p-6">
                    <div class="flex items-center space-x-3 mb-4">
                        <div class="w-10 h-10 bg-accent/10 text-accent rounded-full flex items-center justify-center">
                            "🧩"
                        </div>
                        <div>
                            <h2 class="text-lg font-medium text-heading">"Components Demo"</h2>
                            <p class="text-sm text-label">"Interactive component showcases"</p>
                        </div>
                    </div>
                    <div class="text-4xl font-bold text-accent">
                        {component_count}
                    </div>
                    <p class="text-xs text-label mt-2">"Total component demonstrations"</p>
                </div>
                
                <div class="bg-surface dark:bg-surface/50 border border-border rounded-xl p-6">
                    <div class="flex items-center space-x-3 mb-4">
                        <div class="w-10 h-10 bg-accent/10 text-accent rounded-full flex items-center justify-center">
                            "⚡"
                        </div>
                        <div>
                            <h2 class="text-lg font-medium text-heading">"Hooks Demo"</h2>
                            <p class="text-sm text-label">"Reusable logic demonstrations"</p>
                        </div>
                    </div>
                    <div class="text-4xl font-bold text-accent">
                        {hook_count}
                    </div>
                    <p class="text-xs text-label mt-2">"Total hook demonstrations"</p>
                </div>

                <div class="bg-surface dark:bg-surface/50 border border-border rounded-xl p-6">
                    <div class="flex items-center space-x-3 mb-4">
                        <div class="w-10 h-10 bg-accent/10 text-accent rounded-full flex items-center justify-center">
                            "📊"
                        </div>
                        <div>
                            <h2 class="text-lg font-medium text-heading">"Total Demos"</h2>
                            <p class="text-sm text-label">"Combined component and hook demonstrations"</p>
                        </div>
                    </div>
                    <div class="text-4xl font-bold text-accent">
                        {total_demos}
                    </div>
                    <p class="text-xs text-label mt-2">"Overall demonstration count"</p>
                </div>
            </div>
        </div>
    }.into_any()
}
