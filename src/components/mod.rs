mod controls;
mod video;

pub use controls::Controls;
use js_sys::wasm_bindgen::{closure::Closure, JsCast};
pub use video::*;

use sycamore::prelude::*;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;

use crate::{get_screen_dimensions, AppState};

#[component]
pub async fn App<G: Html>() -> View<G> {
    let state = AppState::new().await;
    state.set_dimensions(get_screen_dimensions());

    provide_context(state);

    let state = use_context::<AppState>();

    window_event_listener(
        "resize",
        Box::new(move |_| {
            state.dimensions.set(get_screen_dimensions());
        }),
    );

    view! {
        div(class="p-2") {
            Video()
        }
    }
}

fn window_event_listener<'a>(event: &str, callback: Box<dyn Fn(Event) + 'a>) {
    let window = web_sys::window().unwrap();
    let callback: Box<dyn Fn(Event) + 'static> = unsafe { std::mem::transmute(callback) };

    let closure = Closure::wrap(callback);

    window
        .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
        .unwrap_throw();

    on_cleanup(move || drop(closure));
}
