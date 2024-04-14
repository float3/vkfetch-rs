pub(crate) const LUT_SIZE: usize = 5;

pub(crate) const CHARS: [char; LUT_SIZE] = ['#', '$', '%', '&', '@'];

pub(crate) const BLACK: &str = "\x1B[30m";
pub(crate) const RED: &str = "\x1B[31m";
// pub(crate) const BRIGHT_RED: &str = "\x1B[91m";
pub(crate) const GREEN: &str = "\x1B[32m";
// pub(crate) const BRIGHT_GREEN: &str = "\x1B[92m";
pub(crate) const YELLOW: &str = "\x1B[33m";
// pub(crate) const BRIGHT_YELLOW: &str = "\x1B[93m";
pub(crate) const BLUE: &str = "\x1B[34m";
// pub(crate) const BRIGHT_BLUE: &str = "\x1B[94m";
pub(crate) const MAGENTA: &str = "\x1B[35m";
// pub(crate) const BRIGHT_MAGENTA: &str = "\x1B[95m";
pub(crate) const CYAN: &str = "\x1B[36m";
// pub(crate) const BRIGHT_CYAN: &str = "\x1B[96m";
pub(crate) const WHITE: &str = "\x1B[37m";
// pub(crate) const BRIGHT_WHITE: &str = "\x1B[97m";

pub(crate) const UNKNOWN: &[&str] = &[
    r#"         ################                    "#,
    r#"     ##########################              "#,
    r#"   #############         ##########          "#,
    r#"  #########                     ######       "#,
    r#"  #######     ####      ####         ####    "#,
    r#"   #####      ####      ####            ###  "#,
    r#"    ####       ####    ####               ## "#,
    r#"     ####      ####    ####                 #"#,
    r#"       ###      ####  ####                   "#,
    r#"         ##     ####  ####                   "#,
    r#"                 ########                    "#,
    r#"                 ########                    "#,
    r#"                  ######                     "#,
];

pub(crate) const UNKOWN_STYLE: [&str; LUT_SIZE] = [RED, "", "", "", BLACK];

pub(crate) const APPLE: &[&str] = &[
    r#"                 ###        "#,
    r#"               ####         "#,
    r#"              ####          "#,
    r#"     ####### ###  #######   "#,
    r#"   ######################## "#,
    r#"  ########################  "#,
    r#" #######################    "#,
    r#"#######################     "#,
    r#"#######################     "#,
    r#"########################    "#,
    r#" #########################  "#,
    r#"  ##########################"#,
    r#"    ######################  "#,
    r#"     ###################    "#,
    r#"       ######    #####      "#,
];

pub(crate) const APPLE_STYLE: [&str; LUT_SIZE] = [WHITE, "", "", "", BLACK];

pub(crate) const GOOGLE: &[&str] = &[
    r#"            #########          "#,
    r#"        #################      "#,
    r#"      #####################    "#,
    r#"    &#########    #######      "#,
    r#"  &&&&#####          ##        "#,
    r#" &&&&&&&#                      "#,
    r#" &&&&&&&                       "#,
    r#"&&&&&&&          $$$$$$$$$$$$$$"#,
    r#"&&&&&&&          $$$$$$$$$$$$$$"#,
    r#"&&&&&&&          $$$$$$$$$$$$$$"#,
    r#" &&&&&&&                $$$$$$$"#,
    r#" &&&&&&&%              $$$$$$$ "#,
    r#"  &&&&%%%%%          %$$$$$$$$ "#,
    r#"    &%%%%%%%%%    %%%%%$$$$$   "#,
    r#"      %%%%%%%%%%%%%%%%%%$$     "#,
    r#"        %%%%%%%%%%%%%%%%%      "#,
    r#"            %%%%%%%%%%         "#,
];

pub(crate) const GOOGLE_STYLE: [&str; LUT_SIZE] = [RED, BLUE, GREEN, YELLOW, BLACK];

pub(crate) const INTEL: &[&str] = &[
    r#"$$$                                  ###"#,
    r#"$$$                                  ###"#,
    r#"      ## ####     ###      ######    ###"#,
    r#"###   #########   #####  ####  ####  ###"#,
    r#"###   ###    ###  ###    ###    ###  ###"#,
    r#"###   ###    ###  ###    ##########  ###"#,
    r#"###   ###    ###  ###    ###         ###"#,
    r#"###   ###    ###  ###     ########   ###"#,
    r#"###   ###    ###   #####    ####     ###"#,
];

pub(crate) const INTEL_STYLE: [&str; LUT_SIZE] = [WHITE, CYAN, "", "", BLACK];

pub(crate) const NVIDIA: &[&str] = &[
    r#"                     ########################"#,
    r#"               ######      ##################"#,
    r#"            ###      #####      #############"#,
    r#"         ####   #####    #####     ##########"#,
    r#"       ####   ####   #       ####   #########"#,
    r#"     ####   ####     ###     ####    ########"#,
    r#"      ####   ####    #########     ##########"#,
    r#"        ###    ###   #####      ####   ######"#,
    r#"         #####   ####       #####      ######"#,
    r#"           #####     ########       #########"#,
    r#"               ######          ##############"#,
    r#"                     ########################"#,
];

pub(crate) const NVIDIA_STYLE: [&str; LUT_SIZE] = [GREEN, "", "", "", BLACK];

pub(crate) const AMD: &[&str] = &[
    r#"    ###     ###      ### #########    $$$$$$$$$"#,
    r#"   #####    #####  ##### ###    ###     $$$$$$$"#,
    r#"  ### ###   ############ ###     ###   $    $$$"#,
    r#" ###   ###  ###  ##  ### ###     ###  $$    $$$"#,
    r#"########### ###      ### ###    ###  $$$$$$$ $$"#,
    r#"###     ### ###      ### #########   $$$$$    $"#,
];

pub(crate) const AMD_STYLE: [&str; LUT_SIZE] = [WHITE, GREEN, "", "", BLACK];

pub(crate) const MESA: &[&str] = &[];

pub(crate) const MESA_STYLE: [&str; LUT_SIZE] = ["", "", "", "", BLACK];
