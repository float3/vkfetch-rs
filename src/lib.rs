#![allow(dead_code)]
pub mod ascii_art;
pub mod device;
pub mod vendor;

use ash::*;
use device::PhysicalDevice;
use std::str;
use vendor::Vendor;

pub fn fetch_device(instance: &Instance, device: vk::PhysicalDevice) {
    let properties = unsafe { instance.get_physical_device_properties(device) };
    let mut properties2 = vk::PhysicalDeviceProperties2::default();
    unsafe { instance.get_physical_device_properties2(device, &mut properties2) }

    let vendor = Vendor::from_vendor_id(properties.vendor_id)
        .unwrap_or_else(|| panic!("unknown vendor: {}", properties.vendor_id));

    let art = vendor.get_ascii_art();

    let device = PhysicalDevice::new(instance, device);

    let info = get_device_info(device);

    let empty = "".to_string();
    for i in 0..art.len().max(info.len()) {
        let art_line = art.get(i).unwrap_or(&empty);
        let info_line = info.get(i).unwrap_or(&empty);
        println!(" {} {}", art_line, info_line);
    }

    println!();
}

const BOLD: &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";
const ALIGNMENT: &str = "    ";

fn get_device_info(device: PhysicalDevice) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    output.push(format!(
        "{}{} : {}{}",
        BOLD,
        device.device_name,
        RESET,
        device.device_type.name()
    ));

    output.push(format!(
        "{}Device: 0x{:X} : 0x{:X} ({})",
        ALIGNMENT,
        device.device_id,
        device.vendor_id,
        device.vendor.name(),
    ));

    output.push(format!(
        "{}Driver: {} : {}",
        ALIGNMENT, device.driver_name, device.driver_info
    ));

    output.push(format!("{}API: {}", ALIGNMENT, device.api_version,));

    output
}

pub fn iterate_devices() {
    let entry = Entry::linked();

    let versions = [
        vk::API_VERSION_1_3,
        vk::API_VERSION_1_2,
        vk::API_VERSION_1_1,
        vk::API_VERSION_1_0,
    ];

    for api_version in versions {
        let app_info = vk::ApplicationInfo {
            api_version,
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);
        let instance_result = unsafe { entry.create_instance(&create_info, None) };
        match instance_result {
            Ok(instance) => {
                let devices_result = unsafe { instance.enumerate_physical_devices() };
                match devices_result {
                    Ok(devices) => {
                        devices.into_iter().for_each(|device| {
                            fetch_device(&instance, device);
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to enumerate physical devices: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to create instance: {:?}", e);
                continue;
            }
        }
        break;
    }
}
