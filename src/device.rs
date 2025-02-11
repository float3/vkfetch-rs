use ash::vk;
use ash::vk::PhysicalDeviceProperties2;
use ash::vk::PhysicalDeviceShaderCoreProperties2AMD;
use ash::vk::PhysicalDeviceShaderCorePropertiesAMD;
use ash::vk::PhysicalDeviceShaderSMBuiltinsPropertiesNV;
use ash::Instance;
use std::ffi::CStr;

use crate::vendor::Vendor;

/// Represents a physical GPU device.
#[derive(Debug)]
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

/// Contains various characteristics of a GPU.
/// Vendor-specific properties are stored as Options.
/// Also includes some general device limits.
#[derive(Debug)]
pub struct GPUCharacteristics {
    /// Memory pressure as computed from VRAM usage (0.0 to 1.0)
    pub memory_pressure: f32,
    // AMD-specific properties.
    pub compute_units: Option<u32>,
    pub shader_engines: Option<u32>,
    pub shader_arrays_per_engine_count: Option<u32>,
    pub compute_units_per_shader_array: Option<u32>,
    pub simd_per_compute_unit: Option<u32>,
    pub wavefronts_per_simd: Option<u32>,
    pub wavefront_size: Option<u32>,
    // NVIDIA-specific properties.
    pub streaming_multiprocessors: Option<u32>,
    pub warps_per_sm: Option<u32>,
    // General device limits.
    pub max_image_dimension_2d: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_invocations: u32,
    // New feature flags.
    pub dedicated_transfer_queue: bool,
    pub dedicated_async_compute_queue: bool,
    pub supports_ray_tracing: bool,
}

impl PhysicalDevice {
    /// Constructs a new `PhysicalDevice` by querying Vulkan properties.
    pub fn new(instance: &Instance, physical_device: vk::PhysicalDevice) -> Self {
        // Get the core properties and limits.
        let physical_device_properties: vk::PhysicalDeviceProperties =
            unsafe { instance.get_physical_device_properties(physical_device) };
        let limits = physical_device_properties.limits;

        // Query additional driver properties.
        let mut driver_properties: vk::PhysicalDeviceDriverProperties =
            vk::PhysicalDeviceDriverProperties::default();
        let mut properties2: PhysicalDeviceProperties2 =
            PhysicalDeviceProperties2::default().push_next(&mut driver_properties);
        unsafe {
            instance.get_physical_device_properties2(physical_device, &mut properties2);
        }

        let vendor_id = physical_device_properties.vendor_id;
        let vendor = Vendor::from_vendor_id(vendor_id).unwrap_or_else(|| {
            eprintln!("Unknown vendor: {}", vendor_id);
            panic!();
        });

        let device_name = cstring_to_string(
            physical_device_properties
                .device_name_as_c_str()
                .unwrap_or(c"Unknown"),
        );
        let device_type = DeviceType::from(physical_device_properties.device_type.as_raw());
        let device_id = physical_device_properties.device_id;
        let api_version = decode_version_number(physical_device_properties.api_version);
        let driver_name = cstring_to_string(
            driver_properties
                .driver_name_as_c_str()
                .unwrap_or(c"Unknown"),
        );
        let driver_info = cstring_to_string(
            driver_properties
                .driver_info_as_c_str()
                .unwrap_or(c"Unknown"),
        );

        // Query VRAM details.
        let mut memory_budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
        let mut memory_properties2 =
            vk::PhysicalDeviceMemoryProperties2::default().push_next(&mut memory_budget);
        unsafe {
            instance
                .get_physical_device_memory_properties2(physical_device, &mut memory_properties2);
        }
        let memory_properties = memory_properties2.memory_properties;
        let vram_heap_index = (0..memory_properties.memory_heap_count)
            .find(|&i| {
                memory_properties.memory_heaps[i as usize]
                    .flags
                    .contains(vk::MemoryHeapFlags::DEVICE_LOCAL)
            })
            .unwrap_or(0);
        let heapsize = memory_properties.memory_heaps[vram_heap_index as usize].size;
        let heapbudget = memory_budget.heap_budget[vram_heap_index as usize];
        let memory_pressure = if heapbudget > 0 {
            (heapsize - heapbudget) as f32 / heapsize as f32
        } else {
            f32::NAN
        };

        // Query queue family properties.
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        let mut dedicated_transfer_queue = false;
        let mut dedicated_async_compute_queue = false;
        for qf in queue_families.iter() {
            let flags = qf.queue_flags;
            if flags.contains(vk::QueueFlags::TRANSFER)
                && !(flags.contains(vk::QueueFlags::GRAPHICS)
                    || flags.contains(vk::QueueFlags::COMPUTE))
            {
                dedicated_transfer_queue = true;
            }
            if flags.contains(vk::QueueFlags::COMPUTE) && !flags.contains(vk::QueueFlags::GRAPHICS)
            {
                dedicated_async_compute_queue = true;
            }
        }

        // Check for ray tracing support via device extensions.
        let extensions = unsafe {
            instance
                .enumerate_device_extension_properties(physical_device)
                .unwrap_or_default()
        };
        let supports_ray_tracing = extensions.iter().any(|ext| {
            let ext_name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
            ext_name.to_str().unwrap_or("") == "VK_KHR_ray_tracing_pipeline"
                || ext_name.to_str().unwrap_or("") == "VK_NV_ray_tracing"
        });

        let mut characteristics = GPUCharacteristics {
            memory_pressure,
            // Vendor-specific fields start as None.
            compute_units: None,
            shader_engines: None,
            shader_arrays_per_engine_count: None,
            compute_units_per_shader_array: None,
            simd_per_compute_unit: None,
            wavefronts_per_simd: None,
            wavefront_size: None,
            streaming_multiprocessors: None,
            warps_per_sm: None,
            // General limits:
            max_image_dimension_2d: limits.max_image_dimension2_d,
            max_compute_shared_memory_size: limits.max_compute_shared_memory_size,
            max_compute_work_group_invocations: limits.max_compute_work_group_invocations,
            // New features:
            dedicated_transfer_queue,
            dedicated_async_compute_queue,
            supports_ray_tracing,
        };

        // Query vendor-specific properties.
        match vendor {
            Vendor::AMD => {
                let mut shader_core_properties = PhysicalDeviceShaderCorePropertiesAMD::default();
                let mut shader_core_properties2 = PhysicalDeviceShaderCoreProperties2AMD::default();
                let mut amd_properties2 = PhysicalDeviceProperties2::default()
                    .push_next(&mut shader_core_properties)
                    .push_next(&mut shader_core_properties2);
                unsafe {
                    instance.get_physical_device_properties2(physical_device, &mut amd_properties2);
                }
                characteristics.compute_units = Some(
                    shader_core_properties.shader_engine_count
                        * shader_core_properties.shader_arrays_per_engine_count
                        * shader_core_properties.compute_units_per_shader_array,
                );
                characteristics.shader_engines = Some(shader_core_properties.shader_engine_count);
                characteristics.shader_arrays_per_engine_count =
                    Some(shader_core_properties.shader_arrays_per_engine_count);
                characteristics.compute_units_per_shader_array =
                    Some(shader_core_properties.compute_units_per_shader_array);
                characteristics.simd_per_compute_unit =
                    Some(shader_core_properties.simd_per_compute_unit);
                characteristics.wavefronts_per_simd =
                    Some(shader_core_properties.wavefronts_per_simd);
                characteristics.wavefront_size = Some(shader_core_properties.wavefront_size);
            }
            Vendor::Nvidia => {
                let mut sm_builtins = PhysicalDeviceShaderSMBuiltinsPropertiesNV::default();
                let mut nv_properties2 =
                    PhysicalDeviceProperties2::default().push_next(&mut sm_builtins);
                unsafe {
                    instance.get_physical_device_properties2(physical_device, &mut nv_properties2);
                }
                characteristics.streaming_multiprocessors = Some(sm_builtins.shader_sm_count);
                characteristics.warps_per_sm = Some(sm_builtins.shader_warps_per_sm);
            }
            _ => {
                // For other vendors, vendor-specific fields remain None.
            }
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

/// Represents the type of device.
#[derive(Debug)]
pub enum DeviceType {
    Other = 0,
    IntegratedGPU = 1,
    DiscreteGPU = 2,
    VirtualGPU = 3,
    CPU = 4,
    Unknown = 5,
}

impl DeviceType {
    /// Converts an integer ID (from Vulkan) into a DeviceType.
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

    /// Returns a human‑readable name.
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

/// Decodes a Vulkan version number into a string of the form "variant.major.minor.patch".
pub fn decode_version_number(version: u32) -> String {
    let variant = (version >> 29) & 0b111;
    let major = (version >> 22) & 0b1111111;
    let minor = (version >> 12) & 0b1111111111;
    let patch = version & 0b111111111111;
    format!("{}.{}.{}.{}", variant, major, minor, patch)
}

/// Converts a CStr to a Rust String.
pub fn cstring_to_string(cstr: &CStr) -> String {
    cstr.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ash::vk;
    use std::ffi::CString;

    // Helper to create a dummy CString.
    fn dummy_cstr(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    #[test]
    fn test_decode_version_number() {
        // Simulate a Vulkan version: variant 0, version 1.2.3
        let version: u32 = (1 << 22) | (2 << 12) | 3;
        let decoded = decode_version_number(version);
        assert_eq!(decoded, "0.1.2.3");
    }

    #[test]
    fn test_cstring_to_string() {
        let original = "Hello, world!";
        let cstr = dummy_cstr(original);
        let s = cstring_to_string(cstr.as_c_str());
        assert_eq!(s, original);
    }

    #[test]
    fn test_device_type_from() {
        assert_eq!(DeviceType::from(0).name(), "Other");
        assert_eq!(DeviceType::from(1).name(), "Integrated GPU");
        assert_eq!(DeviceType::from(2).name(), "Discrete GPU");
        assert_eq!(DeviceType::from(3).name(), "Virtual GPU");
        assert_eq!(DeviceType::from(4).name(), "CPU");
        assert_eq!(DeviceType::from(99).name(), "Unknown");
    }

    #[test]
    fn test_gpu_characteristics_defaults() {
        // Create dummy limits.
        let limits = vk::PhysicalDeviceLimits {
            max_image_dimension2_d: 8192,
            max_compute_shared_memory_size: 16384,
            max_compute_work_group_invocations: 1024,
            ..Default::default()
        };

        // Construct dummy GPUCharacteristics with only common limits.
        let characteristics = GPUCharacteristics {
            memory_pressure: 0.5,
            compute_units: None,
            shader_engines: None,
            shader_arrays_per_engine_count: None,
            compute_units_per_shader_array: None,
            simd_per_compute_unit: None,
            wavefronts_per_simd: None,
            wavefront_size: None,
            streaming_multiprocessors: None,
            warps_per_sm: None,
            max_image_dimension_2d: limits.max_image_dimension2_d,
            max_compute_shared_memory_size: limits.max_compute_shared_memory_size,
            max_compute_work_group_invocations: limits.max_compute_work_group_invocations,
            dedicated_transfer_queue: false,
            dedicated_async_compute_queue: false,
            supports_ray_tracing: false,
        };

        assert_eq!(characteristics.max_image_dimension_2d, 8192);
        assert_eq!(characteristics.max_compute_shared_memory_size, 16384);
        assert_eq!(characteristics.max_compute_work_group_invocations, 1024);
        assert!(characteristics.compute_units.is_none());
    }
}
