use vkfetch_rs::iterate_devices;
fn main() {
    // let vendor = vkfetch_rs::vendor::Vendor::Unknown;
    // debug(vendor.get_ascii_art());
    // let vendor = vkfetch_rs::vendor::Vendor::Apple;
    // debug(vendor.get_ascii_art());
    // let vendor = vkfetch_rs::vendor::Vendor::Google;
    // debug(vendor.get_ascii_art());
    // let vendor = vkfetch_rs::vendor::Vendor::Intel;
    // debug(vendor.get_ascii_art());
    // let vendor = vkfetch_rs::vendor::Vendor::Nvidia;
    // debug(vendor.get_ascii_art());
    // let vendor = vkfetch_rs::vendor::Vendor::AMD;
    // debug(vendor.get_ascii_art());

    iterate_devices()
}

#[allow(dead_code)]
fn debug(x: Vec<String>) {
    println!(" {}", x.join("\n "));
}
