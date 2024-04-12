pub mod ascii_art;
pub mod vendor;

use ash::{vk::PhysicalDeviceProperties, *};
use std::str;
use vendor::{device_type_to_name, Vendor};

pub fn fetch_device(instance: &ash::Instance, device: ash::vk::PhysicalDevice) -> bool {
    let properties = unsafe { instance.get_physical_device_properties(device) };

    let vendor = Vendor::from_vendor_id(properties.vendor_id)
        .expect(&format!("unknown vendor: {}", properties.vendor_id));

    let art = vendor.get_ascii_art();

    let info = get_device_info(&instance, device, properties);

    // iterate over art or info whichever is longer
    let empty = "".to_string();
    for i in 0..art.len().max(info.len()) {
        let art_line = art.get(i).unwrap_or(&empty);
        let info_line = info.get(i).unwrap_or(&empty);
        println!("{} {}", art_line, info_line);
    }

    // println!("{}: {}", properties.vendor_id, properties.device_id);

    true
}

const BOLD: &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";

fn get_device_info(
    _instance: &&Instance,
    _device: vk::PhysicalDevice,
    properties: PhysicalDeviceProperties,
) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    output.push(format!(
        " {}{} : {}{}",
        BOLD,
        &properties.device_name_as_c_str().unwrap().to_str().unwrap(),
        RESET,
        device_type_to_name(properties.device_type.as_raw())
    ));

    output
}

pub fn iterate_devices() {
    let entry = unsafe { ash::Entry::load().unwrap() };
    let create_info = vk::InstanceCreateInfo::default();
    let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    for device in devices {
        fetch_device(&instance, device);
    }
}
