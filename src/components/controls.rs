use sycamore::prelude::*;
use web_sys::Event;

use crate::{AppState, Device};

#[component]
pub fn Controls<G: Html>() -> View<G> {
    let app_state = use_context::<AppState>();
    let devices = app_state.devices.get_clone();
    let video_devices: Signal<Vec<Device>> =
        create_signal(devices.get_video_devices().cloned().collect());
    let video_devices = *video_devices;

    view! {
        div(class="absolute bottom-3 right-4 flex gap-6") {
            select(
                class="form-select px-4 py-3 rounded-full w-64",
                aria-label="Default select example",
                bind:value=app_state.device_id,
                // on:change=|e: Event| {
                //     let target = e.target().unwrap();
                //     let value = target.to_string();
                //     app_state.device_id.set(value.into());
                // }
            ) {
                Keyed(
                    iterable=video_devices,
                    view=|x| view! {
                        option(value=x.id) { (x.label) }
                    },
                    key=|x| x.id.clone(),
                )
            }
        }
    }
}
