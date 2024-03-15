use sycamore::prelude::*;
use webcam::VideoStream;

#[component]
fn App<G: Html>() -> View<G> {
    view! {
        div(class="container p-2") {
            Video()
        }
    }
}

#[component]
fn Video<G: Html>() -> View<G> {
    let video_ref = create_node_ref();
    create_effect(move || {
        if let Some(video) = video_ref.try_get::<DomNode>().map(|v| v.unchecked_into()) {
            let video_stream = VideoStream::new(video);
            video_stream.set_media_src();
        }
    });
    view! {
        video(
            ref=video_ref,
            class="border border-gray-300 rounded-md",
            width="1280",
            controls=true,
            autoplay=true,
            src="http://127.0.0.1:8888/video/video.mp4",
            ) {}
    }
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(App);
}
