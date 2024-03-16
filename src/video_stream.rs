use js_sys::wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlVideoElement, MediaStreamConstraints};

use crate::Devices;

pub struct VideoStream {
    pub el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream { el }
    }

    pub async fn set_media_src(&self, video_constraints: serde_json::Value) {
        let devices = Devices::get_media_devices();
        let mut constraints = MediaStreamConstraints::new();

        constraints.video(&serde_wasm_bindgen::to_value(&video_constraints).unwrap());
        constraints.audio(&false.into());

        let media = JsFuture::from(
            devices
                .get_user_media_with_constraints(&constraints)
                .expect("no media devices"),
        )
        .await
        .unwrap();

        let media_stream = media.unchecked_into::<web_sys::MediaStream>();

        self.el.set_src_object(Some(&media_stream));
    }
}
