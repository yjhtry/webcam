use serde_json::json;
use sycamore::prelude::*;

use crate::{AppState, Controls, Devices, VideoStream};

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
                    "width": app_state.get_width(),
                    "height": app_state.get_height(),
                },
            }),
            id => json!({
                "audio": false,
                "video": {
                    "deviceId": {
                        "exact": id,
                    },
                    "width": app_state.get_width(),
                    "height": app_state.get_height(),
                },
            }),
        };
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(video) = video_ref.try_get::<DomNode>().map(|v| v.unchecked_into()) {
                let video_stream = VideoStream::new(video);
                video_stream.set_media_src(constraints).await;
            }

            Devices::load().await;
        })
    });
    view! {
        div(class="relative w-[640px]") {
            video(
                ref=video_ref,
                class="border border-gray-300 rounded-md",
                width=app_state.get_width(),
                height=app_state.get_height(),
                autoplay=false,
                // src="http://127.0.0.1:8888/video/video.mp4",
                ) {}

            Controls()
        }
    }
}
