use leptos::prelude::*;

pub const SOURCE: &str = include_str!("sliding_panel.rs");

#[component]
pub fn SlidingPanelDemo() -> AnyView {
    let open = RwSignal::new(false);

    let toggle = move |_| open.update(|v| *v = !*v);
    let close = move |_| open.set(false);

    view! {
        <div class="px-2.5 py-3 border-b dark:border-slate-800 border-slate-200">
            <p class="text-[10px] uppercase font-bold text-slate-500 tracking-wider px-3 mb-2">"Sliding Panel Demo"</p>
            <div class="px-3">
                <button
                    class="flex items-center gap-2 w-full px-3 py-2 rounded-lg text-sm font-medium bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 transition-colors cursor-pointer"
                    on:click=toggle
                >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4 shrink-0">
                        <path fill-rule="evenodd" d="M2 3.75A.75.75 0 0 1 2.75 3h14.5a.75.75 0 0 1 0 1.5H2.75A.75.75 0 0 1 2 3.75ZM2 10a.75.75 0 0 1 .75-.75h14.5a.75.75 0 0 1 0 1.5H2.75A.75.75 0 0 1 2 10Zm0 6.25a.75.75 0 0 1 .75-.75h14.5a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1-.75-.75Z" clip-rule="evenodd"/>
                    </svg>
                    <span>"Open Panel"</span>
                </button>
            </div>
        </div>

        {move || {
            if open.get() {
                view! {
                    <>
                        <div class="fixed inset-0 bg-black/50 z-40" on:click=close></div>
                        <div class="fixed bottom-0 left-0 right-0 z-50 animate-slide-up">
                            <div class="rounded-t-2xl shadow-2xl mx-2 mb-2 border-t dark:bg-slate-800 dark:border-slate-700 bg-white border-slate-200">
                                <div class="flex items-center justify-between px-5 h-12 border-b dark:border-slate-700 border-slate-200">
                                    <span class="text-sm font-semibold dark:text-slate-200 text-slate-800">"Sliding Panel"</span>
                                    <button
                                        class="flex items-center justify-center w-7 h-7 rounded-lg transition-colors cursor-pointer text-slate-400 hover:text-slate-900 hover:bg-slate-100 dark:text-slate-500 dark:hover:text-slate-100 dark:hover:bg-slate-700"
                                        on:click=close
                                    >
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
                                            <path d="M6.28 5.22a.75.75 0 0 0-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 1 0 1.06 1.06L10 11.06l3.72 3.72a.75.75 0 1 0 1.06-1.06L11.06 10l3.72-3.72a.75.75 0 0 0-1.06-1.06L10 8.94 6.28 5.22Z"/>
                                        </svg>
                                    </button>
                                </div>
                                <div class="p-5 space-y-3 text-sm dark:text-slate-400 text-slate-600">
                                    <p>"This panel slides up from the bottom of the screen."</p>
                                    <p>"It can contain any content — forms, details, actions."</p>
                                    <p>"Click the backdrop or close button to dismiss."</p>
                                </div>
                            </div>
                        </div>
                    </>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }.into_any()
}
