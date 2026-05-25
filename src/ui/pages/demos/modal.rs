use leptos::prelude::*;
use crate::ui::components::modal::Modal;

#[component]
pub fn ModalDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    
    let open_modal = move |_| is_open.set(true);
    let close_modal = move |_| is_open.set(false);

    view! {
        <div class="flex flex-col items-center justify-center gap-6 p-12">
            <button 
                class="px-6 py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors shadow-lg shadow-blue-500/30"
                on:click=open_modal
            >
                "Open Accessibility Modal"
            </button>

            <Show when=move || is_open.get()>
                    <Modal 
                        _is_open=is_open
                        on_close=Callback::new(close_modal)
                        title="Confirmation Dialog"
                    >

                    <div class="space-y-4 text-center">
                        <div class="mx-auto w-12 h-12 bg-blue-100 dark:bg-blue-900/30 text-blue-600 rounded-full flex items-center justify-center mb-4">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        </div>
                        <p class="text-slate-600 dark:text-slate-400">
                            "Are you sure you want to proceed with this action? This cannot be undone."
                        </p>
                        <div class="flex justify-center gap-3 mt-6">
                            <button 
                                class="px-4 py-2 text-sm font-medium text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
                                on:click=move |_| is_open.set(false)
                            >
                                "Cancel"
                            </button>
                            <button 
                                class="px-4 py-2 text-sm font-medium bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                                on:click=move |_| is_open.set(false)
                            >
                                "Confirm Action"
                            </button>
                        </div>
                    </div>
                </Modal>
            </Show>
        </div>
    }
}

pub const SOURCE: &str = r#"
#[component]
pub fn ModalDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    
    view! {
        <button on:click=move |_| is_open.set(true)>"Open Modal"</button>
        <Show when=move || is_open.get()>
            <Modal 
                is_open=is_open 
                on_close=Callback::new(move |_| is_open.set(false))
                title="My Modal"
            >
                <div>"Modal content goes here"</div>
            </Modal>
        </Show>
    }
}
"#;
