use serde_json::json;
use sycamore::prelude::*;
use tracing::info;

use crate::{Controls, Devices, VideoStream};

#[component]
pub fn Video<G: Html>() -> View<G> {
    let video_ref = create_node_ref();
    create_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(video) = video_ref.try_get::<DomNode>().map(|v| v.unchecked_into()) {
                let video_stream = VideoStream::new(video);
                video_stream
                    .set_media_src(json!({
                        "audio": false,
                        "video": {
                            "facingMode": "environment",
                            "width": 640,
                            "height": 480,
                        },
                    }))
                    .await;
            }

            let devices = Devices::load().await;

            info!("{:?}", devices);
        })
    });
    view! {
        div(class="relative w-[640px]") {
            video(
                ref=video_ref,
                class="border border-gray-300 rounded-md",
                width="640",
                width="480",
                autoplay=false,
                // src="http://127.0.0.1:8888/video/video.mp4",
                ) {}

            Controls()
        }
    }
}
