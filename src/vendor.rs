use crate::ascii_art::*;

pub enum Vendor {
    AMD = 0x1002,
    ImgTec = 0x1010,
    Apple = 0x106B,
    Nvidia = 0x10DE,
    ARM = 0x13B5,
    Google = 0x1AE0,
    Qualcomm = 0x5143,
    Intel = 0x8086,
    Unknown = 0xFFFF,
    VIV = 0x10001,
    VSI = 0x10002,
    Kazan = 0x10003,
    Codeplay = 0x10004,
    Mesa = 0x10005,
    Pocl = 0x10006,
    MobileEye = 0x10007,
}

const BLOCK: &str = "\x1B[7m \x1B[0m";

impl Vendor {
    pub fn get_ascii_art(&self) -> Vec<String> {
        let art: &[&str] = match self {
            Vendor::AMD => AMD,
            Vendor::Apple => APPLE,
            Vendor::ARM => ARM,
            Vendor::Google => GOOGLE,
            Vendor::Intel => INTEL,
            Vendor::Mesa => MESA,
            Vendor::Nvidia => NVIDIA,
            Vendor::Unknown => VULKAN,
            _ => VULKAN,
        };

        let styles = self.get_styles();

        let mut modified_art: Vec<String> = art.iter().map(|&line| line.to_string()).collect();

        for (char, style) in CHARS.iter().zip(styles.iter()) {
            if !style.is_empty() {
                modified_art = modified_art
                    .iter()
                    .map(|line| line.replace(*char, &format!("{}{}", style, BLOCK)))
                    .collect();
            }
        }

        modified_art
    }

    pub const fn get_styles(&self) -> [&str; LUT_SIZE] {
        match self {
            Vendor::AMD => AMD_STYLE,
            Vendor::Apple => APPLE_STYLE,
            Vendor::ARM => ARM_STYLE,
            Vendor::Google => GOOGLE_STYLE,
            Vendor::Intel => INTEL_STYLE,
            Vendor::Mesa => MESA_STYLE,
            Vendor::Nvidia => NVIDIA_STYLE,
            Vendor::Unknown => VULKAN_STYLE,
            _ => VULKAN_STYLE,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Vendor::AMD => "AMD",
            Vendor::Apple => "Apple",
            Vendor::ARM => "ARM",
            Vendor::Codeplay => "Codeplay",
            Vendor::Google => "Google",
            Vendor::ImgTec => "ImgTec",
            Vendor::Intel => "Intel",
            Vendor::Kazan => "Kazan",
            Vendor::Mesa => "Mesa",
            Vendor::MobileEye => "MobileEye",
            Vendor::Nvidia => "Nvidia",
            Vendor::Pocl => "Pocl",
            Vendor::Qualcomm => "Qualcomm",
            Vendor::Unknown => "Unknown",
            Vendor::VIV => "VIV",
            Vendor::VSI => "VSI",
        }
    }

    pub fn from_vendor_id(id: u32) -> Option<Self> {
        match id {
            0x1002 => Some(Vendor::AMD),
            0x1010 => Some(Vendor::ImgTec),
            0x106B => Some(Vendor::Apple),
            0x10DE => Some(Vendor::Nvidia),
            0x13B5 => Some(Vendor::ARM),
            0x1AE0 => Some(Vendor::Google),
            0x5143 => Some(Vendor::Qualcomm),
            0x8086 => Some(Vendor::Intel),
            0xFFFF => Some(Vendor::Unknown),
            0x10001 => Some(Vendor::VIV),
            0x10002 => Some(Vendor::VSI),
            0x10003 => Some(Vendor::Kazan),
            0x10004 => Some(Vendor::Codeplay),
            0x10005 => Some(Vendor::Mesa),
            0x10006 => Some(Vendor::Pocl),
            0x10007 => Some(Vendor::MobileEye),
            _ => None,
        }
    }
}

impl From<Vendor> for u32 {
    fn from(v: Vendor) -> Self {
        v as u32
    }
}
