use leptos::prelude::*;

pub const SOURCE: &str = include_str!("input.rs");

#[component]
pub fn BaseInput(
    #[prop(into)] label: String,
    #[prop(into)] placeholder: String,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] set_value: WriteSignal<String>,
    #[prop(optional)] error: String,
) -> AnyView {
    let show_error = !error.is_empty();
    view! {
        <div class="space-y-1.5">
            <label class="text-sm font-semibold text-slate-300">{label}</label>
            <input
                type="text"
                class=format!("w-full px-4 py-2.5 rounded-xl border bg-slate-800 text-slate-100 placeholder-slate-500 transition-all focus:outline-none focus:ring-2 focus:border-transparent {}",
                    if show_error { "border-red-500/50 focus:ring-red-500" } else { "border-slate-600 focus:ring-indigo-500 hover:border-slate-500" })
                placeholder=placeholder
                prop:value=value
                on:input=move |ev| set_value.set(event_target_value(&ev))
            />
            {if show_error {
                Some(view! { <p class="text-xs text-red-400 mt-1">{error}</p> })
            } else {
                None
            }}
        </div>
    }.into_any()
}
