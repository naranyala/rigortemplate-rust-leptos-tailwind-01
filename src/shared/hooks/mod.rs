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
pub use window::*;
pub use mouse::*;
pub use timer::*;
pub use media::*;
pub use dom::*;
pub use keyboard::*;
pub use state::*;
pub use url::*;
pub use observer::*;
pub use browser::*;
pub use idle::*;

pub type StoredNode<T> = NodeRef<T>;
