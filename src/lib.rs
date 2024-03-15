use tracing::info;
use web_sys::HtmlVideoElement;

pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream { el }
    }

    pub fn set_media_src(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let devices = navigator.media_devices().expect("no media devices");

        info!("devices [from tracing_wasm]: {:?}", devices);

        web_sys::console::log_1(&devices);
    }
}
