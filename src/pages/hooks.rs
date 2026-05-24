use leptos::prelude::*;
use crate::stdlib::hooks::*;
use crate::stdlib::components::showcase::Showcase;
use crate::stdlib::components::code_block::CodeBlock;

#[component]
pub fn Hooks() -> AnyView {
    view! {
        <div class="p-6 md:p-10 max-w-6xl mx-auto space-y-12">
            <header>
                <h1 class="text-3xl font-black tracking-tight text-slate-100">"Hooks Library"</h1>
                <p class="text-slate-400 mt-1">"A collection of reusable reactive primitives for Leptos."</p>
            </header>

            <section class="space-y-8">
                <h2 class="text-xl font-bold text-indigo-400 border-b border-indigo-500/20 pb-2">"State & Logic"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Showcase label="use_toggle" source="pub fn use_toggle(initial: bool) -> (ReadSignal<bool>, impl Fn()) { ... }">
                        {move || {
                            let (val, toggle) = use_toggle(false);
                            view! {
                                <div class="flex items-center gap-3">
                                    <button 
                                        class=move || format!("px-4 py-2 rounded-lg text-sm font-medium transition-colors {}", 
                                            if val.get() { "bg-indigo-500 text-white" } else { "bg-slate-700 text-slate-300" })
                                        on:click=toggle
                                    >
                                        {move || if val.get() { "ON" } else { "OFF" }}
                                    </button>
                                    <span class="text-sm text-slate-400">"State: " {move || if val.get() { "Active" } else { "Inactive" }}</span>
                                </div>
                            }.into_any()
                        }}
                    </Showcase>
                    <Showcase label="use_counter_with_step" source="pub fn use_counter_with_step<T>(initial: T, step: T) -> (ReadSignal<T>, impl Fn(), impl Fn(), impl Fn(T)) { ... }">
                        {move || {
                            let (count, inc, dec, _) = use_counter_with_step(0, 1);
                            view! {
                                <div class="flex items-center gap-3">
                                    <button class="px-3 py-1 bg-slate-700 rounded" on:click=dec>"-"</button>
                                    <span class="font-mono text-lg w-12 text-center">{move || count.get()}</span>
                                    <button class="px-3 py-1 bg-slate-700 rounded" on:click=inc>"+"</button>
                                </div>
                            }.into_any()
                        }}
                    </Showcase>
                </div>
            </section>

            <section class="space-y-8">
                <h2 class="text-xl font-bold text-indigo-400 border-b border-indigo-500/20 pb-2">"Browser & Window"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Showcase label="use_window_size" source="pub fn use_window_size() -> ReadSignal<WindowSize> { ... }">
                        {move || {
                            let size = use_window_size();
                            view! {
                                <div class="text-sm font-mono text-slate-300">
                                    "Size: " {move || format!("{} x {}", size.get().width, size.get().height)}
                                </div>
                            }.into_any()
                        }}
                    </Showcase>
                    <Showcase label="use_mouse_position" source="pub fn use_mouse_position() -> ReadSignal<MousePosition> { ... }">
                        {move || {
                            let pos = use_mouse_position();
                            view! {
                                <div class="text-sm font-mono text-slate-300">
                                    "Mouse: " {move || format!("x: {}, y: {}", pos.get().x, pos.get().y)}
                                </div>
                            }.into_any()
                        }}
                    </Showcase>
                    <Showcase label="use_media_query" source="pub fn use_media_query(query: &'static str) -> ReadSignal<bool> { ... }">
                        {move || {
                            let is_mobile = use_media_query("(max-width: 768px)");
                            view! {
                                <div class="flex items-center gap-2">
                                    <span class="text-sm">"Mobile View: "</span>
                                    <span class=move || if is_mobile.get() { "text-emerald-400" } else { "text-slate-400" }>
                                        {move || if is_mobile.get() { "Yes" } else { "No" }}
                                    </span>
                                </div>
                            }.into_any()
                        }}
                    </Showcase>
                    <Showcase label="use_online_status" source="pub fn use_online_status() -> ReadSignal<bool> { ... }">
                        {move || {
                            let online = use_online_status();
                            view! {
                                <div class="flex items-center gap-2">
                                    <span class="text-sm">"Network: "</span>
                                    <span class=move || if online.get() { "text-emerald-400" } else { "text-red-400" }>
                                        {move || if online.get() { "Online" } else { "Offline" }}
                                    </span>
                                </div>
                            }.into_any()
                        }}
                    </Showcase>
                </div>
            </section>

            <section class="space-y-8">
                <h2 class="text-xl font-bold text-indigo-400 border-b border-indigo-500/20 pb-2">"DOM & Interaction"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Showcase label="use_click_outside" source="pub fn use_click_outside<F>(target: StoredNode<Div>, callback: F) { ... }">
                        {move || {
                            let div_ref = create_node_ref::<leptos::html::Div>();
                            let (clicked_out, set_clicked_out) = signal(false);
                            
                            use_click_outside(div_ref, move || set_clicked_out.set(true));

                            view! {
                                <div node_ref=div_ref class="p-4 bg-slate-700 rounded-lg text-sm text-center">
                                    "Click outside me!"
                                    <div class="mt-2 text-xs text-indigo-300">
                                        {move || if clicked_out.get() { "Clicked Outside!" } else { "Waiting..." }}
                                    </div>
                                </div>
                            }.into_any()
                        }}
                    </Showcase>
                    <Showcase label="use_keyboard_shortcut" source="pub fn use_keyboard_shortcut<F>(shortcut: Shortcut, callback: F) { ... }">
                        {move || {
                            let (status, set_status) = signal("Press Ctrl+S".to_string());
                            
                            use_keyboard_shortcut(Shortcut {
                                key: "s".to_string(), ctrl: true, alt: false, shift: false, meta: false
                            }, move || set_status.set("Shortcut Triggered!".to_string()));

                            view! {
                                <div class="p-4 bg-slate-700 rounded-lg text-sm text-center">
                                    {move || status.get()}
                                </div>
                            }.into_any()
                        }}
                    </Showcase>
                </div>
            </section>
        </div>
    }.into_any()
}
