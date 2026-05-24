use leptos::prelude::*;

pub const SOURCE: &str = include_str!("accordion.rs");

#[derive(Clone)]
struct AccordionSection {
    title: &'static str,
    content: &'static str,
    icon: &'static str,
}

#[component]
pub fn AccordionDemo() -> AnyView {
    let sections = vec![
        AccordionSection {
            title: "Getting Started",
            content: "Leptos is a full-stack, isomorphic Rust web framework leveraging fine-grained reactivity to build declarative user interfaces.",
            icon: "🚀",
        },
        AccordionSection {
            title: "Components",
            content: "Components are the building blocks of Leptos UIs. Each component is a Rust function that returns a view.",
            icon: "🧩",
        },
        AccordionSection {
            title: "Reactivity",
            content: "Signals are reactive values that automatically track dependencies and update the DOM when changed.",
            icon: "⚡",
        },
    ];

    let active = RwSignal::new(None::<usize>);

    let toggle = move |idx: usize| {
        active.set(if active.get() == Some(idx) { None } else { Some(idx) });
    };

    view! {
        <div class="px-2.5 py-3 border-b border-slate-800">
            <p class="text-[10px] uppercase font-bold text-slate-500 tracking-wider px-3 mb-2">"Accordion Demo"</p>
            <div class="space-y-1">
                {sections.into_iter().enumerate().map(|(idx, section)| {
                    let is_open = move || active.get() == Some(idx);
                    let toggle_idx = toggle;
                    view! {
                        <div>
                            <button
                                class="flex items-center gap-2 w-full px-3 py-2 rounded-lg text-sm font-medium text-slate-400 hover:text-slate-100 hover:bg-slate-800 transition-colors text-left cursor-pointer"
                                on:click=move |_| toggle_idx(idx)
                            >
                                <span class="text-base shrink-0">{section.icon}</span>
                                <span class="flex-1 truncate">{section.title}</span>
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 20 20"
                                    fill="currentColor"
                                    class="w-4 h-4 shrink-0 transition-transform duration-200"
                                    class=("rotate-180", is_open)
                                >
                                    <path fill-rule="evenodd" d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z" clip-rule="evenodd"/>
                                </svg>
                            </button>
                            {move || {
                                if is_open() {
                                    view! {
                                        <div class="px-3 py-2 text-xs text-slate-500 leading-relaxed ml-1 border-l-2 border-indigo-500/30 mt-0.5 mb-1">
                                            {section.content}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }.into_any()
}
