use leptos::prelude::*;
use crate::shared::hooks::*;

#[component]
pub fn HookToggle() -> AnyView {
    let (val, toggle) = use_toggle(false);
    view! {
        <div class="flex items-center gap-3">
            <button 
                class=move || if val.get() { "px-4 py-2 rounded-lg text-sm font-medium transition-colors bg-accent text-white" } else { "px-4 py-2 rounded-lg text-sm font-medium transition-colors bg-muted text-body" }
                on:click=move |_| toggle()
            >
                {move || if val.get() { "ON" } else { "OFF" }}
            </button>
            <span class="text-sm text-label">"State: " {move || if val.get() { "Active" } else { "Inactive" }}</span>
        </div>
    }.into_any()
}

#[component]
pub fn HookCounter() -> AnyView {
    let (count, inc, dec, _) = use_counter_with_step(0, 1);
    view! {
        <div class="flex items-center gap-3">
            <button class="px-3 py-1 rounded bg-muted" on:click=move |_| dec()>"-"</button>
            <span class="font-mono text-lg w-12 text-center text-heading">{move || count.get()}</span>
            <button class="px-3 py-1 rounded bg-muted" on:click=move |_| inc()>"+"</button>
        </div>
    }.into_any()
}

#[component]
pub fn HookWindowSize() -> AnyView {
    let size = use_window_size();
    view! {
        <div class="text-sm font-mono text-body">
            "Size: " {move || format!("{} x {}", size.get().width, size.get().height)}
        </div>
    }.into_any()
}

#[component]
pub fn HookMousePosition() -> AnyView {
    let pos = use_mouse_position();
    view! {
        <div class="text-sm font-mono text-body">
            "Mouse: " {move || format!("x: {}, y: {}", pos.get().x, pos.get().y)}
        </div>
    }.into_any()
}

#[component]
pub fn HookMediaQuery() -> AnyView {
    let is_mobile = use_media_query("(max-width: 768px)");
    view! {
        <div class="flex items-center gap-2">
            <span class="text-sm text-body">"Mobile View: "</span>
            <span class=move || if is_mobile.get() { "text-emerald-500" } else { "text-label" }>
                {move || if is_mobile.get() { "Yes" } else { "No" }}
            </span>
        </div>
    }.into_any()
}

#[component]
pub fn HookOnlineStatus() -> AnyView {
    let online = use_online_status();
    view! {
        <div class="flex items-center gap-2">
            <span class="text-sm text-body">"Network: "</span>
            <span class=move || if online.get() { "text-emerald-500" } else { "text-red-500" }>
                {move || if online.get() { "Online" } else { "Offline" }}
            </span>
        </div>
    }.into_any()
}

#[component]
pub fn HookClickOutside() -> AnyView {
    let div_ref = NodeRef::<leptos::html::Div>::new();
    let (clicked_out, set_clicked_out) = signal(false);

    use_click_outside(div_ref, move || set_clicked_out.set(true));

    view! {
        <div node_ref=div_ref class="p-4 rounded-lg text-sm text-center bg-muted text-body">
            "Click outside me!"
            <div class="mt-2 text-xs text-accent-text">
                {move || if clicked_out.get() { "Clicked Outside!" } else { "Waiting..." }}
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn HookKeyboardShortcut() -> AnyView {
    let (status, set_status) = signal("Press Ctrl+S".to_string());

    use_keyboard_shortcut(Shortcut {
        key: "s".to_string(), ctrl: true, alt: false, shift: false, meta: false
    }, move || set_status.set("Shortcut Triggered!".to_string()));

    view! {
        <div class="p-4 rounded-lg text-sm text-center bg-muted text-body">
            {move || status.get()}
        </div>
    }.into_any()
}
