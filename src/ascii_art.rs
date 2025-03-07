pub(crate) const LUT_SIZE: usize = 5;

pub(crate) const CHARS: [char; LUT_SIZE] = ['#', '$', '%', '&', '@'];

pub(crate) const BLACK: &str = "\x1B[30m";
pub(crate) const RED: &str = "\x1B[31m";
pub(crate) const BRIGHT_RED: &str = "\x1B[91m";
pub(crate) const GREEN: &str = "\x1B[32m";
pub(crate) const BRIGHT_GREEN: &str = "\x1B[92m";
pub(crate) const YELLOW: &str = "\x1B[33m";
pub(crate) const BRIGHT_YELLOW: &str = "\x1B[93m";
pub(crate) const BLUE: &str = "\x1B[34m";
pub(crate) const BRIGHT_BLUE: &str = "\x1B[94m";
// pub(crate) const MAGENTA: &str = "\x1B[35m";
// pub(crate) const BRIGHT_MAGENTA: &str = "\x1B[95m";
pub(crate) const CYAN: &str = "\x1B[36m";
pub(crate) const BRIGHT_CYAN: &str = "\x1B[96m";
pub(crate) const WHITE: &str = "\x1B[37m";
pub(crate) const BRIGHT_WHITE: &str = "\x1B[97m";

pub(crate) const VULKAN: &[&str] = &[
    r#"          ################                     "#,
    r#"      ##########################               "#,
    r#"    #############         ##########           "#,
    r#"   #########                     ######        "#,
    r#"   #######     ####      ####         ####     "#,
    r#"    #####      ####      ####            ###   "#,
    r#"     ####       ####    ####               ##  "#,
    r#"      ####      ####    ####                 # "#,
    r#"        ###      ####  ####                    "#,
    r#"          ##     ####  ####                    "#,
    r#"                  ########                     "#,
    r#"                  ########                     "#,
    r#"                   ######                      "#,
];

pub(crate) const VULKAN_STYLE: [&str; LUT_SIZE] = [RED, "", "", "", BLACK];
pub(crate) const VULKAN_STYLE_ALT: [&str; LUT_SIZE] = [BRIGHT_RED, "", "", "", BLACK];

pub(crate) const APPLE: &[&str] = &[
    r#"                          ###                  "#,
    r#"                        ####                   "#,
    r#"                       ####                    "#,
    r#"              ####### ###  #######             "#,
    r#"            ########################           "#,
    r#"           ########################            "#,
    r#"          #######################              "#,
    r#"         #######################               "#,
    r#"         #######################               "#,
    r#"         ########################              "#,
    r#"          #########################            "#,
    r#"           ##########################          "#,
    r#"             ######################            "#,
    r#"              ###################              "#,
    r#"                ######    #####                "#,
];

pub(crate) const APPLE_STYLE: [&str; LUT_SIZE] = [WHITE, "", "", "", BLACK];
pub(crate) const APPLE_STYLE_ALT: [&str; LUT_SIZE] = [BRIGHT_WHITE, "", "", "", BLACK];

pub(crate) const GOOGLE: &[&str] = &[
    r#"                    #########                  "#,
    r#"                #################              "#,
    r#"              #####################            "#,
    r#"            &#########    #######              "#,
    r#"          &&&&#####          ##                "#,
    r#"         &&&&&&&#                              "#,
    r#"         &&&&&&&                               "#,
    r#"        &&&&&&&          $$$$$$$$$$$$$$        "#,
    r#"        &&&&&&&          $$$$$$$$$$$$$$        "#,
    r#"        &&&&&&&          $$$$$$$$$$$$$$        "#,
    r#"         &&&&&&&                $$$$$$$        "#,
    r#"         &&&&&&&%              $$$$$$$         "#,
    r#"          &&&&%%%%%          %$$$$$$$$         "#,
    r#"            &%%%%%%%%%    %%%%%$$$$$           "#,
    r#"              %%%%%%%%%%%%%%%%%%$$             "#,
    r#"                %%%%%%%%%%%%%%%%%              "#,
    r#"                    %%%%%%%%%%                 "#,
];

pub(crate) const GOOGLE_STYLE: [&str; LUT_SIZE] = [RED, BLUE, GREEN, YELLOW, BLACK];
pub(crate) const GOOGLE_STYLE_ALT: [&str; LUT_SIZE] =
    [BRIGHT_RED, BRIGHT_BLUE, BRIGHT_GREEN, BRIGHT_YELLOW, BLACK];

pub(crate) const INTEL: &[&str] = &[
    r#"   $$$                                  ###    "#,
    r#"   $$$                                  ###    "#,
    r#"         ## ####     ###      ######    ###    "#,
    r#"   ###   #########   #####  ####  ####  ###    "#,
    r#"   ###   ###    ###  ###    ###    ###  ###    "#,
    r#"   ###   ###    ###  ###    ##########  ###    "#,
    r#"   ###   ###    ###  ###    ###         ###    "#,
    r#"   ###   ###    ###  ###     ########   ###    "#,
    r#"   ###   ###    ###   #####    ####     ###    "#,
];

pub(crate) const INTEL_STYLE: [&str; LUT_SIZE] = [WHITE, CYAN, "", "", BLACK];
pub(crate) const INTEL_STYLE_ALT: [&str; LUT_SIZE] = [BRIGHT_WHITE, BRIGHT_CYAN, "", "", BLACK];

pub(crate) const NVIDIA: &[&str] = &[
    r#"                      #########################"#,
    r#"                ######      ###################"#,
    r#"             ###      #####      ##############"#,
    r#"          ####   #####    #####     ###########"#,
    r#"        ####   ####   #       ####   ##########"#,
    r#"      ####   ####     ###     ####    #########"#,
    r#"       ####   ####    #########     ###########"#,
    r#"         ###    ###   #####      ####   #######"#,
    r#"          #####   ####       #####      #######"#,
    r#"            #####     ########       ##########"#,
    r#"                ######          ###############"#,
    r#"                      #########################"#,
];

pub(crate) const NVIDIA_STYLE: [&str; LUT_SIZE] = [GREEN, "", "", "", BLACK];
pub(crate) const NVIDIA_STYLE_ALT: [&str; LUT_SIZE] = [BRIGHT_GREEN, "", "", "", BLACK];

pub(crate) const AMD: &[&str] = &[
    r#"    ###     ###      ### #########    $$$$$$$$$"#,
    r#"   #####    #####  ##### ###    ###     $$$$$$$"#,
    r#"  ### ###   ############ ###     ###   $    $$$"#,
    r#" ###   ###  ###  ##  ### ###     ###  $$    $$$"#,
    r#"########### ###      ### ###    ###  $$$$$$$ $$"#,
    r#"###     ### ###      ### #########   $$$$$    $"#,
];

pub(crate) const AMD_STYLE: [&str; LUT_SIZE] = [WHITE, GREEN, "", "", BLACK];
pub(crate) const AMD_STYLE_ALT: [&str; LUT_SIZE] = [BRIGHT_WHITE, BRIGHT_GREEN, "", "", BLACK];

pub(crate) const ARM: &[&str] = &[
    r#"          ###     ########   ###      ###      "#,
    r#"         #####    ###   ###  #####  #####      "#,
    r#"        ### ###   ########   ############      "#,
    r#"       ###   ###  ### ####   ###  ##  ###      "#,
    r#"      ########### ###   ###  ###      ###      "#,
    r#"      ###     ### ###    ### ###      ###      "#,
];

pub(crate) const ARM_STYLE: [&str; LUT_SIZE] = [RED, "", "", "", BLACK];
pub(crate) const ARM_STYLE_ALT: [&str; LUT_SIZE] = [BRIGHT_RED, "", "", "", BLACK];

// pub(crate) const MESA: &[&str] = &[
//     r#"   ###      ### $$$$$$$$ %%%%%%%%     &&&      "#,
//     r#"   #####  ##### $$$      %%          &&&&&     "#,
//     r#"   ############ $$$$$$$$ %%%%%%%%   &&& &&&    "#,
//     r#"   ###  ##  ### $$$            %%  &&&&&&&&&   "#,
//     r#"   ###      ### $$$$$$$$ %%%%%%%% &&&     &&&  "#,
// ];

// pub(crate) const MESA_STYLE: [&str; LUT_SIZE] = [BLUE, RED, GREEN, WHITE, BLACK];

pub(crate) const MICROSOFT: &[&str] = &[
    r#"                         "#,
    r#" ########### $$$$$$$$$$$ "#,
    r#" ########### $$$$$$$$$$$ "#,
    r#" ########### $$$$$$$$$$$ "#,
    r#" ########### $$$$$$$$$$$ "#,
    r#" ########### $$$$$$$$$$$ "#,
    r#" ########### $$$$$$$$$$$ "#,
    r#"                         "#,
    r#" %%%%%%%%%%% &&&&&&&&&&& "#,
    r#" %%%%%%%%%%% &&&&&&&&&&& "#,
    r#" %%%%%%%%%%% &&&&&&&&&&& "#,
    r#" %%%%%%%%%%% &&&&&&&&&&& "#,
    r#" %%%%%%%%%%% &&&&&&&&&&& "#,
    r#" %%%%%%%%%%% &&&&&&&&&&& "#,
    r#"                         "#,
];

pub(crate) const MICROSOFT_STYLE: [&str; LUT_SIZE] = [RED, GREEN, BLUE, YELLOW, ""];
pub(crate) const MICROSOFT_STYLE_ALT: [&str; LUT_SIZE] =
    [BRIGHT_RED, BRIGHT_GREEN, BRIGHT_BLUE, BRIGHT_YELLOW, ""];

pub(crate) const QUALCOMM: &[&str] = &[
    r#"         ########         "#,
    r#"      ##############      "#,
    r#"    ####          ####    "#,
    r#"  ####              ####  "#,
    r#" ####                #### "#,
    r#" ###                  ### "#,
    r#"####                  ####"#,
    r#"####                  ####"#,
    r#" ###                  ### "#,
    r#" ####          ###   #### "#,
    r#"  ####          ### ####  "#,
    r#"    ####         #####    "#,
    r#"      ###############     "#,
    r#"         ########  ###    "#,
    r#"                    ###   "#,
];

pub(crate) const QUALCOMM_STYLE: [&str; LUT_SIZE] = [BLUE, "", "", "", ""];
pub(crate) const QUALCOMM_STYLE_ALT: [&str; LUT_SIZE] = [BRIGHT_BLUE, "", "", "", ""];
