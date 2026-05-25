use leptos::prelude::*;
use crate::shared::utils::syntax_highlight::highlight_rust;

pub const SOURCE: &str = include_str!("code_block.rs");

#[component]
pub fn CodeBlock(source: &'static str) -> AnyView {
    let html = highlight_rust(source);

    view! {
        <div class="relative group rounded-xl overflow-hidden border border-slate-800 bg-slate-900 shadow-2xl">
            <div class="flex items-center justify-between px-4 py-2 border-b border-slate-800 bg-slate-800/40">
                <div class="flex items-center gap-1.5">
                    <div class="w-3 h-3 rounded-full bg-red-500/80" />
                    <div class="w-3 h-3 rounded-full bg-yellow-500/80" />
                    <div class="w-3 h-3 rounded-full bg-green-500/80" />
                </div>
                <span class="text-[11px] text-slate-500 font-mono font-medium">"source.rs"</span>
            </div>
            <div class="overflow-x-auto">
                <pre class="p-5 text-[13px] leading-relaxed font-mono text-slate-300 whitespace-pre"><code inner_html=html></code></pre>
            </div>
        </div>
    }.into_any()
}
