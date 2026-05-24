use leptos::prelude::*;

pub const SOURCE: &str = include_str!("error_display.rs");

#[component]
pub fn ErrorMessage(
    #[prop(into)] title: String,
    #[prop(into)] message: String,
) -> AnyView {
    view! {
        <div class="flex flex-col items-center justify-center p-12 text-center space-y-4">
            <div class="w-16 h-16 bg-red-900/30 text-red-400 rounded-2xl flex items-center justify-center text-3xl border border-red-800/50">
                "⚠"
            </div>
            <h3 class="text-lg font-bold text-slate-200">{title}</h3>
            <p class="text-sm text-slate-400 max-w-md">{message}</p>
        </div>
    }.into_any()
}

#[component]
pub fn ErrorFallback(
    #[prop(into)] title: String,
    #[prop(into)] message: String,
    #[prop(optional)] children: Option<Children>,
) -> AnyView {
    view! {
        <div class="bg-slate-800 border border-red-800/50 rounded-2xl shadow-sm overflow-hidden">
            <div class="px-6 py-4 border-b border-red-800/30 bg-red-900/10">
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 bg-red-900/40 text-red-400 rounded-lg flex items-center justify-center text-sm font-bold">"!"</div>
                    <h3 class="font-bold text-red-400">{title}</h3>
                </div>
            </div>
            <div class="p-6">
                <p class="text-sm text-slate-400">{message}</p>
                {children.map(|c| view! { <div class="mt-4">{c()}</div> })}
            </div>
        </div>
    }.into_any()
}
