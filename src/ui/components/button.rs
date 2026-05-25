use leptos::prelude::*;

pub const SOURCE: &str = include_str!("button.rs");

#[derive(Clone, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
}

#[component]
pub fn Btn(
    children: Children,
    #[prop(into)]
    #[prop(optional)]
    on_click: Option<Box<dyn Fn(leptos::ev::MouseEvent) + Send + Sync>>,
    #[prop(optional)]
    variant: ButtonVariant,
    #[prop(into)]
    #[prop(optional)]
    class: Option<String>,
) -> AnyView {
    let base = "inline-flex items-center justify-center gap-2 px-4 py-2 rounded-xl text-sm font-bold transition-all duration-200 cursor-pointer select-none disabled:opacity-50 disabled:pointer-events-none";
    let variant_class = match variant {
        ButtonVariant::Primary => "bg-indigo-500 text-white hover:bg-indigo-600",
        ButtonVariant::Secondary => "dark:bg-slate-700 dark:text-slate-200 dark:hover:bg-slate-600 bg-slate-200 text-slate-700 hover:bg-slate-300",
        ButtonVariant::Outline => "border-2 dark:border-slate-600 dark:text-slate-300 dark:hover:bg-slate-800 border-slate-300 text-slate-700 hover:bg-slate-100",
        ButtonVariant::Ghost => "dark:text-slate-400 dark:hover:bg-slate-800 text-slate-600 hover:bg-slate-100",
    };

    let cls = move || {
        let mut s = format!("{} {}", base, variant_class);
        if let Some(ref extra) = class {
            s = format!("{} {}", s, extra);
        }
        s
    };

    view! {
        <button class=cls on:click=move |ev| { if let Some(ref cb) = on_click { cb(ev); } }>
            {children()}
        </button>
    }.into_any()
}
