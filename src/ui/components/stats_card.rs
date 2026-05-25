use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Trend {
    Up,
    Down,
    Neutral,
}

#[component]
pub fn StatsCard(
    #[prop(into)] title: String,
    #[prop(into)] value: String,
    #[prop(into)] change: String,
    #[prop(into)] trend: Trend,
    #[prop(into)] icon: AnyView,
    #[prop(into)] color: String,
) -> impl IntoView {
    let trend_icon = match trend {
        Trend::Up => view! { 
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18" />
            </svg>
        },
        Trend::Down => view! { 
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
            </svg>
        },
        Trend::Neutral => view! { 
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14" />
            </svg>
        },
    };

    view! {
        <div class="p-6 bg-raised rounded-xl shadow-sm border border-border flex items-center justify-between">
            <div>
                <p class="text-sm font-medium text-label mb-1">{title}</p>
                <h3 class="text-2xl font-bold text-heading">{value}</h3>
                <div class="flex items-center mt-2">
                    <span class=format!("flex items-center text-xs font-semibold {}", 
                        if trend == Trend::Up { "text-emerald-500" } else if trend == Trend::Down { "text-rose-500" } else { "text-label" }
                    )>
                        {trend_icon}
                        <span class="ml-1">{change}</span>
                    </span>
                    <span class="ml-2 text-xs text-label">{ "vs last month" }</span>
                </div>
            </div>
            <div class=format!("p-3 rounded-lg bg-opacity-10 bg-muted {}", color)>
                {icon}
            </div>
        </div>
    }
}
