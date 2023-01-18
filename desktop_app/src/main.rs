use cpal::{traits::{HostTrait, DeviceTrait}};

fn main() {
    let host = cpal::default_host();
    for device in host.devices().unwrap() {
        device.name().map(|name| println!("Device name: {}", name)).expect("Device name not found");
    }
}
