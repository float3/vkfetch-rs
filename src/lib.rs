pub mod ascii_art;
pub mod device;
pub mod vendor;

use ash::{self, vk, Entry, Instance};
use device::PhysicalDevice;
use std::error::Error;
use vendor::Vendor;

const BOLD: &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";
const ALIGNMENT: &str = "    ";
const EMPTY: &str = "";

/// Fetches and prints information for a given physical device.
pub fn fetch_device(
    instance: &Instance,
    device_handle: vk::PhysicalDevice,
) -> Result<(), Box<dyn Error>> {
    let properties = unsafe { instance.get_physical_device_properties(device_handle) };
    let mut properties2 = vk::PhysicalDeviceProperties2::default();
    unsafe {
        instance.get_physical_device_properties2(device_handle, &mut properties2);
    }

    let vendor = Vendor::from_vendor_id(properties.vendor_id)
        .unwrap_or_else(|| panic!("unknown vendor: {}", properties.vendor_id));
    let art = vendor.get_ascii_art();

    let device = PhysicalDevice::new(instance, device_handle);
    let info = get_device_info(&device, vendor.get_styles()[0]);

    let x = art.get(0).unwrap().len();
    let empty = " ".repeat(x);

    for i in 0..art.len().max(info.len()) {
        let art_line = art.get(i).map(String::as_str).unwrap_or(&empty);
        let info_line = info.get(i).map(String::as_str).unwrap_or(EMPTY);
        println!(" {} {}", art_line, info_line);
    }

    println!();
    Ok(())
}

/// Returns a vector of formatted strings representing the device info,
/// including extra vendor-specific and general device limits.
/// Lines for optional fields are only included if available.
fn get_device_info(device: &PhysicalDevice, color: &str) -> Vec<String> {
    let mut lines = Vec::new();

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

    let meter_width = 30;
    let filled = (device.characteristics.memory_pressure * meter_width as f32).round() as usize;

    // Basic device info.
    lines.push(title);
    lines.push(format!("{}{}{}", BOLD, color, underline));
    lines.push(format!(
        "{}{}Device{}: 0x{:X} : 0x{:X} ({})",
        ALIGNMENT,
        color,
        RESET,
        device.device_id,
        device.vendor_id,
        device.vendor.name(),
    ));
    lines.push(format!(
        "{}{}Driver{}: {} : {}",
        ALIGNMENT, color, RESET, device.driver_name, device.driver_info
    ));
    lines.push(format!(
        "{}{}API{}: {}",
        ALIGNMENT, color, RESET, device.api_version
    ));
    lines.push(format!(
        "{}{}VRAM{}: {}{}{} / {}",
        ALIGNMENT,
        color,
        RESET,
        color,
        format_bytes(device.heapbudget),
        RESET,
        format_bytes(device.heapsize)
    ));
    lines.push(format!(
        "{}[{}{}{}{}] % {}{:.2}{}",
        ALIGNMENT,
        color,
        "|".repeat(filled),
        RESET,
        " ".repeat(meter_width - filled),
        color,
        device.characteristics.memory_pressure * 100.0,
        RESET
    ));

    // Vendor-specific extra info.
    if let Some(cu) = device.characteristics.compute_units {
        lines.push(format!(
            "{}{}Compute Units{}: {}",
            ALIGNMENT, color, RESET, cu
        ));
    }
    if let Some(se) = device.characteristics.shader_engines {
        lines.push(format!(
            "{}{}Shader Engines{}: {}",
            ALIGNMENT, color, RESET, se
        ));
    }
    if let Some(sapec) = device.characteristics.shader_arrays_per_engine_count {
        lines.push(format!(
            "{}{}Shader Arrays per Engine{}: {}",
            ALIGNMENT, color, RESET, sapec
        ));
    }
    if let Some(cups) = device.characteristics.compute_units_per_shader_array {
        lines.push(format!(
            "{}{}Compute Units per Shader Array{}: {}",
            ALIGNMENT, color, RESET, cups
        ));
    }
    if let Some(simd) = device.characteristics.simd_per_compute_unit {
        lines.push(format!(
            "{}{}SIMD per Compute Unit{}: {}",
            ALIGNMENT, color, RESET, simd
        ));
    }
    if let Some(wfs) = device.characteristics.wavefronts_per_simd {
        lines.push(format!(
            "{}{}Wavefronts per SIMD{}: {}",
            ALIGNMENT, color, RESET, wfs
        ));
    }
    if let Some(wfsz) = device.characteristics.wavefront_size {
        lines.push(format!(
            "{}{}Wavefront Size{}: {}",
            ALIGNMENT, color, RESET, wfsz
        ));
    }
    if let Some(sm) = device.characteristics.streaming_multiprocessors {
        lines.push(format!(
            "{}{}Streaming Multiprocessors{}: {}",
            ALIGNMENT, color, RESET, sm
        ));
    }
    if let Some(wps) = device.characteristics.warps_per_sm {
        lines.push(format!(
            "{}{}Warps per SM{}: {}",
            ALIGNMENT, color, RESET, wps
        ));
    }

    // General device limits.
    // lines.push(format!(
    //     "{}{}Max Image Dimension 2D{}: {}",
    //     ALIGNMENT,
    //     color,
    //     RESET,
    //     format_bytes(device.characteristics.max_image_dimension_2d.into())
    // ));
    lines.push(format!(
        "{}{}Max Compute Shared Memory Size{}: {}",
        ALIGNMENT,
        color,
        RESET,
        format_bytes(device.characteristics.max_compute_shared_memory_size.into())
    ));
    lines.push(format!(
        "{}{}Max Compute Work Group Invocations{}: {}",
        ALIGNMENT,
        color,
        RESET,
        format_bytes(
            device
                .characteristics
                .max_compute_work_group_invocations
                .into()
        )
    ));
    lines.push(format!(
        "{} | {} | {}",
        format!(
            "{}{}Raytracing{}: {}",
            ALIGNMENT,
            color,
            RESET,
            if device.characteristics.supports_ray_tracing {
                "[x]"
            } else {
                "[ ]"
            },
        ),
        format!(
            "{}Dedicated Transfer Queue{}: {}",
            color,
            RESET,
            if device.characteristics.supports_ray_tracing {
                "[x]"
            } else {
                "[ ]"
            },
        ),
        format!(
            "{}Dedicated Async Compute Queue{}: {}",
            color,
            RESET,
            if device.characteristics.supports_ray_tracing {
                "[x]"
            } else {
                "[ ]"
            },
        ),
    ));

    lines
}

/// Converts a byte count into a human‐readable string with up to TB precision.
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

/// Iterates through API versions and prints info for every physical device
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{GPUCharacteristics, PhysicalDevice};
    use crate::vendor::Vendor;

    /// For testing purposes we use the Unknown vendor variant.
    impl Vendor {
        pub fn dummy() -> Self {
            Vendor::Unknown
        }
    }

    /// Creates a dummy PhysicalDevice instance for tests.
    fn dummy_physical_device() -> PhysicalDevice {
        PhysicalDevice {
            vendor: Vendor::dummy(),
            device_name: "TestDevice".to_string(),
            device_type: crate::device::DeviceType::DiscreteGPU,
            device_id: 0xDEADBEEF,
            vendor_id: 0xBEEF,
            driver_name: "TestDriver".to_string(),
            driver_info: "TestDriverInfo".to_string(),
            api_version: "1.2.3.4".to_string(),
            heapbudget: 8 * 1024 * 1024 * 1024, // 8 GB
            heapsize: 10 * 1024 * 1024 * 1024,  // 10 GB
            characteristics: GPUCharacteristics {
                memory_pressure: 0.2, // 20%
                compute_units: Some(10),
                shader_engines: Some(2),
                shader_arrays_per_engine_count: Some(2),
                compute_units_per_shader_array: Some(5),
                simd_per_compute_unit: Some(64),
                wavefronts_per_simd: Some(4),
                wavefront_size: Some(32),
                streaming_multiprocessors: Some(46),
                warps_per_sm: Some(32),
                max_image_dimension_2d: 16384,
                max_compute_shared_memory_size: 65536,
                max_compute_work_group_invocations: 1024,
                dedicated_transfer_queue: true,
                dedicated_async_compute_queue: true,
                supports_ray_tracing: true,
            },
        }
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.000 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.000 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.000 GB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 1024), "1.000 TB");
    }

    #[test]
    fn test_get_device_info() {
        let device = dummy_physical_device();
        let color = "\x1B[32m";
        let info = get_device_info(&device, color);
        assert!(info.len() >= 9);
        assert!(info[0].contains("TestDevice"));
        assert!(info[0].contains(device.device_type.name()));
        assert!(info[2].contains("0xDEADBEEF"));
        assert!(info[2].contains("0xBEEF"));
        assert!(info[7].contains("10") || info[7].contains("N/A"));
        assert!(info[8].contains("32") || info[8].contains("N/A"));
    }
}
