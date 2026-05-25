use leptos::prelude::*;

pub const SOURCE: &str = include_str!("input.rs");

#[component]
pub fn TheInput(
    label: &'static str,
    #[prop(into)]
    value: RwSignal<String>,
    #[prop(optional)]
    placeholder: &'static str,
    #[prop(optional)]
    input_type: &'static str,
) -> AnyView {
    let placeholder = move || placeholder;
    let input_type = move || input_type;

    view! {
        <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium uppercase tracking-wider dark:text-slate-400 text-slate-600">{label}</label>
            <input
                type=input_type
                placeholder=placeholder
                prop:value=value
                on:input=move |ev| value.set(event_target_value(&ev))
                class="px-3 py-2 rounded-xl text-sm border transition-colors outline-none focus:ring-2 focus:ring-indigo-500/40 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-200 dark:placeholder-slate-500 bg-white border-slate-300 text-slate-800 placeholder-slate-400"
            />
        </div>
    }.into_any()
}
