use leptos::prelude::*;

#[component]
pub fn ErrorDisplay(errors: StoredValue<Vec<String>>) -> AnyView {
    view! {
        <div class="space-y-2">
            <div class="flex items-center gap-2 text-xs uppercase tracking-wider font-bold text-label">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 1 1-16 0 8 8 0 0 1 16 0Zm-7-4a1 1 0 1 1-2 0 1 1 0 0 1 2 0ZM9 9a.75.75 0 0 0 0 1.5h.253a.25.25 0 0 1 .244.304l-.459 2.066A1.75 1.75 0 0 0 10.747 15H11a.75.75 0 0 0 0-1.5h-.253a.25.25 0 0 1-.244-.304l.459-2.066A1.75 1.75 0 0 0 9.253 9H9Z" clip-rule="evenodd"/>
                </svg>
                <span>"Errors"</span>
            </div>
            {move || errors.get_value().iter().map(|e| view! {
                <div class="flex items-center gap-2 px-3 py-2 rounded-lg text-xs dark:bg-red-900/30 dark:text-red-400 dark:border-red-800 bg-red-50 text-red-700 border border-red-200">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4 shrink-0">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16ZM8.28 7.22a.75.75 0 0 0-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 1 0 1.06 1.06L10 11.06l1.72 1.72a.75.75 0 1 0 1.06-1.06L11.06 10l1.72-1.72a.75.75 0 0 0-1.06-1.06L10 8.94 8.28 7.22Z" clip-rule="evenodd"/>
                    </svg>
                    <span>{e.clone()}</span>
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }.into_any()
}
