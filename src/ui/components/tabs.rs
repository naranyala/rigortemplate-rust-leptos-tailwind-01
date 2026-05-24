use leptos::prelude::*;

pub struct TabItem {
    pub id: String,
    pub label: String,
    pub content: Box<dyn Fn() -> AnyView + Send + Sync>,
}

#[component]
pub fn Tabs(
    #[prop(into)] items: Vec<std::sync::Arc<TabItem>>,
    #[prop(into)] default_tab: String,
) -> impl IntoView {
    let active_tab = RwSignal::new(default_tab);
    let items_arc = std::sync::Arc::new(items);

    let items_for_first_for = items_arc.clone();
    let items_for_second_for = items_arc.clone();

    view! {
        <div class="flex flex-col w-full">
            <div class="flex border-b border-slate-200 dark:border-slate-700">
                <For
                    each=move || (*items_for_first_for).clone()
                    key=|item| item.id.clone()
                    children=move |item| {
                        let id = item.id.clone();
                        let id_for_class = id.clone();
                        let id_for_click = id.clone();
                        let id_for_div = id.clone();
                        view! {
                            <button 
                                class=move || {
                                    format!(
                                        "px-4 py-2 text-sm font-medium transition-colors relative outline-none {}",
                                        if active_tab.get() == id_for_class {
                                            "text-blue-600 dark:text-blue-400"
                                        } else {
                                            "text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200"
                                        }
                                    )
                                }
                                on:click=move |_| active_tab.set(id_for_click.clone())
                            >
                                {item.label.clone()}
                                <div class=move || {
                                    format!(
                                        "absolute bottom-0 left-0 right-0 h-0.5 bg-blue-600 dark:bg-blue-400 transition-transform duration-200 {}",
                                        if active_tab.get() == id_for_div { "translate-x-0" } else { "translate-x-full" }
                                    )
                                }/>
                            </button>
                        }
                    }
                />
            </div>
            <div class="p-4">
                <For
                    each=move || (*items_for_second_for).clone()
                    key=|item| item.id.clone()
                    children=move |item| {
                        let id = item.id.clone();
                        view! {
                            <div class=move || {
                                let id = id.clone();
                                format!("transition-all duration-200 {}", 
                                    if active_tab.get() == id { "block" } else { "hidden" }
                                )
                            }>
                                {(item.content)()}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
