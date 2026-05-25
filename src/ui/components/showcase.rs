use leptos::prelude::*;

pub const SOURCE: &str = include_str!("showcase.rs");

#[component]
pub fn Showcase(
    name: &'static str,
    #[prop(optional)]
    description: &'static str,
    children: Children,
) -> AnyView {
    view! {
        <div class="bg-raised border-border rounded-2xl shadow-sm overflow-hidden border">
            <div class="px-6 py-4 border-b border-border">
                <h3 class="font-bold text-heading">{name}</h3>
                {move || if !description.is_empty() {
                    view! { <p class="text-sm text-label mt-0.5">{description}</p> }.into_any()
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
