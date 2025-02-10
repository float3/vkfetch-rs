pub mod ascii_art;
pub mod device;
pub mod vendor;

use ash::{self, vk, Entry, Instance};
use device::PhysicalDevice;
use vendor::Vendor;

const BOLD: &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";
const ALIGNMENT: &str = "    ";
const EMPTY: &str = "";

pub fn fetch_device(instance: &Instance, device_handle: vk::PhysicalDevice) {
    let properties = unsafe { instance.get_physical_device_properties(device_handle) };
    let mut properties2 = vk::PhysicalDeviceProperties2::default();
    unsafe {
        instance.get_physical_device_properties2(device_handle, &mut properties2);
    }

    let vendor = Vendor::from_vendor_id(properties.vendor_id)
        .unwrap_or_else(|| panic!("unknown vendor: {}", properties.vendor_id));
    let art = vendor.get_ascii_art();

    let device = PhysicalDevice::new(instance, device_handle);
    let info = get_device_info(device, vendor.get_styles()[0]);

    for i in 0..art.len().max(info.len()) {
        let art_line = art.get(i).map(String::as_str).unwrap_or(EMPTY);
        let info_line = info.get(i).map(String::as_str).unwrap_or(EMPTY);
        println!(" {} {}", art_line, info_line);
    }

    println!();
}

fn get_device_info(device: PhysicalDevice, color: &str) -> Vec<String> {
    let title = format!(
        "{}{}{}{}: {}",
        BOLD,
        color,
        device.device_name,
        RESET,
        device.device_type.name()
    );
    let underline_len = device.device_name.len() + device.device_type.name().len() + 3;
    let underline = "=".repeat(underline_len);

    // Assume meter_width is defined (e.g. 30)
    let meter_width = 30;
    let filled = (device.characteristics.memory_pressure * meter_width as f32).round() as usize;

    vec![
        title,
        format!("{}{}{}", BOLD, color, underline),
        format!(
            "{}{}Device{}: 0x{:X} : 0x{:X} ({})",
            ALIGNMENT,
            color,
            RESET,
            device.device_id,
            device.vendor_id,
            device.vendor.name(),
        ),
        format!(
            "{}{}Driver{}: {} : {}",
            ALIGNMENT, color, RESET, device.driver_name, device.driver_info
        ),
        format!("{}{}API{}: {}", ALIGNMENT, color, RESET, device.api_version),
        format!(
            "{}{}VRAM{}: {}{}{} / {}",
            ALIGNMENT,
            color,
            RESET,
            color,
            format_bytes(device.heapbudget),
            RESET,
            format_bytes(device.heapsize)
        ),
        format!(
            "{}[{}{}{}{}] % {}{:.2}{}",
            ALIGNMENT,
            color,
            "|".repeat(filled),
            RESET,
            " ".repeat(meter_width - filled),
            color,
            device.characteristics.memory_pressure * 100.0,
            RESET
        ),
        format!(
            "{}{}Streaming Multiprocessors{}: {}",
            ALIGNMENT,
            color,
            RESET,
            device
                .characteristics
                .streaming_multiprocessors
                .map_or("N/A".to_string(), |v| v.to_string())
        ),
        format!(
            "{}{}Warps per SM{}: {}",
            ALIGNMENT,
            color,
            RESET,
            device
                .characteristics
                .warps_per_sm
                .map_or("N/A".to_string(), |v| v.to_string())
        ),
    ]
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;
    let bytes_f64 = bytes as f64;
    if bytes_f64 >= TB {
        format!("{:.3} TB", bytes_f64 / TB)
    } else if bytes_f64 >= GB {
        format!("{:.3} GB", bytes_f64 / GB)
    } else if bytes_f64 >= MB {
        format!("{:.3} MB", bytes_f64 / MB)
    } else if bytes_f64 >= KB {
        format!("{:.3} KB", bytes_f64 / KB)
    } else {
        format!("{} B", bytes)
    }
}

pub fn iterate_devices() {
    let entry = {
        #[cfg(not(feature = "loaded"))]
        {
            Entry::linked()
        }
        #[cfg(feature = "loaded")]
        {
            match unsafe { Entry::load() } {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("Failed to load entry: {:?}", e);
                    return;
                }
            }
        }
    };

    for api_version in [
        vk::API_VERSION_1_3,
        vk::API_VERSION_1_2,
        vk::API_VERSION_1_1,
        vk::API_VERSION_1_0,
    ] {
        let app_info = vk::ApplicationInfo {
            api_version,
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);

        let instance = match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => instance,
            Err(e) => {
                eprintln!("Failed to create instance: {:?}", e);
                continue;
            }
        };

        match unsafe { instance.enumerate_physical_devices() } {
            Ok(devices) => {
                for device in devices {
                    fetch_device(&instance, device);
                }
            }
            Err(e) => {
                eprintln!("Failed to enumerate physical devices: {:?}", e);
            }
        }
        break;
    }
}
