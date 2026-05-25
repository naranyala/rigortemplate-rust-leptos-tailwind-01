use leptos::prelude::*;

#[component]
pub fn Showcase(
    name: &'static str,
    #[prop(optional)]
    description: &'static str,
    children: Children,
) -> AnyView {
    view! {
        <div class="dark:bg-slate-800/50 bg-white dark:border-slate-700/50 border-slate-200 rounded-2xl shadow-sm overflow-hidden border">
            <div class="px-6 py-4 border-b dark:border-slate-700/50 border-slate-200">
                <h3 class="font-bold dark:text-slate-200 text-slate-800">{name}</h3>
                {move || if !description.is_empty() {
                    view! { <p class="text-sm dark:text-slate-400 text-slate-600 mt-0.5">{description}</p> }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
            <div class="p-6">
                {children()}
            </div>
        </div>
    }.into_any()
}
