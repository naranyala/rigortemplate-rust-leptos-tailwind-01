use leptos::prelude::*;
use crate::shared::utils::syntax_highlight::highlight_rust;

pub const SOURCE: &str = include_str!("code_block.rs");

#[component]
pub fn CodeBlock(source: &'static str) -> AnyView {
    let html = highlight_rust(source);
    let is_empty = source.trim().is_empty();

    let container_class = if is_empty {
        "relative group rounded-xl overflow-hidden border border-red-500 bg-slate-900 shadow-2xl ring-2 ring-red-500/20"
    } else {
        "relative group rounded-xl overflow-hidden border border-slate-800 bg-slate-900 shadow-2xl"
    };

    view! {
        <div class=container_class>
            <div class="flex items-center justify-between px-4 py-2 border-b border-slate-800 bg-slate-800/40">
                <div class="flex items-center gap-1.5">
                    <div class="w-3 h-3 rounded-full bg-red-500/80" />
                    <div class="w-3 h-3 rounded-full bg-yellow-500/80" />
                    <div class="w-3 h-3 rounded-full bg-green-500/80" />
                </div>
                <span class="text-[11px] text-slate-500 font-mono font-medium">"source.rs"</span>
            </div>
            <div class="overflow-x-auto">
                <pre class="p-6 text-[13px] leading-[1.7] font-mono text-slate-200 whitespace-pre">
                    {if is_empty {
                        view! { <span class="text-red-400 italic">"⚠️ Source code is missing or empty!"</span> }.into_any()
                    } else {
                        view! { <code inner_html=html class="block"></code> }.into_any()
                    }}
                </pre>
            </div>
        </div>
    }.into_any()
}
