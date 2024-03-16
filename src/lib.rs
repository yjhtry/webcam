mod components;
mod devices;
mod video_stream;

pub use components::*;
pub use devices::*;
use sycamore::reactive::{create_signal, Signal};
pub use video_stream::*;

#[derive(Debug, Clone)]
pub struct AppState {
    pub device_id: Signal<String>,
    pub devices: Signal<Devices>,
}

impl AppState {
    pub async fn new() -> AppState {
        let device_id = create_signal("".into());
        let devices = create_signal(Devices::load().await);
        AppState {
            device_id: device_id,
            devices: devices,
        }
    }
}
