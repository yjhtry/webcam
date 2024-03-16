mod controls;
mod video;

pub use controls::Controls;
pub use video::*;

use sycamore::prelude::*;

use crate::AppState;

#[component]
pub async fn App<G: Html>() -> View<G> {
    let state = AppState::new().await;
    provide_context(state);

    view! {
        div(class="container p-2") {
            Video()
        }
    }
}
