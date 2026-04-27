pub mod ascii_art;
pub mod device;
pub mod vendor;

use ascii_art::{BRIGHT_GREEN, BRIGHT_RED, BRIGHT_YELLOW};
use ash::{self, Entry, Instance, vk};
use device::Device;
use std::{
    error::Error,
    ffi::CStr,
    io::{self, Write},
};
use vt::enable_virtual_terminal_processing;

const BOLD: &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";
const DIM: &str = "\x1B[90m";
const WRAP_OFF: &str = "\x1B[?7l";
const WRAP_ON: &str = "\x1B[?7h";
const ALIGNMENT: &str = "    ";
const EMPTY: &str = "";

/// Fetches and prints information for a given physical device.
pub fn fetch_device(
    instance: &Instance,
    device_handle: vk::PhysicalDevice,
) -> Result<(), Box<dyn Error>> {
    let _ = enable_virtual_terminal_processing();
    let use_ansi = is_ansi_supported();

    let device = Device::new(instance, device_handle);
    let vendor = device.vendor;
    let art = vendor.get_ascii_art_with_ansi(use_ansi);
    let blank_art = " ".repeat(vendor.ascii_art_width());

    let accent = if use_ansi {
        vendor.get_alternative_style()[0]
    } else {
        EMPTY
    };
    let info = get_device_info(&device, accent, use_ansi);

    if use_ansi {
        print!("{}", WRAP_OFF);
        io::stdout().flush()?;
    }
    for i in 0..art.len().max(info.len()) {
        let art_line = art.get(i).map(String::as_str).unwrap_or(&blank_art);
        let info_line = info.get(i).map(String::as_str).unwrap_or(EMPTY);

        println!(" {} {}", art_line, info_line);
    }
    if use_ansi {
        print!("{}", WRAP_ON);
        io::stdout().flush()?;
    }

    println!();
    Ok(())
}

/// Returns a vector of formatted strings representing the device info,
/// including extra vendor-specific and general device limits.
/// Lines for optional fields are only included if available.
fn get_device_info(device: &Device, color: &str, use_ansi: bool) -> Vec<String> {
    let mut lines = Vec::new();
    let bold = if use_ansi { BOLD } else { EMPTY };
    let reset = if use_ansi { RESET } else { EMPTY };
    let value_color = if use_ansi { "\x1B[37m" } else { EMPTY };

    let title = format!(
        "{}{}{}{}: {}",
        bold,
        color,
        device.device_name,
        reset,
        device.device_type.name()
    );
    let underline_len = device.device_name.len() + device.device_type.name().len() + 3;
    let underline = "=".repeat(underline_len);

    // Basic device info.
    lines.push(title);
    lines.push(format!("{}{}{}{}", bold, color, underline, reset));
    lines.push(format!(
        "{}{}Device{}: {}0x{:X}{} : {}0x{:X}{} ({})",
        ALIGNMENT,
        color,
        reset,
        value_color,
        device.device_id,
        reset,
        value_color,
        device.vendor_id,
        reset,
        device.vendor.name(),
    ));
    push_driver_info(&mut lines, device, color, value_color, reset);
    lines.push(format!(
        "{}{}API{}: {}{}{}",
        ALIGNMENT, color, reset, value_color, device.api_version, reset
    ));

    let pressure_color = memory_pressure_color(device.characteristics.memory_pressure, use_ansi);
    let heap_budget = device
        .heapbudget
        .map(format_bytes)
        .unwrap_or_else(|| "???".to_string());
    lines.push(format!(
        "{}{}VRAM{}: {}{}{} / {}",
        ALIGNMENT,
        color,
        reset,
        pressure_color,
        heap_budget,
        reset,
        format_bytes(device.heapsize)
    ));

    let pressure = device.characteristics.memory_pressure;
    let pressure_text = format_memory_pressure(pressure);
    lines.push(format!(
        "{}{} % {}{}{}",
        ALIGNMENT,
        format_meter(30, pressure, use_ansi),
        pressure_color,
        pressure_text,
        reset
    ));

    // Vendor-specific extra info.
    if let Some(cu) = device.characteristics.compute_units {
        let compute_units = match device.characteristics.active_compute_units {
            Some(active) if active > 0 => format!("{} / {}", active, cu),
            _ => cu.to_string(),
        };
        lines.push(format!(
            "{}{}Compute Units{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, compute_units, reset
        ));
    }
    if let Some(se) = device.characteristics.shader_engines {
        lines.push(format!(
            "{}{}Shader Engines{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, se, reset
        ));
    }
    if let Some(sapec) = device.characteristics.shader_arrays_per_engine_count {
        lines.push(format!(
            "{}{}Shader Arrays per Engine{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, sapec, reset
        ));
    }
    if let Some(cups) = device.characteristics.compute_units_per_shader_array {
        lines.push(format!(
            "{}{}Compute Units per Shader Array{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, cups, reset
        ));
    }
    if let Some(simd) = device.characteristics.simd_per_compute_unit {
        lines.push(format!(
            "{}{}SIMD per Compute Unit{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, simd, reset
        ));
    }
    if let Some(wfs) = device.characteristics.wavefronts_per_simd {
        lines.push(format!(
            "{}{}Wavefronts per SIMD{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, wfs, reset
        ));
    }
    if let Some(wfsz) = device.characteristics.wavefront_size {
        lines.push(format!(
            "{}{}Wavefront Size{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, wfsz, reset
        ));
    }
    if let Some(sm) = device.characteristics.streaming_multiprocessors {
        lines.push(format!(
            "{}{}Streaming Multiprocessors{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, sm, reset
        ));
    }
    if let Some(wps) = device.characteristics.warps_per_sm {
        lines.push(format!(
            "{}{}Warps per SM{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, wps, reset
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
        reset,
        format_bytes(device.characteristics.max_compute_shared_memory_size.into())
    ));
    lines.push(format!(
        "{}{}Max Compute Work Group Invocations{}: {}",
        ALIGNMENT, color, reset, device.characteristics.max_compute_work_group_invocations
    ));

    let checkbox = |b: bool| if b { "[x]" } else { "[ ]" };
    let x = checkbox(device.characteristics.supports_ray_tracing);
    let y = checkbox(device.characteristics.dedicated_transfer_queue);
    let z = checkbox(device.characteristics.dedicated_async_compute_queue);

    lines.push(format!(
        "{}{}Raytracing{}: {} | {}Dedicated Transfer Queue{}: {} | {}Dedicated Async Compute Queue{}: {}",
        ALIGNMENT,
        color, reset, x,
        color, reset, y,
        color, reset, z,
    ));

    lines
}

fn push_driver_info(
    lines: &mut Vec<String>,
    device: &Device,
    color: &str,
    value_color: &str,
    reset: &str,
) {
    let mut driver_info_lines = device.driver_info.lines().filter(|line| !line.is_empty());
    match driver_info_lines.next() {
        Some(first_line) => lines.push(format!(
            "{}{}Driver{}: {}{}{} | {}{}{}",
            ALIGNMENT,
            color,
            reset,
            value_color,
            device.driver_name,
            reset,
            value_color,
            first_line,
            reset
        )),
        None => lines.push(format!(
            "{}{}Driver{}: {}{}{}",
            ALIGNMENT, color, reset, value_color, device.driver_name, reset
        )),
    }

    for line in driver_info_lines {
        lines.push(format!("           {}{}{}", value_color, line, reset));
    }
}

fn memory_pressure_color(pressure: Option<f32>, use_ansi: bool) -> &'static str {
    if !use_ansi {
        return EMPTY;
    }

    match pressure {
        Some(pressure) if pressure < 0.5 => BRIGHT_GREEN,
        Some(pressure) if pressure < 0.75 => BRIGHT_YELLOW,
        Some(_) => BRIGHT_RED,
        None => DIM,
    }
}

fn format_memory_pressure(pressure: Option<f32>) -> String {
    pressure
        .filter(|pressure| pressure.is_finite())
        .map(|pressure| format!("{:.2}", pressure * 100.0))
        .unwrap_or_else(|| "???".to_string())
}

fn format_meter(width: usize, completion: Option<f32>, use_ansi: bool) -> String {
    let inner_width = width.saturating_sub(2).max(1);
    let completion = completion.filter(|completion| completion.is_finite() && *completion >= 0.0);
    let mut result = String::with_capacity(width);

    result.push('[');
    for index in 0..inner_width {
        match completion {
            Some(completion) => {
                let denominator = inner_width.saturating_sub(1).max(1) as f32;
                let phase = index as f32 / denominator;
                if phase <= completion.clamp(0.0, 1.0) {
                    result.push_str(memory_pressure_color(Some(phase), use_ansi));
                    result.push('|');
                } else {
                    result.push(' ');
                }
            }
            None => {
                if use_ansi {
                    result.push_str(DIM);
                }
                result.push('-');
            }
        }
    }
    if use_ansi {
        result.push_str(RESET);
    }
    result.push(']');
    result
}

/// Converts a byte count into a human-readable string with binary units.
fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 9] = [
        "Bytes", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB",
    ];

    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit + 1 < UNITS.len() {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.3} {}", size, UNITS[unit])
}

/// Iterates through API versions and prints info for every physical device
pub fn iterate_devices() -> Result<(), Box<dyn Error>> {
    let entry = {
        #[cfg(not(feature = "loaded"))]
        {
            Entry::linked()
        }
        #[cfg(feature = "loaded")]
        {
            unsafe { Entry::load().map_err(|err| io::Error::other(format!("{err:?}")))? }
        }
    };

    let mut last_create_error = None;
    for api_version in [
        vk::API_VERSION_1_3,
        vk::API_VERSION_1_2,
        vk::API_VERSION_1_1,
        vk::API_VERSION_1_0,
    ] {
        let app_info = vk::ApplicationInfo::default()
            .application_name(c"vkfetch-rs")
            .application_version(package_version())
            .engine_name(c"vkfetch-rs")
            .engine_version(package_version())
            .api_version(api_version);

        let mut extension_names = Vec::new();
        let mut flags = vk::InstanceCreateFlags::empty();
        if supports_instance_extension(&entry, vk::KHR_PORTABILITY_ENUMERATION_NAME) {
            extension_names.push(vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr());
            flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
        }

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names)
            .flags(flags);

        match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => {
                match unsafe { instance.enumerate_physical_devices() } {
                    Ok(devices) => {
                        for device in devices {
                            if let Err(error) = fetch_device(&instance, device) {
                                unsafe {
                                    instance.destroy_instance(None);
                                }
                                return Err(error);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to enumerate physical devices: {:?}", e);
                    }
                };
                unsafe {
                    instance.destroy_instance(None);
                }
            }
            Err(e) => {
                eprintln!("Failed to create instance: {:?}", e);
                last_create_error = Some(e);
                continue;
            }
        };

        return Ok(());
    }

    match last_create_error {
        Some(error) => {
            Err(io::Error::other(format!("failed to create Vulkan instance: {error:?}")).into())
        }
        None => Ok(()),
    }
}

fn supports_instance_extension(entry: &Entry, extension_name: &CStr) -> bool {
    let Ok(extensions) = (unsafe { entry.enumerate_instance_extension_properties(None) }) else {
        return false;
    };

    extensions.iter().any(|extension| {
        extension
            .extension_name_as_c_str()
            .is_ok_and(|name| name == extension_name)
    })
}

fn package_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap_or(0);
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap_or(0);
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap_or(0);
    vk::make_api_version(0, major, minor, patch)
}

#[cfg(windows)]
mod vt {
    use std::io::{Error, Result};
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    /// Enables Virtual Terminal Processing on Windows.
    pub fn enable_virtual_terminal_processing() -> Result<()> {
        unsafe {
            let std_out = GetStdHandle(STD_OUTPUT_HANDLE);
            if std_out == INVALID_HANDLE_VALUE {
                return Err(Error::last_os_error());
            }
            let mut mode = 0;
            if GetConsoleMode(std_out, &mut mode) == 0 {
                return Err(Error::last_os_error());
            }
            mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            if SetConsoleMode(std_out, mode) == 0 {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }

    /// Checks if Virtual Terminal Processing is enabled.
    pub fn is_vt_enabled() -> bool {
        unsafe {
            let std_out = GetStdHandle(STD_OUTPUT_HANDLE);
            if std_out == INVALID_HANDLE_VALUE {
                return false;
            }
            let mut mode = 0;
            if GetConsoleMode(std_out, &mut mode) == 0 {
                return false;
            }
            (mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING) != 0
        }
    }
}

#[cfg(not(windows))]
mod vt {
    use std::io::Result;

    /// On non-Windows platforms, VT processing is typically enabled by default.
    pub fn enable_virtual_terminal_processing() -> Result<()> {
        Ok(())
    }

    /// Assume ANSI escape codes are supported.
    pub fn is_vt_enabled() -> bool {
        true
    }
}

/// Returns `true` if stdout is a TTY and (on Windows) VT processing is enabled.
fn is_ansi_supported() -> bool {
    std::io::IsTerminal::is_terminal(&std::io::stdout()) && vt::is_vt_enabled()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{Device, GPUCharacteristics};
    use crate::vendor::Vendor;

    /// For testing purposes we use the Unknown vendor variant.
    impl Vendor {
        pub fn dummy() -> Self {
            Vendor::Unknown
        }
    }

    /// Creates a dummy PhysicalDevice instance for tests.
    fn dummy_physical_device() -> Device {
        Device {
            vendor: Vendor::dummy(),
            device_name: "TestDevice".to_string(),
            device_type: crate::device::DeviceType::DiscreteGPU,
            device_id: 0xDEADBEEF,
            vendor_id: 0xBEEF,
            driver_name: "TestDriver".to_string(),
            driver_info: "TestDriverInfo\nSecond line".to_string(),
            api_version: "1.2.3.4".to_string(),
            heapbudget: Some(8 * 1024 * 1024 * 1024), // 8 GiB
            heapsize: 10 * 1024 * 1024 * 1024,        // 10 GB
            characteristics: GPUCharacteristics {
                memory_pressure: Some(0.2), // 20%
                compute_units: Some(10),
                active_compute_units: Some(8),
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
        assert_eq!(format_bytes(500), "500.000 Bytes");
        assert_eq!(format_bytes(1024), "1.000 KiB");
        assert_eq!(format_bytes(1024 * 1024), "1.000 MiB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.000 GiB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 1024), "1.000 TiB");
    }

    #[test]
    fn test_get_device_info() {
        let device = dummy_physical_device();
        let color = "\x1B[32m";
        let info = get_device_info(&device, color, true);
        assert!(info.len() >= 9);
        assert!(info[0].contains("TestDevice"));
        assert!(info[0].contains(device.device_type.name()));
        assert!(info[2].contains("0xDEADBEEF"));
        assert!(info[2].contains("0xBEEF"));
        assert!(info.iter().any(|line| line.contains("Second line")));
        assert!(info.iter().any(|line| line.contains("8 / 10")));
        assert!(info.iter().any(|line| line.contains("32")));
    }

    #[test]
    fn test_get_device_info_without_ansi() {
        let device = dummy_physical_device();
        let info = get_device_info(&device, EMPTY, false);
        assert!(!info.iter().any(|line| line.contains("\x1B")));
        assert!(info.iter().any(|line| line.contains("8.000 GiB")));
    }

    #[test]
    fn test_unknown_memory_pressure_meter() {
        let meter = format_meter(6, None, false);
        assert_eq!(meter, "[----]");
    }
}
