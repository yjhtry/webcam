mod components;
mod devices;
mod video_stream;

pub use components::*;
pub use devices::*;
use sycamore::reactive::{create_signal, Signal};
pub use video_stream::*;

#[derive(Debug, Clone, Copy)]
pub struct AppState {
    pub dimensions: Signal<(u32, u32)>,
    pub device_id: Signal<String>,
    pub devices: Signal<Devices>,
}

impl AppState {
    pub async fn new() -> AppState {
        AppState {
            dimensions: create_signal((640, 400)),
            device_id: create_signal("".into()),
            devices: create_signal(Devices::load().await),
        }
    }

    pub fn set_dimensions(&self, (width, height): (u32, u32)) {
        self.dimensions.set((width, height));
    }

    pub fn get_width(&self) -> u32 {
        self.dimensions.get().0
    }

    pub fn get_height(&self) -> u32 {
        self.dimensions.get().1
    }
}

pub fn get_screen_dimensions() -> (u32, u32) {
    let window = web_sys::window().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
    (width, height)
}
