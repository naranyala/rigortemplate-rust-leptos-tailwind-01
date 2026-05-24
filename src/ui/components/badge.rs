use leptos::prelude::*;

pub const SOURCE: &str = include_str!("badge.rs");

#[component]
pub fn BaseBadge(
    #[prop(into)] text: String,
    #[prop(into)] color: String,
) -> AnyView {
    view! {
        <span class=format!("text-xs font-bold px-2 py-1 rounded-full {}", color)>
            {text}
        </span>
    }.into_any()
}
