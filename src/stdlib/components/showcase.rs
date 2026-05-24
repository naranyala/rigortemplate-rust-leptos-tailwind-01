use leptos::prelude::*;
use super::code_block::CodeBlock;

pub const SOURCE: &str = include_str!("showcase.rs");

#[component]
pub fn Showcase(
    label: &'static str,
    source: &'static str,
    children: Children,
) -> AnyView {
    view! {
        <div class="space-y-3">
            <div class="flex items-center justify-between">
                <h3 class="text-sm font-semibold text-slate-300 uppercase tracking-wider">{label}</h3>
            </div>
            <div class="p-5 bg-slate-800/50 border border-slate-700/50 rounded-xl flex items-center gap-3 flex-wrap">
                {children()}
            </div>
            <CodeBlock source=source />
        </div>
    }.into_any()
}
