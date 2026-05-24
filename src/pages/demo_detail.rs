use leptos::prelude::*;
use crate::ui::components::code_block::CodeBlock;

#[component]
pub fn DemoDetail(
    label: &'static str,
    source: &'static str,
    children: Children,
) -> AnyView {
    view! {
        <div class="p-6 md:p-10 max-w-6xl mx-auto space-y-8">
            <header class="flex flex-col gap-2">
                <h1 class="text-3xl font-black tracking-tight text-slate-100">{label}</h1>
                <p class="text-slate-400">"Interactive component demonstration with live rendering and source code."</p>
            </header>

            <div class="grid grid-cols-1 gap-8">
                <section class="space-y-4">
                    <div class="flex items-center justify-between">
                        <h2 class="text-sm font-semibold text-slate-300 uppercase tracking-wider">"Live Rendering"</h2>
                    </div>
                    <div class="p-8 bg-slate-800/50 border border-slate-700/50 rounded-2xl flex items-center justify-center min-h-[300px]">
                        {children()}
                    </div>
                </section>

                <section class="space-y-4">
                    <div class="flex items-center justify-between">
                        <h2 class="text-sm font-semibold text-slate-300 uppercase tracking-wider">"Source Code"</h2>
                    </div>
                    <CodeBlock source=source />
                </section>
            </div>
        </div>
    }.into_any()
}
