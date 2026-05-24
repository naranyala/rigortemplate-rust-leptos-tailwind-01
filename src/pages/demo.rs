use leptos::prelude::*;
use crate::pages::demos::{
    accordion::{AccordionDemo, SOURCE as ACCORDION_SRC},
    sliding_panel::{SlidingPanelDemo, SOURCE as PANEL_SRC},
    stats::{StatsDemo, SOURCE as STATS_SRC},
    tabs::{TabsDemo, SOURCE as TABS_SRC},
    modal::{ModalDemo, SOURCE as MODAL_SRC},
};
use crate::ui::components::{
    button::{BaseButton, ButtonVariant, SOURCE as BUTTON_SRC},
    input::{BaseInput, SOURCE as INPUT_SRC},
    card::{BaseCard, SOURCE as CARD_SRC},
    badge::{BaseBadge, SOURCE as BADGE_SRC},
    code_block::CodeBlock,
    error_display::ErrorMessage,
    showcase::Showcase,
};
use crate::core::services::task_service::TaskService;
use crate::core::store::GlobalStore;


#[component]
pub fn Demo() -> AnyView {
    let store = match use_context::<GlobalStore>() {
        Some(s) => s,
        None => {
            crate::error::log_error(&crate::error::AppError::Context(
                "GlobalStore not found. Ensure App() provides it.".into(),
            ));
            return view! {
                <div class="p-6 md:p-10 max-w-6xl mx-auto">
                    <ErrorMessage
                        title="Application Error"
                        message="Required application state is missing. Try restarting the page."
                    />
                </div>
            }.into_any();
        }
    };
    let task_service = store.services.get::<TaskService>().expect("TaskService not registered");
    let (input, set_input) = signal(String::new());

    let ts = task_service.clone();
    let add_task = move |_| {
        let text = input.get();
        if !text.is_empty() {
            ts.add_task(text);
            set_input.set(String::new());
        }
    };

    let ts = task_service.clone();
    let progress = move || ts.get_progress();

    let ts = task_service.clone();
    let each = move || ts.tasks.get();

    let ts = task_service.clone();
    let total_len = move || ts.tasks.get().len();

    let ts = task_service.clone();
    let done_count = move || ts.tasks.get().iter().filter(|t| t.completed).count();

    let ts = task_service.clone();
    let pending_count = move || ts.tasks.get().iter().filter(|t| !t.completed).count();

    let (showcase_input, set_showcase_input) = signal(String::new());

    view! {
        <div class="p-6 md:p-10 max-w-6xl mx-auto space-y-12">
            <div class="flex flex-col lg:flex-row gap-8">
                <div class="flex-1 space-y-8">
                    <div class="flex items-center justify-between">
                        <div>
                            <h1 class="text-3xl font-black tracking-tight text-slate-100">"Task Manager"</h1>
                            <p class="text-slate-400">"Organize and track your development milestones."</p>
                        </div>
                        <div class="hidden sm:flex items-center gap-4 bg-slate-800/80 border border-slate-700/50 p-3 rounded-2xl shadow-sm">
                            <div class="text-right">
                                <p class="text-[10px] uppercase font-bold text-slate-500">"Progress"</p>
                                <p class="text-xl font-black text-indigo-400">{move || format!("{:.0}%", progress())}</p>
                            </div>
                            <div class="w-12 h-12 rounded-full border-4 border-slate-700 border-t-indigo-400 animate-spin-slow flex items-center justify-center">
                                <div class="w-1 h-1 bg-indigo-400 rounded-full"></div>
                            </div>
                        </div>
                    </div>

                    <div class="bg-slate-800/80 border border-slate-700/50 shadow-sm rounded-2xl p-6">
                        <div class="flex gap-3 mb-8">
                            <div class="relative flex-1">
                                <BaseInput
                                    label="Task Description"
                                    placeholder="What needs to be done?"
                                    value=input
                                    set_value=set_input
                                />
                            </div>
                            <div class="self-end">
                                <BaseButton variant=ButtonVariant::Primary on:click=add_task>
                                    "Add"
                                </BaseButton>
                            </div>
                        </div>

                        <div class="space-y-3">
                            <For
                                each=each
                                key=|task| task.id.clone()
                                children=move |task| {
                                    let ts_change = task_service.clone();
                                    let ts_remove = task_service.clone();
                                    let id = task.id.clone();
                                    let id_remove = id.clone();
                                    view! {
                                        <div class="flex items-center justify-between p-4 bg-slate-800/50 border border-transparent hover:border-indigo-500/30 rounded-2xl transition-all group">
                                            <div class="flex items-center gap-4">
                                                <input
                                                    type="checkbox"
                                                    class="w-4 h-4 rounded border-slate-600 bg-slate-700 text-indigo-500 focus:ring-indigo-500 cursor-pointer"
                                                    on:change=move |_| ts_change.toggle_task(id.clone())
                                                    prop:checked=task.completed
                                                />
                                                <span class=move || if task.completed { "line-through text-slate-500" } else { "font-medium text-slate-200" }>
                                                    {task.text}
                                                </span>
                                            </div>
                                            <BaseButton variant=ButtonVariant::Ghost on:click=move |_| ts_remove.remove_task(id_remove.clone())>
                                                "✕"
                                            </BaseButton>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                </div>

                <div class="w-full lg:w-80 space-y-6">
                    <div class="bg-indigo-500/10 text-white p-8 rounded-2xl border border-indigo-500/20 relative overflow-hidden">
                        <div class="relative z-10">
                            <h3 class="text-xl font-bold mb-3 text-indigo-300">"Pro Tip"</h3>
                            <p class="text-sm text-indigo-300/60 leading-relaxed">"Break large projects into smaller, atomic tasks to maintain momentum and visibility."</p>
                        </div>
                        <div class="absolute -right-4 -bottom-4 text-8xl opacity-10 rotate-12">"💡"</div>
                    </div>
                    <BaseCard title="Stats">
                        <div class="space-y-4">
                            <div class="flex justify-between items-center p-3 bg-slate-800/50 rounded-xl">
                                <span class="text-sm text-slate-400">"Total"</span>
                                <span class="font-black text-slate-100">{total_len}</span>
                            </div>
                            <div class="flex justify-between items-center p-3 bg-emerald-900/20 text-emerald-400 rounded-xl">
                                <span class="text-sm font-medium">"Done"</span>
                                <span class="font-black">{done_count}</span>
                            </div>
                            <div class="flex justify-between items-center p-3 bg-amber-900/20 text-amber-400 rounded-xl">
                                <span class="text-sm font-medium">"Pending"</span>
                                <span class="font-black">{pending_count}</span>
                            </div>
                        </div>
                    </BaseCard>
                </div>
            </div>

            <hr class="border-slate-800" />

            <section class="space-y-8">
                <div>
                    <h2 class="text-2xl font-black tracking-tight text-slate-100">"Component Library"</h2>
                    <p class="text-slate-400 mt-1">"Interactive showcase with live source code from each single-file component."</p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Showcase label="BaseButton" source=BUTTON_SRC>
                        <BaseButton variant=ButtonVariant::Primary>"Primary"</BaseButton>
                        <BaseButton variant=ButtonVariant::Secondary>"Secondary"</BaseButton>
                        <BaseButton variant=ButtonVariant::Outline>"Outline"</BaseButton>
                        <BaseButton variant=ButtonVariant::Ghost>"Ghost"</BaseButton>
                        <BaseButton variant=ButtonVariant::Error>"Error"</BaseButton>
                    </Showcase>

                    <Showcase label="BaseInput" source=INPUT_SRC>
                        <BaseInput
                            label="Sample Input"
                            placeholder="Type something..."
                            value=showcase_input
                            set_value=set_showcase_input
                        />
                    </Showcase>

                    <Showcase label="BaseCard" source=CARD_SRC>
                        <BaseCard title="Card Title">
                            <p class="text-sm text-slate-400">"Card body content with any children."</p>
                        </BaseCard>
                        <BaseCard>
                            <p class="text-sm text-slate-400">"Card without title."</p>
                        </BaseCard>
                    </Showcase>

                    <Showcase label="BaseBadge" source=BADGE_SRC>
                        <BaseBadge text=String::from("v2.1.0") color=String::from("bg-indigo-500/20 text-indigo-300") />
                        <BaseBadge text=String::from("Done") color=String::from("bg-emerald-500/20 text-emerald-300") />
                        <BaseBadge text=String::from("Pending") color=String::from("bg-amber-500/20 text-amber-300") />
                        <BaseBadge text=String::from("Error") color=String::from("bg-red-500/20 text-red-300") />
                    </Showcase>
                </div>
            </section>

            <section class="space-y-8">
                <div>
                    <h2 class="text-2xl font-black tracking-tight text-slate-100">"UI Pattern Demos"</h2>
                    <p class="text-slate-400 mt-1">"Live interactive demos with full source code and syntax highlighting."</p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <h3 class="text-sm font-semibold text-slate-300 uppercase tracking-wider">"Accordion"</h3>
                        </div>
                        <div class="p-5 bg-slate-800/50 border border-slate-700/50 rounded-xl">
                            <AccordionDemo />
                        </div>
                        <CodeBlock source=ACCORDION_SRC />
                    </div>

                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <h3 class="text-sm font-semibold text-slate-300 uppercase tracking-wider">"Sliding Panel"</h3>
                        </div>
                        <div class="p-5 bg-slate-800/50 border border-slate-700/50 rounded-xl">
                            <SlidingPanelDemo />
                        </div>
                        <CodeBlock source=PANEL_SRC />
                    </div>
                </div>
            </section>

            <section class="space-y-6">
                <div>
                    <h2 class="text-2xl font-black tracking-tight text-slate-100">"Showcase Source"</h2>
                    <p class="text-slate-400 mt-1">"The CodeBlock and Showcase components used to build this library page."</p>
                </div>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Showcase label="CodeBlock" source=crate::ui::components::code_block::SOURCE>
                        <p class="text-sm text-slate-400">"A dark-theme code viewer with window controls header and monospace rendering — used to display component source files inline."</p>
                    </Showcase>
                    <Showcase label="Showcase" source=crate::ui::components::showcase::SOURCE>
                        <p class="text-sm text-slate-400">"Wrapper that renders a component preview alongside its labeled header and source code below."</p>
                    </Showcase>
                </div>
            </section>
        </div>
    }.into_any()
}
