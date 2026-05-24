use leptos::prelude::*;

#[component]
pub fn Modal(
    #[prop(into)] _is_open: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6">
            // Backdrop
            <div 
                class="absolute inset-0 bg-slate-900/60 backdrop-blur-sm transition-opacity"
                on:click=move |_| on_close.run(())
            />
            
            // Modal Content
            <div class="relative w-full max-w-lg bg-white dark:bg-slate-800 rounded-2xl shadow-2xl transition-all animate-in zoom-in-95 duration-200 overflow-hidden">
                // Header
                <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
                    <h3 class="text-lg font-semibold text-slate-900 dark:text-white">{title}</h3>
                    <button 
                        class="p-2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 transition-colors rounded-full hover:bg-slate-100 dark:hover:bg-slate-700"
                        on:click=move |_| on_close.run(())
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
                
                // Body
                <div class="p-6">
                    {children()}
                </div>
            </div>
        </div>
    }
}
