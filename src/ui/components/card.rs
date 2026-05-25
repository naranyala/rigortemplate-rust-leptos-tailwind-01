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
        <div class="bg-raised border-border rounded-2xl shadow-sm overflow-hidden border">
            {move || title.clone().map(|t| view! {
                <div class="px-6 py-4 border-b border-border">
                    <h3 class="font-bold text-heading">{t}</h3>
                </div>
            })}
            <div class="p-6">
                {children()}
            </div>
        </div>
    }.into_any()
}
