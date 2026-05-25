use leptos::prelude::*;
use crate::ui::pages::demos::{
    accordion::{AccordionDemo, SOURCE as ACCORDION_SRC},
    sliding_panel::{SlidingPanelDemo, SOURCE as PANEL_SRC},
};
use crate::ui::components::{
    button::{Btn, ButtonVariant},
    input::TheInput,
    card::BaseCard,
    badge::BaseBadge,
    code_block::CodeBlock,
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
                    <div class="p-4 rounded-xl bg-red-50 border border-red-200 dark:bg-red-900/30 dark:border-red-800">
                        <p class="text-sm font-medium text-red-700 dark:text-red-400">"Application Error"</p>
                        <p class="text-xs text-red-600 dark:text-red-300 mt-1">"Required application state is missing. Try restarting the page."</p>
                    </div>
                </div>
            }.into_any();
        }
    };
    let task_service = store.services.get::<TaskService>().expect("TaskService not registered");
    let input = RwSignal::new(String::new());

    let ts = task_service.clone();
    let add_task = move |_| {
        let text = input.get();
        if !text.is_empty() {
            ts.add_task(text);
            input.set(String::new());
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

    let showcase_input = RwSignal::new(String::new());

    view! {
        <div class="p-6 md:p-10 max-w-6xl mx-auto space-y-12">
            <div class="flex flex-col lg:flex-row gap-8">
                <div class="flex-1 space-y-8">
                    <div class="flex items-center justify-between">
                        <div>
                            <h1 class="text-3xl font-black tracking-tight text-heading">"Task Manager"</h1>
                            <p class="text-body">"Organize and track your development milestones."</p>
                        </div>
                        <div class="hidden sm:flex items-center gap-4 p-3 rounded-2xl shadow-sm border bg-raised border-border">
                            <div class="text-right">
                                <p class="text-[10px] uppercase font-bold text-label">"Progress"</p>
                                <p class="text-xl font-black text-accent">{move || format!("{:.0}%", progress())}</p>
                            </div>
                            <div class="w-12 h-12 rounded-full border-4 border-t-accent animate-spin-slow flex items-center justify-center border-input relative">
                                <div class="w-1 h-1 bg-accent rounded-full"></div>
                            </div>
                        </div>
                    </div>

                    <div class="bg-raised border-border shadow-sm rounded-2xl p-6 border">
                        <div class="flex gap-3 mb-8">
                            <div class="relative flex-1">
                                <TheInput
                                    label="Task Description"
                                    placeholder="What needs to be done?"
                                    value=input
                                />
                            </div>
                            <div class="self-end">
                                <Btn variant=ButtonVariant::Primary on:click=add_task>
                                    "Add"
                                </Btn>
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
                                        <div class="flex items-center justify-between p-4 border border-transparent hover:border-accent/30 rounded-2xl transition-all group bg-muted">
                                            <div class="flex items-center gap-4">
                                                <input
                                                    type="checkbox"
                                                    class="w-4 h-4 rounded text-accent focus:ring-accent cursor-pointer border-input bg-surface"
                                                    on:change=move |_| ts_change.toggle_task(id.clone())
                                                    prop:checked=task.completed
                                                />
                                                <span class=move || {
                                                    if task.completed { "line-through text-label" } else { "font-medium text-heading" }
                                                }>
                                                    {task.text}
                                                </span>
                                            </div>
                                            <Btn variant=ButtonVariant::Ghost on:click=move |_| ts_remove.remove_task(id_remove.clone())>
                                                "X"
                                            </Btn>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                </div>

                <div class="w-full lg:w-80 space-y-6">
                    <div class="bg-accent/10 text-white p-8 rounded-2xl border border-accent/20 relative overflow-hidden">
                        <div class="relative z-10">
                            <h3 class="text-xl font-bold mb-3 text-accent-text">"Pro Tip"</h3>
                            <p class="text-sm text-accent/60 leading-relaxed">"Break large projects into smaller, atomic tasks to maintain momentum and visibility."</p>
                        </div>
                    </div>
                    <BaseCard title="Stats">
                        <div class="space-y-4">
                            <div class="flex justify-between items-center p-3 rounded-xl bg-muted">
                                <span class="text-sm text-label">"Total"</span>
                                <span class="font-black text-heading">{total_len}</span>
                            </div>
                            <div class="flex justify-between items-center p-3 bg-emerald-900/20 text-emerald-500 rounded-xl">
                                <span class="text-sm font-medium">"Done"</span>
                                <span class="font-black">{done_count}</span>
                            </div>
                            <div class="flex justify-between items-center p-3 bg-amber-900/20 text-amber-500 rounded-xl">
                                <span class="text-sm font-medium">"Pending"</span>
                                <span class="font-black">{pending_count}</span>
                            </div>
                        </div>
                    </BaseCard>
                </div>
            </div>

            <hr class="border-border" />

            <section class="space-y-8">
                <div>
                    <h2 class="text-2xl font-black tracking-tight text-heading">"Component Library"</h2>
                    <p class="text-body mt-1">"Interactive showcase with live source code from each single-file component."</p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Showcase name="Btn">
                        <Btn variant=ButtonVariant::Primary>"Primary"</Btn>
                        <Btn variant=ButtonVariant::Secondary>"Secondary"</Btn>
                        <Btn variant=ButtonVariant::Outline>"Outline"</Btn>
                        <Btn variant=ButtonVariant::Ghost>"Ghost"</Btn>
                    </Showcase>

                    <Showcase name="TheInput">
                        <TheInput
                            label="Sample Input"
                            placeholder="Type something..."
                            value=showcase_input
                        />
                    </Showcase>

                    <Showcase name="BaseCard">
                        <BaseCard title="Card Title">
                            <p class="text-sm text-body">"Card body content with any children."</p>
                        </BaseCard>
                        <BaseCard>
                            <p class="text-sm text-body">"Card without title."</p>
                        </BaseCard>
                    </Showcase>

                    <Showcase name="BaseBadge">
                        <BaseBadge text=String::from("v2.1.0") color=String::from("bg-accent/20 text-accent-text") />
                        <BaseBadge text=String::from("Done") color=String::from("bg-emerald-500/20 text-emerald-700") />
                        <BaseBadge text=String::from("Pending") color=String::from("bg-amber-500/20 text-amber-700") />
                        <BaseBadge text=String::from("Error") color=String::from("bg-red-500/20 text-red-700") />
                    </Showcase>
                </div>
            </section>

            <section class="space-y-8">
                <div>
                    <h2 class="text-2xl font-black tracking-tight text-heading">"UI Pattern Demos"</h2>
                    <p class="text-body mt-1">"Live interactive demos with full source code and syntax highlighting."</p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <h3 class="text-sm font-semibold uppercase tracking-wider text-body">"Accordion"</h3>
                        </div>
                        <div class="p-5 rounded-xl border bg-raised border-border">
                            <AccordionDemo />
                        </div>
                        <CodeBlock source=ACCORDION_SRC />
                    </div>

                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <h3 class="text-sm font-semibold uppercase tracking-wider text-body">"Sliding Panel"</h3>
                        </div>
                        <div class="p-5 rounded-xl border bg-raised border-border">
                            <SlidingPanelDemo />
                        </div>
                        <CodeBlock source=PANEL_SRC />
                    </div>
                </div>
            </section>

            <section class="space-y-6">
                <div>
                    <h2 class="text-2xl font-black tracking-tight text-heading">"Showcase Source"</h2>
                    <p class="text-body mt-1">"The CodeBlock and Showcase components used to build this library page."</p>
                </div>
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                        <Showcase name="CodeBlock">
                            <CodeBlock source=crate::ui::components::code_block::SOURCE />
                        </Showcase>
                        <Showcase name="Showcase">
                            <CodeBlock source=crate::ui::components::showcase::SOURCE />
                        </Showcase>
                    </div>

            </section>
        </div>
    }.into_any()
}
