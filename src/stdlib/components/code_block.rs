use leptos::prelude::*;
use crate::stdlib::utils::syntax_highlight::highlight_rust;

pub const SOURCE: &str = include_str!("code_block.rs");

#[component]
pub fn CodeBlock(source: &'static str) -> AnyView {
    let html = highlight_rust(source);

    view! {
        <div class="relative group rounded-xl overflow-hidden border border-slate-700/50 bg-[#0d1117] shadow-sm">
            <div class="flex items-center gap-1.5 px-4 py-2 border-b border-slate-700/50 bg-slate-800/50">
                <div class="w-2.5 h-2.5 rounded-full bg-red-500/80" />
                <div class="w-2.5 h-2.5 rounded-full bg-yellow-500/80" />
                <div class="w-2.5 h-2.5 rounded-full bg-green-500/80" />
                <span class="ml-2 text-[11px] text-slate-500 font-mono font-medium">"source.rs"</span>
            </div>
            <div class="overflow-x-auto">
                <pre class="p-4 text-[13px] leading-relaxed font-mono whitespace-pre-wrap break-all"><code inner_html=html></code></pre>
            </div>
        </div>
    }.into_any()
}
