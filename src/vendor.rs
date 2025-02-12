use crate::{ascii_art::*, is_ansi_supported};

/// Represents a GPU vendor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vendor {
    AMD = 0x1002,
    ImgTec = 0x1010,
    Apple = 0x106B,
    Nvidia = 0x10DE,
    ARM = 0x13B5,
    Microsoft = 0x1414,
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
    /// Returns the vendor-specific ASCII art with color styling applied.
    pub fn get_ascii_art(&self) -> Vec<String> {
        let art: &[&str] = match self {
            Vendor::AMD => AMD,
            Vendor::Apple => APPLE,
            Vendor::ARM => ARM,
            Vendor::Google => GOOGLE,
            Vendor::Intel => INTEL,
            Vendor::Nvidia => NVIDIA,
            Vendor::Microsoft => MICROSOFT,
            Vendor::Qualcomm => QUALCOMM,
            Vendor::Mesa => VULKAN,
            Vendor::Unknown
            | Vendor::ImgTec
            | Vendor::VIV
            | Vendor::VSI
            | Vendor::Kazan
            | Vendor::Codeplay
            | Vendor::Pocl
            | Vendor::MobileEye => VULKAN,
        };

        let style = if is_ansi_supported() {
            self.get_alternative_style()
        } else {
            self.get_style()
        };
        let mut modified_art: Vec<String> = art.iter().map(|&line| line.to_string()).collect();

        for (symbol, style) in CHARS.iter().zip(style.iter()) {
            if !style.is_empty() {
                modified_art = modified_art
                    .iter()
                    .map(|line| line.replace(*symbol, &format!("{}{}", style, BLOCK)))
                    .collect();
            }
        }

        modified_art
    }

    /// Returns an array of style strings associated with the vendor.
    pub const fn get_style(&self) -> [&str; LUT_SIZE] {
        match self {
            Vendor::AMD => AMD_STYLE,
            Vendor::Apple => APPLE_STYLE,
            Vendor::ARM => ARM_STYLE,
            Vendor::Google => GOOGLE_STYLE,
            Vendor::Intel => INTEL_STYLE,
            Vendor::Nvidia => NVIDIA_STYLE,
            Vendor::Microsoft => MICROSOFT_STYLE,
            Vendor::Qualcomm => QUALCOMM_STYLE,
            Vendor::Mesa => VULKAN_STYLE,
            Vendor::Unknown
            | Vendor::ImgTec
            | Vendor::VIV
            | Vendor::VSI
            | Vendor::Kazan
            | Vendor::Codeplay
            | Vendor::Pocl
            | Vendor::MobileEye => VULKAN_STYLE,
        }
    }

    /// Returns a human-readable name for the vendor.
    pub const fn name(&self) -> &'static str {
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
            Vendor::Microsoft => "Microsoft",
        }
    }

    /// Constructs a Vendor from a vendor ID, if recognized.
    pub fn from_vendor_id(id: u32) -> Option<Self> {
        match id {
            0x1002 => Some(Vendor::AMD),
            0x1010 => Some(Vendor::ImgTec),
            0x106B => Some(Vendor::Apple),
            0x10DE => Some(Vendor::Nvidia),
            0x13B5 => Some(Vendor::ARM),
            0x1414 => Some(Vendor::Microsoft),
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

    pub const fn get_alternative_style(&self) -> [&str; LUT_SIZE] {
        match self {
            Vendor::AMD => AMD_STYLE_ALT,
            Vendor::Apple => APPLE_STYLE_ALT,
            Vendor::ARM => ARM_STYLE_ALT,
            Vendor::Google => GOOGLE_STYLE_ALT,
            Vendor::Intel => INTEL_STYLE_ALT,
            Vendor::Nvidia => NVIDIA_STYLE_ALT,
            Vendor::Microsoft => MICROSOFT_STYLE_ALT,
            Vendor::Qualcomm => QUALCOMM_STYLE_ALT,
            Vendor::Mesa => VULKAN_STYLE_ALT,
            Vendor::Unknown
            | Vendor::ImgTec
            | Vendor::VIV
            | Vendor::VSI
            | Vendor::Kazan
            | Vendor::Codeplay
            | Vendor::Pocl
            | Vendor::MobileEye => VULKAN_STYLE_ALT,
        }
    }
}

impl From<Vendor> for u32 {
    fn from(v: Vendor) -> Self {
        v as u32
    }
}

/// Allows a vendor to be printed using its human‑readable name.
impl std::fmt::Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vendor_id() {
        assert_eq!(Vendor::from_vendor_id(0x1002), Some(Vendor::AMD));
        assert_eq!(Vendor::from_vendor_id(0x1010), Some(Vendor::ImgTec));
        assert_eq!(Vendor::from_vendor_id(0x10DE), Some(Vendor::Nvidia));
        assert_eq!(Vendor::from_vendor_id(0x9999), None);
    }

    #[test]
    fn test_name() {
        assert_eq!(Vendor::AMD.name(), "AMD");
        assert_eq!(Vendor::Apple.name(), "Apple");
        assert_eq!(Vendor::Intel.name(), "Intel");
        assert_eq!(Vendor::Unknown.name(), "Unknown");
    }

    #[test]
    fn test_display() {
        let vendor = Vendor::Nvidia;
        let s = format!("{}", vendor);
        assert_eq!(s, vendor.name());
    }

    #[test]
    fn test_get_styles_length() {
        let vendor = Vendor::AMD;
        let styles = vendor.get_style();
        assert_eq!(styles.len(), crate::ascii_art::LUT_SIZE);
    }

    #[test]
    fn test_get_ascii_art() {
        let vendor = Vendor::AMD;
        let art = vendor.get_ascii_art();
        assert!(!art.is_empty(), "ASCII art should not be empty");

        let styles = vendor.get_style();
        let non_empty_style = styles.iter().any(|s| !s.is_empty());
        if non_empty_style {
            let block_found = art.iter().any(|line| line.contains(BLOCK));
            assert!(block_found, "Styled block should appear in ASCII art lines");
        }
    }
}
