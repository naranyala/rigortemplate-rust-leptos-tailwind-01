use leptos::prelude::*;
use crate::ui::components::stats_card::{StatsCard, Trend};

#[component]
pub fn StatsDemo() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <StatsCard 
                title="Total Revenue" 
                value="$45,231.89" 
                change="+20.1%" 
                trend=Trend::Up 
                color="text-emerald-500"
                icon=view! { 
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                }.into_any() 
            />
            <StatsCard 
                title="Active Users" 
                value="+2,350" 
                change="+180.1%" 
                trend=Trend::Up 
                color="text-blue-500"
                icon=view! { 
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                    </svg>
                }.into_any() 
            />
            <StatsCard 
                title="Sales" 
                value="+12,234" 
                change="-3.4%" 
                trend=Trend::Down 
                color="text-amber-500"
                icon=view! { 
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                    </svg>
                }.into_any() 
            />
            <StatsCard 
                title="Active Now" 
                value="+573" 
                change="+201" 
                trend=Trend::Up 
                color="text-indigo-500"
                icon=view! { 
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                    </svg>
                }.into_any() 
            />
        </div>
    }
}

pub const SOURCE: &str = r#"
#[component]
pub fn StatsDemo() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <StatsCard 
                title="Total Revenue" 
                value="$45,231.89" 
                change="+20.1%" 
                trend=Trend::Up 
                color="text-emerald-500"
                icon=view! { <Icon name="currency-dollar" /> } 
            />
            // ... other cards
        </div>
    }
}
"#;
