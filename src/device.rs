use ash::{
    vk::{self},
    Instance,
};

use crate::vendor::Vendor;

pub struct PhysicalDevice {
    pub vendor: Vendor,
    pub device_name: String,
    pub device_type: DeviceType,
    pub device_id: u32,
    pub vendor_id: u32,
    pub driver_name: String,
    pub driver_info: String,
    pub api_version: String,
    // VRAM:
    pub heapbudget: u64,
    pub heapsize: u64,
    pub characteristics: GPUCharacteristics,
}

pub struct GPUCharacteristics {
    pub memory_pressure: f32,
    pub compute_units: u32,
    pub shader_engines: u32,
    pub shader_arrays_per_engine_count: u32,
    pub compute_units_per_shader_array: u32,
    pub simd_per_compute_unit: u32,
    pub wavefronts_per_simd: u32,
    pub wavefront_size: u32,
    // Nvidia specific
    pub streaming_multiprocessors: Option<u32>,
    pub warps_per_sm: Option<u32>,
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

        let vendor = match Vendor::from_vendor_id(vendor_id) {
            Some(v) => v,
            None => {
                eprintln!("Unknown vendor: {}", vendor_id);
                panic!();
            }
        };

        let device_name =
            cstring_to_string(physical_device_properties.device_name_as_c_str().unwrap());

        let device_type = DeviceType::from(physical_device_properties.device_type.as_raw());

        let device_id = physical_device_properties.device_id;

        let api_version = decode_version_number(physical_device_properties.api_version);

        let driver_name = cstring_to_string(driver_properties.driver_name_as_c_str().unwrap());

        let driver_info = cstring_to_string(driver_properties.driver_info_as_c_str().unwrap());

        let mut memory_budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
        let mut memory_properties2 =
            vk::PhysicalDeviceMemoryProperties2::default().push_next(&mut memory_budget);
        unsafe {
            instance
                .get_physical_device_memory_properties2(physical_device, &mut memory_properties2);
        }
        let memory_properties = memory_properties2.memory_properties;

        // Determine VRAM heap index (first DEVICE_LOCAL heap)
        let vram_heap_index = (0..memory_properties.memory_heap_count)
            .find(|&i| {
                memory_properties.memory_heaps[i as usize]
                    .flags
                    .contains(vk::MemoryHeapFlags::DEVICE_LOCAL)
            })
            .unwrap_or(0);

        // Compute heapsize, budget, and memory pressure
        let heapsize = memory_properties.memory_heaps[vram_heap_index as usize].size;
        let heapbudget = memory_budget.heap_budget[vram_heap_index as usize];
        let memory_pressure = if heapbudget > 0 {
            (heapsize - heapbudget) as f32 / heapsize as f32
        } else {
            f32::NAN
        };

        // Get vendor-specific characteristics.
        let characteristics = match vendor {
            Vendor::AMD => {
                let mut shader_core_properties =
                    vk::PhysicalDeviceShaderCorePropertiesAMD::default();
                let mut shader_core_properties2 =
                    vk::PhysicalDeviceShaderCoreProperties2AMD::default();
                let mut amd_properties2 = vk::PhysicalDeviceProperties2::default()
                    .push_next(&mut shader_core_properties)
                    .push_next(&mut shader_core_properties2);
                unsafe {
                    instance.get_physical_device_properties2(physical_device, &mut amd_properties2);
                }
                GPUCharacteristics {
                    memory_pressure,
                    compute_units: shader_core_properties.shader_engine_count
                        * shader_core_properties.shader_arrays_per_engine_count
                        * shader_core_properties.compute_units_per_shader_array,
                    shader_engines: shader_core_properties.shader_engine_count,
                    shader_arrays_per_engine_count: shader_core_properties
                        .shader_arrays_per_engine_count,
                    compute_units_per_shader_array: shader_core_properties
                        .compute_units_per_shader_array,
                    simd_per_compute_unit: shader_core_properties.simd_per_compute_unit,
                    wavefronts_per_simd: shader_core_properties.wavefronts_per_simd,
                    wavefront_size: shader_core_properties.wavefront_size,
                    streaming_multiprocessors: None,
                    warps_per_sm: None,
                }
            }
            Vendor::Nvidia => {
                let mut sm_builtins = vk::PhysicalDeviceShaderSMBuiltinsPropertiesNV::default();
                let mut nv_properties2 =
                    vk::PhysicalDeviceProperties2::default().push_next(&mut sm_builtins);
                unsafe {
                    instance.get_physical_device_properties2(physical_device, &mut nv_properties2);
                }
                GPUCharacteristics {
                    memory_pressure,
                    // For NVIDIA, AMD-specific values are not applicable.
                    compute_units: 0,
                    shader_engines: 0,
                    shader_arrays_per_engine_count: 0,
                    compute_units_per_shader_array: 0,
                    simd_per_compute_unit: 0,
                    wavefronts_per_simd: 0,
                    wavefront_size: 0,
                    streaming_multiprocessors: Some(sm_builtins.shader_sm_count),
                    warps_per_sm: Some(sm_builtins.shader_warps_per_sm),
                }
            }
            _ => GPUCharacteristics {
                memory_pressure,
                compute_units: 0,
                shader_engines: 0,
                shader_arrays_per_engine_count: 0,
                compute_units_per_shader_array: 0,
                simd_per_compute_unit: 0,
                wavefronts_per_simd: 0,
                wavefront_size: 0,
                streaming_multiprocessors: None,
                warps_per_sm: None,
            },
        };

        PhysicalDevice {
            vendor,
            device_name,
            device_type,
            device_id,
            vendor_id,
            driver_name,
            driver_info,
            api_version,
            heapbudget,
            heapsize,
            characteristics,
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
