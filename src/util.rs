use std::env;

use colored::{ColoredString, Colorize};

///Checks if the terminal supports truecolor mode.
/// Returns false if not.
pub fn supports_truecolor() -> bool {
    match env::var("COLORTERM") {
        Ok(var) => var.contains("truecolor") || var.contains("24bit"),
        Err(_) => false, //not found, true colors are not supported
    }
}

#[cfg(test)]
mod test_color_support {
    use super::*;

    #[test]
    fn true_when_env_is_truecolor() {
        env::set_var("COLORTERM", "truecolor");
        assert_eq!(true, supports_truecolor());
    }

    #[test]
    fn true_when_env_is_24bit() {
        env::set_var("COLORTERM", "24bit");
        assert_eq!(true, supports_truecolor());
    }

    #[test]
    fn false_with_different_env() {
        env::set_var("COLORTERM", "asdas");
        assert_eq!(false, supports_truecolor());
    }
}

///Remap a value from one range to another.
pub fn map_range(from_range: (f64, f64), to_range: (f64, f64), value: f64) -> f64 {
    to_range.0 + (value - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

#[cfg(test)]
mod test_range {
    use super::*;

    #[test]
    fn remap_values() {
        //remap 2 to 4
        assert_eq!(4f64, map_range((0f64, 10f64), (0f64, 20f64), 2f64));
    }

    #[test]
    fn remap_values_above_range() {
        //remap 21 to 42, since the value will be doubled
        assert_eq!(42f64, map_range((0f64, 10f64), (0f64, 20f64), 21f64));
    }

    #[test]
    fn remap_values_below_range() {
        //remap -1 to -2, since the value will be doubled
        assert_eq!(-2f64, map_range((0f64, 10f64), (0f64, 20f64), -1f64));
    }
}

///Converts the given input string to an ansi colored string, somewhat matching given rgb values
/// Since only 8 ansi colors are supported
pub fn convert_rgb_ansi(
    input: &str,
    input_color_r: u8,
    input_color_g: u8,
    input_color_b: u8,
) -> ColoredString {
    //get rgb values and convert them to i32, since later on the could negative when subtracting
    let r = input_color_r as i32;
    let g = input_color_g as i32;
    let b = input_color_b as i32;

    //vga colors as example ansi color
    //from https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
    let vga_colors = [
        [0, 0, 0],       //black
        [170, 0, 0],     //red
        [0, 170, 0],     //green
        [170, 85, 0],    //yellow
        [0, 0, 170],     //blue
        [170, 0, 170],   //magenta
        [0, 170, 170],   //cyan
        [170, 170, 170], //white
        [128, 128, 128], //bright black/gray
        [255, 0, 0],     //bright red
        [0, 255, 0],     //bright green
        [255, 255, 0],   //bright yellow
        [0, 0, 255],     //bright blue
        [255, 0, 255],   //bright magenta
        [0, 255, 255],   //bright cyan
        [255, 255, 255], //bright white
    ];

    //find nearest color
    let mut smallest_distance = i32::MAX;
    let mut smallest_distance_index: u8 = 7;
    //maybe there is a better method for this
    for (index, vga_color) in vga_colors.iter().enumerate() {
        let distance =
            (r - vga_color[0]).pow(2) + (g - vga_color[1]).pow(2) + (b - vga_color[2]).pow(2);

        if distance < smallest_distance {
            smallest_distance = distance;
            smallest_distance_index = index as u8;
        }
    }

    //convert string to matching color
    match smallest_distance_index {
        0 => input.black(),
        1 => input.red(),
        2 => input.green(),
        3 => input.yellow(),
        4 => input.blue(),
        5 => input.magenta(),
        6 => input.cyan(),
        7 => input.white(),
        8 => input.bright_black(),
        9 => input.bright_red(),
        10 => input.bright_green(),
        11 => input.bright_yellow(),
        12 => input.bright_blue(),
        13 => input.bright_magenta(),
        14 => input.bright_cyan(),
        15 => input.bright_white(),
        _ => input.normal(),
    }
}

#[cfg(test)]
mod test_convert_rgb_ansi {
    use super::*;

    #[test]
    fn convert_vga_normal_values() {
        //convert black to ansi black color
        assert_eq!("input".black(), convert_rgb_ansi("input", 0, 0, 0));
        //convert red to ansi red color
        assert_eq!("input".red(), convert_rgb_ansi("input", 170, 0, 0));
        //convert green to ansi green color
        assert_eq!("input".green(), convert_rgb_ansi("input", 0, 170, 0));
        //convert yellow to ansi yellow color
        assert_eq!("input".yellow(), convert_rgb_ansi("input", 170, 85, 0));
        //convert blue to ansi blue color
        assert_eq!("input".blue(), convert_rgb_ansi("input", 0, 0, 170));
        //convert magenta to ansi magenta color
        assert_eq!("input".magenta(), convert_rgb_ansi("input", 170, 0, 170));
        //convert cyan to ansi cyan color
        assert_eq!("input".cyan(), convert_rgb_ansi("input", 0, 170, 170));
        //convert white to ansi white color
        assert_eq!("input".white(), convert_rgb_ansi("input", 170, 170, 170));
    }

    #[test]
    fn convert_vga_bright_values() {
        //convert bright black to ansi bright black color
        assert_eq!(
            "input".bright_black(),
            convert_rgb_ansi("input", 128, 128, 128)
        );
        //convert bright red to ansi bright red color
        assert_eq!("input".bright_red(), convert_rgb_ansi("input", 255, 0, 0));
        //convert bright green to ansi bright green color
        assert_eq!("input".bright_green(), convert_rgb_ansi("input", 0, 255, 0));
        //convert bright yellow to ansi bright yellow color
        assert_eq!(
            "input".bright_yellow(),
            convert_rgb_ansi("input", 255, 255, 0)
        );
        //convert bright blue to ansi bright blue color
        assert_eq!("input".bright_blue(), convert_rgb_ansi("input", 0, 0, 255));
        //convert bright magenta to ansi bright magenta color
        assert_eq!(
            "input".bright_magenta(),
            convert_rgb_ansi("input", 255, 0, 255)
        );
        //convert bright cyan to ansi bright cyan color
        assert_eq!(
            "input".bright_cyan(),
            convert_rgb_ansi("input", 0, 255, 255)
        );
        //convert bright white to ansi bright white color
        assert_eq!(
            "input".bright_white(),
            convert_rgb_ansi("input", 255, 255, 255)
        );
    }
}