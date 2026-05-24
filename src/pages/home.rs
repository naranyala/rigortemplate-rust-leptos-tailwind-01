use leptos::prelude::*;
use crate::stdlib::components::{button::{BaseButton, ButtonVariant}, card::BaseCard, error_display::ErrorMessage};
use crate::store::{GlobalStore, Page};

fn time_ago(dt: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let dur = now.signed_duration_since(*dt);
    if dur.num_minutes() < 1 {
        "just now".to_string()
    } else if dur.num_minutes() < 60 {
        format!("{}m ago", dur.num_minutes())
    } else if dur.num_hours() < 24 {
        format!("{}h ago", dur.num_hours())
    } else {
        format!("{}d ago", dur.num_days())
    }
}

fn completion_rate(tasks: &[crate::services::task_service::Task]) -> f64 {
    let total = tasks.len();
    if total == 0 {
        0.0
    } else {
        let done = tasks.iter().filter(|t| t.completed).count();
        done as f64 / total as f64 * 100.0
    }
}

#[component]
pub fn Home() -> AnyView {
    let store = match use_context::<GlobalStore>() {
        Some(s) => s,
        None => {
            crate::error::log_error(&crate::error::AppError::Context(
                "GlobalStore not found in Home".into(),
            ));
            return view! {
                <div class="p-6 md:p-10 max-w-7xl mx-auto">
                    <ErrorMessage
                        title="Dashboard Error"
                        message="Required application state is missing. Try restarting the page."
                    />
                </div>
            }.into_any();
        }
    };

    let ts = store.task_service.clone();
    let page = store.current_page;

    let total = {
        let ts = ts.clone();
        Memo::new(move |_| ts.tasks.get().len())
    };

    let completed = {
        let ts = ts.clone();
        Memo::new(move |_| ts.tasks.get().iter().filter(|t| t.completed).count())
    };

    let pending = Memo::new(move |_| total.get() - completed.get());

    let rate = {
        let ts = ts.clone();
        Memo::new(move |_| completion_rate(&ts.tasks.get()))
    };

    let recent = {
        let ts = ts.clone();
        Memo::new(move |_| {
            let mut all = ts.tasks.get();
            all.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            all.truncate(6);
            all
        })
    };

    let go_demo = move |_| page.set(Page::Demo);

    view! {
        <div class="p-6 md:p-10 max-w-7xl mx-auto space-y-8">

            <header class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                <div>
                    <div class="flex items-center gap-2 text-sm font-medium text-slate-500 mb-0.5">
                        <span>"Workspace"</span>
                        <span class="text-slate-600">"/"</span>
                        <span class="text-slate-300">"Dashboard"</span>
                    </div>
                    <h1 class="text-3xl font-black tracking-tight text-slate-100">"Overview"</h1>
                    <p class="text-sm text-slate-500 mt-0.5">"Real-time project metrics and activity."</p>
                </div>
                <div class="flex gap-3">
                    <BaseButton variant=ButtonVariant::Outline>"Refresh"</BaseButton>
                    <BaseButton variant=ButtonVariant::Primary on:click=go_demo>"New Task"</BaseButton>
                </div>
            </header>

            <div class="grid grid-cols-2 lg:grid-cols-4 gap-5">
                <div class="bg-slate-800/80 border border-slate-700/50 rounded-2xl p-5 shadow-sm space-y-3">
                    <div class="flex items-center justify-between">
                        <div class="w-10 h-10 bg-indigo-900/30 text-indigo-400 rounded-xl flex items-center justify-center text-lg">"📋"</div>
                        <span class="text-xs font-bold px-2 py-1 rounded-full bg-indigo-900/30 text-indigo-400">{move || total.get().to_string()}</span>
                    </div>
                    <div>
                        <p class="text-xs font-medium text-slate-500 uppercase tracking-wider">"Total Tasks"</p>
                        <p class="text-3xl font-black text-slate-100 mt-0.5">{move || total.get()}</p>
                    </div>
                </div>

                <div class="bg-slate-800/80 border border-slate-700/50 rounded-2xl p-5 shadow-sm space-y-3">
                    <div class="flex items-center justify-between">
                        <div class="w-10 h-10 bg-emerald-900/30 text-emerald-400 rounded-xl flex items-center justify-center text-lg">"✅"</div>
                        <span class="text-xs font-bold px-2 py-1 rounded-full bg-emerald-900/30 text-emerald-400">{move || format!("{:.0}%", rate.get())}</span>
                    </div>
                    <div>
                        <p class="text-xs font-medium text-slate-500 uppercase tracking-wider">"Completed"</p>
                        <p class="text-3xl font-black text-slate-100 mt-0.5">{move || completed.get()}</p>
                    </div>
                </div>

                <div class="bg-slate-800/80 border border-slate-700/50 rounded-2xl p-5 shadow-sm space-y-3">
                    <div class="flex items-center justify-between">
                        <div class="w-10 h-10 bg-amber-900/30 text-amber-400 rounded-xl flex items-center justify-center text-lg">"⏳"</div>
                        <span class="text-xs font-bold px-2 py-1 rounded-full bg-amber-900/30 text-amber-400">{move || pending.get().to_string()}</span>
                    </div>
                    <div>
                        <p class="text-xs font-medium text-slate-500 uppercase tracking-wider">"Pending"</p>
                        <p class="text-3xl font-black text-slate-100 mt-0.5">{move || pending.get()}</p>
                    </div>
                </div>

                <div class="bg-slate-800/80 border border-slate-700/50 rounded-2xl p-5 shadow-sm space-y-3">
                    <div class="flex items-center justify-between">
                        <div class="w-10 h-10 bg-sky-900/30 text-sky-400 rounded-xl flex items-center justify-center text-lg">"🎯"</div>
                        <span class="text-xs font-bold px-2 py-1 rounded-full bg-sky-900/30 text-sky-400">{move || format!("{:.0}%", rate.get())}</span>
                    </div>
                    <div>
                        <p class="text-xs font-medium text-slate-500 uppercase tracking-wider">"Completion Rate"</p>
                        <p class="text-3xl font-black text-slate-100 mt-0.5">{move || format!("{:.0}%", rate.get())}</p>
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="lg:col-span-2 space-y-6">
                    <BaseCard title="Recent Activity">
                        {move || {
                            let items = recent.get();
                            if items.is_empty() {
                                view! {
                                    <div class="flex flex-col items-center justify-center py-12 text-center space-y-3">
                                        <div class="w-14 h-14 bg-slate-800 rounded-2xl flex items-center justify-center text-2xl">"📭"</div>
                                        <p class="text-sm font-medium text-slate-500">"No activity yet. Create your first task."</p>
                                        <BaseButton variant=ButtonVariant::Primary on:click=go_demo>"Create a Task"</BaseButton>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-1">
                                        {items.into_iter().map(|task| {
                                            let id = task.id.clone();
                                            let is_done = task.completed;
                                            let ts_cb = ts.clone();
                                            view! {
                                                <div class="flex items-start gap-4 p-3 -mx-2 rounded-xl hover:bg-slate-800/50 transition-all group">
                                                    <div class=format!("w-2.5 h-2.5 rounded-full mt-1.5 shrink-0 {}",
                                                        if is_done { "bg-emerald-400" } else { "bg-slate-600" })
                                                    ></div>
                                                    <div class="flex-1 min-w-0">
                                                        <p class=format!("text-sm font-medium truncate {}",
                                                            if is_done { "line-through text-slate-500" } else { "text-slate-200" })
                                                        >
                                                            {task.text}
                                                        </p>
                                                        <p class="text-xs text-slate-500 mt-0.5">{time_ago(&task.created_at)}</p>
                                                    </div>
                                                    <div class="opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
                                                        <input
                                                            type="checkbox"
                                                            class="w-4 h-4 rounded border-slate-600 bg-slate-700 text-indigo-500 focus:ring-indigo-500 cursor-pointer"
                                                            prop:checked=is_done
                                                            on:click=move |_| ts_cb.toggle_task(id.clone())
                                                        />
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        }}
                    </BaseCard>
                </div>

                <div class="space-y-6">
                    <BaseCard title="Quick Actions">
                        <div class="space-y-3">
                            <button
                                class="w-full flex items-center gap-3 px-4 py-3 rounded-xl bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 transition-all font-medium text-sm text-left"
                                on:click=go_demo
                            >
                                <span class="text-lg">"⚡"</span>
                                <div>
                                    <p class="font-bold">"Open Task Manager"</p>
                                    <p class="text-xs text-indigo-400/60 mt-0.5">"Create, edit, and track tasks"</p>
                                </div>
                            </button>
                            <button class="w-full flex items-center gap-3 px-4 py-3 rounded-xl bg-slate-800 text-slate-600 cursor-not-allowed transition-all font-medium text-sm text-left">
                                <span class="text-lg">"📊"</span>
                                <div>
                                    <p class="font-bold">"Generate Report"</p>
                                    <p class="text-xs text-slate-600 mt-0.5">"Coming soon"</p>
                                </div>
                            </button>
                            <button class="w-full flex items-center gap-3 px-4 py-3 rounded-xl bg-slate-800 text-slate-600 cursor-not-allowed transition-all font-medium text-sm text-left">
                                <span class="text-lg">"⚙️"</span>
                                <div>
                                    <p class="font-bold">"Settings"</p>
                                    <p class="text-xs text-slate-600 mt-0.5">"Coming soon"</p>
                                </div>
                            </button>
                        </div>
                    </BaseCard>

                    <div class="bg-slate-800/80 border border-slate-700/50 rounded-2xl p-6 shadow-sm space-y-4">
                        <h3 class="font-bold text-sm uppercase tracking-wider text-slate-500">"Task Breakdown"</h3>
                        {move || {
                            let t = total.get();
                            let c = completed.get();
                            let p = t - c;
                            if t == 0 {
                                view! {
                                    <p class="text-xs text-slate-500 text-center py-4">"No data to display."</p>
                                }.into_any()
                            } else {
                                let c_pct = c as f64 / t as f64 * 100.0;
                                let p_pct = p as f64 / t as f64 * 100.0;
                                view! {
                                    <div class="space-y-4">
                                        <div class="flex h-3 rounded-full overflow-hidden bg-slate-700">
                                            <div
                                                class="bg-emerald-500 transition-all duration-500"
                                                style=format!("width: {c_pct}%")
                                            ></div>
                                            <div
                                                class="bg-amber-500 transition-all duration-500"
                                                style=format!("width: {p_pct}%")
                                            ></div>
                                        </div>
                                        <div class="flex justify-between text-xs text-slate-400">
                                            <div class="flex items-center gap-2">
                                                <div class="w-2.5 h-2.5 rounded-full bg-emerald-500"></div>
                                                <span class="font-medium text-slate-300">"Done"</span>
                                                <span class="text-slate-500">{c} tasks</span>
                                            </div>
                                            <div class="flex items-center gap-2">
                                                <div class="w-2.5 h-2.5 rounded-full bg-amber-500"></div>
                                                <span class="font-medium text-slate-300">"Open"</span>
                                                <span class="text-slate-500">{p} tasks</span>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }.into_any()
}
