use leptos::prelude::*;

pub mod storage;
pub mod window;
pub mod mouse;
pub mod timer;
pub mod media;
pub mod dom;
pub mod keyboard;
pub mod state;
pub mod url;
pub mod observer;
pub mod browser;
pub mod idle;

pub use storage::*;
pub use window::use_window_size;
pub use mouse::use_mouse_position;
pub use timer::*;
pub use media::use_media_query;
pub use dom::use_click_outside;
pub use keyboard::{use_keyboard_shortcut, Shortcut};
pub use state::{use_toggle, use_counter, use_counter_with_step};
pub use url::*;
pub use observer::*;
pub use browser::{use_online_status, use_dark_mode};
pub use idle::*;

pub type StoredNode<T> = NodeRef<T>;
