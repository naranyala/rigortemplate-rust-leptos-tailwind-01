use leptos::prelude::*;

pub const SOURCE: &str = include_str!("card.rs");

#[component]
pub fn BaseCard(
    #[prop(into)]
    #[prop(optional)]
    title: Option<String>,
    children: Children,
) -> AnyView {
    view! {
        <div class="dark:bg-slate-800/80 bg-white dark:border-slate-700/50 border-slate-200 rounded-2xl shadow-sm overflow-hidden border">
            {move || title.clone().map(|t| view! {
                <div class="px-6 py-4 border-b dark:border-slate-700/50 border-slate-200">
                    <h3 class="font-bold dark:text-slate-200 text-slate-800">{t}</h3>
                </div>
            })}
            <div class="p-6">
                {children()}
            </div>
        </div>
    }.into_any()
}
