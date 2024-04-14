use ash::{
    vk::{self},
    Instance,
};

use crate::vendor::Vendor;

pub struct PhysicalDevice {
    pub vendor: Vendor,
    pub device_name: String, // :
    pub device_type: DeviceType,
    // Device :
    pub device_id: u32, // :
    pub vendor_id: u32, // (VendorName)
    // Driver:
    pub driver_name: String, // :
    pub driver_info: String,
    // API:
    pub api_version: String,
    // VRAM:
    // pub heapbudget: u64,
    // pub heapsize: u64,
    // pub characteristics: Option<GPUCharacteristics>,
}

#[allow(dead_code)]
struct GPUCharacteristics {
    compute_units: u32,
    shader_engines: u32,
    shader_arrays_per_engine_count: u32,
    compute_units_per_shader_array: u32,
    simd_per_compute_unit: u32,
    wavefronts_per_simd: u32,
    wavefront_size: u32,
}

impl PhysicalDevice {
    pub fn new(instance: &Instance, physical_device: vk::PhysicalDevice) -> Self {
        let physical_device_properties: vk::PhysicalDeviceProperties =
            unsafe { instance.get_physical_device_properties(physical_device) };

        let mut driver_properties: vk::PhysicalDeviceDriverProperties =
            vk::PhysicalDeviceDriverProperties::default();

        let mut properties2: vk::PhysicalDeviceProperties2 =
            vk::PhysicalDeviceProperties2::default().push_next(&mut driver_properties);

        unsafe {
            instance.get_physical_device_properties2(physical_device, &mut properties2);
        };

        let vendor_id = physical_device_properties.vendor_id;

        let vendor = Vendor::from_vendor_id(vendor_id).unwrap();

        let device_name =
            cstring_to_string(physical_device_properties.device_name_as_c_str().unwrap());

        let device_type = DeviceType::from(physical_device_properties.device_type.as_raw());

        let device_id = physical_device_properties.device_id;

        let api_version = decode_version_number(physical_device_properties.api_version);

        let driver_name = cstring_to_string(driver_properties.driver_name_as_c_str().unwrap());

        let driver_info = cstring_to_string(driver_properties.driver_info_as_c_str().unwrap());

        PhysicalDevice {
            vendor,
            device_name,
            device_type,
            device_id,
            vendor_id,
            driver_name,
            driver_info,
            api_version,
        }
    }
}

pub enum DeviceType {
    Other = 0,
    IntegratedGPU = 1,
    DiscreteGPU = 2,
    VirtualGPU = 3,
    CPU = 4,
    Unknown = 5,
}

impl DeviceType {
    pub fn from(id: i32) -> Self {
        match id {
            0 => DeviceType::Other,
            1 => DeviceType::IntegratedGPU,
            2 => DeviceType::DiscreteGPU,
            3 => DeviceType::VirtualGPU,
            4 => DeviceType::CPU,
            _ => DeviceType::Unknown,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            DeviceType::Other => "Other",
            DeviceType::IntegratedGPU => "Integrated GPU",
            DeviceType::DiscreteGPU => "Discrete GPU",
            DeviceType::VirtualGPU => "Virtual GPU",
            DeviceType::CPU => "CPU",
            DeviceType::Unknown => "Unknown",
        }
    }
}

/*
    The variant is a 3-bit integer packed into bits 31-29.
    The major version is a 7-bit integer packed into bits 28-22.
    The minor version number is a 10-bit integer packed into bits 21-12.
    The patch version number is a 12-bit integer packed into bits 11-0.
*/
pub fn decode_version_number(version: u32) -> String {
    let variant = (version >> 29) & 0b111;
    let major = (version >> 22) & 0b1111111;
    let minor = (version >> 12) & 0b1111111111;
    let patch = version & 0b111111111111;
    format!("{}.{}.{}.{}", variant, major, minor, patch)
}

pub fn cstring_to_string(cstr: &std::ffi::CStr) -> String {
    cstr.to_string_lossy().to_string()
}
