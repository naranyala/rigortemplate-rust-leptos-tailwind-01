use leptos::prelude::*;

pub const SOURCE: &str = include_str!("button.rs");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Error,
}

impl ButtonVariant {
    fn to_class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-indigo-500 text-white hover:bg-indigo-600 shadow-sm shadow-indigo-500/20",
            ButtonVariant::Secondary => "bg-slate-700 text-slate-200 hover:bg-slate-600",
            ButtonVariant::Outline => "border border-slate-600 text-slate-300 hover:bg-slate-800",
            ButtonVariant::Ghost => "text-slate-400 hover:text-slate-100 hover:bg-slate-800",
            ButtonVariant::Error => "bg-red-500 text-white hover:bg-red-600 shadow-sm shadow-red-500/20",
        }
    }
}

#[component]
pub fn BaseButton(
    #[prop(into)] variant: ButtonVariant,
    #[prop(optional)] disabled: bool,
    children: Children,
) -> AnyView {
    view! {
        <button
            class=format!("inline-flex items-center justify-center gap-2 px-4 py-2 rounded-xl text-sm font-medium transition-all active:scale-[0.97] focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 {}", variant.to_class())
            disabled=disabled
        >
            {children()}
        </button>
    }.into_any()
}
