fn main() {
    match pkg_config::probe_library("vulkan") {
        Ok(_) => {}
        Err(e) => {
            panic!(
                "Feature `vulkan` is enabled but failed to find the Vulkan loader: {}",
                e
            );
        }
    }
}
