use std::ops::Deref;

use js_sys::wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaDeviceKind, MediaDevices};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Devices(Vec<Device>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashMediaDeviceKind {
    Audioinput = 0,
    Audiooutput = 1,
    Videoinput = 2,
    // some variants omitted
}

impl From<MediaDeviceKind> for HashMediaDeviceKind {
    fn from(kind: MediaDeviceKind) -> Self {
        match kind {
            MediaDeviceKind::Audioinput => HashMediaDeviceKind::Audioinput,
            MediaDeviceKind::Audiooutput => HashMediaDeviceKind::Audiooutput,
            MediaDeviceKind::Videoinput => HashMediaDeviceKind::Videoinput,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Device {
    pub label: String,
    pub id: String,
    pub kind: HashMediaDeviceKind,
}

impl Devices {
    pub async fn load() -> Self {
        let devices = Self::get_media_devices();
        let all_devices = JsFuture::from(devices.enumerate_devices().unwrap())
            .await
            .unwrap();

        web_sys::console::log_1(&all_devices);

        Devices::from(&all_devices)
    }

    pub fn get_video_devices(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(HashMediaDeviceKind::Videoinput)
    }

    pub fn get_audio_devices(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(HashMediaDeviceKind::Audioinput)
    }

    pub fn get_media_devices() -> MediaDevices {
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        navigator.media_devices().expect("no media devices")
    }

    fn iter_by_kind(&self, kind: HashMediaDeviceKind) -> impl Iterator<Item = &Device> {
        self.iter().filter(move |device| device.kind == kind)
    }
}

impl Deref for Devices {
    type Target = Vec<Device>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&JsValue> for Devices {
    fn from(value: &JsValue) -> Self {
        match js_sys::try_iter(value) {
            Ok(Some(iter)) => {
                let devices = iter
                    .filter(|x| x.is_ok())
                    .map(|x| x.unwrap().into())
                    .collect::<Vec<Device>>();
                Devices(devices)
            }
            _ => Devices::default(),
        }
    }
}

impl From<JsValue> for Device {
    fn from(value: JsValue) -> Self {
        let device = value.unchecked_into::<web_sys::MediaDeviceInfo>();
        let kind = device.kind().into();
        let label = device.label();
        let id = device.device_id();
        Device { label, id, kind }
    }
}
