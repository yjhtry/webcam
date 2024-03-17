use serde_json::json;
use sycamore::prelude::*;

use crate::{AppState, Controls, VideoStream};

#[component]
pub fn Video<G: Html>() -> View<G> {
    let video_ref = create_node_ref();
    let app_state = use_context::<AppState>();

    create_effect(move || {
        let constraints = match app_state.device_id.get_clone().as_str() {
            "" => json!({
                "audio": false,
                "video": {
                    "facingMode": "environment",
                },
            }),
            id => json!({
                "audio": false,
                "video": {
                    "deviceId": {
                        "exact": id,
                    },
                },
            }),
        };
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(video) = video_ref.try_get::<DomNode>().map(|v| v.unchecked_into()) {
                let video_stream = VideoStream::new(video);
                video_stream.set_media_src(constraints).await;
            }
        })
    });

    view! {
        div(class="relative group") {
            video(
                ref=video_ref,
                class="border border-gray-300 rounded-md",
                width=app_state.get_width(),
                height=app_state.get_height(),
                autoplay=false,
                // style=format!("width: {}px; height: {}px; object-fit: fill", app_state.get_width(), app_state.get_height())
                ) {}

            Controls()
        }
    }
}
