pub mod ascii_art;
pub mod device;
pub mod vendor;

use ash::*;
use device::Device;
use std::str;
use vendor::Vendor;

pub fn fetch_device(instance: &Instance, device: vk::PhysicalDevice) -> bool {
    let properties = unsafe { instance.get_physical_device_properties(device) };
    let mut properties2 = vk::PhysicalDeviceProperties2::default();
    unsafe { instance.get_physical_device_properties2(device, &mut properties2) }

    // println!("device raw: {}", device.as_raw());

    let vendor = Vendor::from_vendor_id(properties.vendor_id)
        .expect(&format!("unknown vendor: {}", properties.vendor_id));

    let art = vendor.get_ascii_art();

    let device = Device::new(instance, device);

    let info = get_device_info(device);

    // iterate over art or info whichever is longer
    let empty = "".to_string();
    for i in 0..art.len().max(info.len()) {
        let art_line = art.get(i).unwrap_or(&empty);
        let info_line = info.get(i).unwrap_or(&empty);
        println!(" {} {}", art_line, info_line);
    }

    // println!(
    //     "Device ID{}: Device Type {}",
    //     properties.device_id,
    //     properties.device_type.as_raw()
    // );

    true
}

const BOLD: &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";
const ALIGNMENT: &str = "    ";

fn get_device_info(device: Device) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    output.push(format!(
        "{}{} : {}{}",
        BOLD,
        device.device_name,
        RESET,
        device.device_type.name()
    ));
    /* 	Fetch.push_back(fmt::format(
        "    Device: \033[37m{:04x}\033[0m : \033[37m{:04x}\033[0m ({})",
        DeviceProperties.properties.deviceID,
        DeviceProperties.properties.vendorID,
        Vulkan::Util::VendorName(static_cast<Vulkan::Util::VendorID>(
            DeviceProperties.properties.vendorID
        ))
    )); */
    output.push(format!(
        "{}Device: {} : {} ({})",
        ALIGNMENT,
        device.device_id,
        device.vendor_id,
        device.vendor.name(),
    ));

    output.push(format!("{}Driver: {} : {} {}", ALIGNMENT, "", "", ""));

    output.push(format!("{}API: {}", ALIGNMENT, device.api_version,));

    output
}

pub fn iterate_devices() {
    /*
    #ifdef _WIN32
    #define NOMINMAX
    #include <Windows.h>
    // Statically enables "ENABLE_VIRTUAL_TERMINAL_PROCESSING" for the terminal
    // at runtime to allow for unix-style escape sequences.
    static const bool _WndV100Enabled = []() -> bool {
        const auto Handle = GetStdHandle(STD_OUTPUT_HANDLE);
        DWORD      ConsoleMode;
        GetConsoleMode(Handle, &ConsoleMode);
        SetConsoleMode(Handle, ConsoleMode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        GetConsoleMode(Handle, &ConsoleMode);
        return ConsoleMode & ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    }();
    #endif
     */
    let entry = unsafe { Entry::load().unwrap() };
    let create_info = vk::InstanceCreateInfo::default();
    let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    for device in devices {
        fetch_device(&instance, device);
    }
}
