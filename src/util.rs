use std::{env, process};

use colored::{ColoredString, Colorize};
use log::error;

///Returns if the terminal supports truecolor mode.
///
/// It checks the `COLORTERM` environnement variable,
/// if it is either set to
/// `truecolor` or `24bit` true is returned.
///
/// In all other cases false will be returned.
///
/// # Examples
/// ```
/// //only true when run in a shell that supports true color
/// let color_support = supports_truecolor();
/// assert!(color_support);
/// ```
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
        assert!(supports_truecolor());
    }

    #[test]
    fn true_when_env_is_24bit() {
        env::set_var("COLORTERM", "24bit");
        assert!(supports_truecolor());
    }

    #[test]
    fn false_with_different_env() {
        env::set_var("COLORTERM", "asdas");
        assert!(!supports_truecolor());
    }
}

///Remap a value from one range to another.
///
/// If the value is outside of the specified range, it will still be
/// converted as if it was in the range. This means it could be much larger or smaller than expected.
/// This can be fixed by using the `clamp` function after the remapping.
///
///# Examples
/// ```
/// let remapped = map_range((0f64, 10f64), (0f64, 20f64), 2f64)
/// assert_eq!(4f64, remapped);
/// ```
pub fn map_range(from_range: (f64, f64), to_range: (f64, f64), value: f64) -> f64 {
    to_range.0 + (value - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

#[cfg(test)]
mod test_map_range {
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

///Converts the given input string to an ansi colored string
///
/// It tries to match the ANSI-Color as closely as possible by calculating the distance between all
/// 8 colors and the given input color from `r`, `b` and `b`, then returning the nearest.
/// It will not be 100% accurate, since every terminal has slightly different
/// ANSI-Colors. It used the VGA-Colors as ANSI-Color.
///
/// # Examples
/// ```
/// //convert black to ansi black color
/// assert_eq!("input".black(), rgb_to_ansi("input", 0, 0, 0));
/// ```
pub fn rgb_to_ansi(input: &str, r: u8, g: u8, b: u8) -> ColoredString {
    //get rgb values and convert them to i32, since later on the could negative when subtracting
    let r = r as i32;
    let g = g as i32;
    let b = b as i32;

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
        assert_eq!("input".black(), rgb_to_ansi("input", 0, 0, 0));
        //convert red to ansi red color
        assert_eq!("input".red(), rgb_to_ansi("input", 170, 0, 0));
        //convert green to ansi green color
        assert_eq!("input".green(), rgb_to_ansi("input", 0, 170, 0));
        //convert yellow to ansi yellow color
        assert_eq!("input".yellow(), rgb_to_ansi("input", 170, 85, 0));
        //convert blue to ansi blue color
        assert_eq!("input".blue(), rgb_to_ansi("input", 0, 0, 170));
        //convert magenta to ansi magenta color
        assert_eq!("input".magenta(), rgb_to_ansi("input", 170, 0, 170));
        //convert cyan to ansi cyan color
        assert_eq!("input".cyan(), rgb_to_ansi("input", 0, 170, 170));
        //convert white to ansi white color
        assert_eq!("input".white(), rgb_to_ansi("input", 170, 170, 170));
    }

    #[test]
    fn convert_vga_bright_values() {
        //convert bright black to ansi bright black color
        assert_eq!("input".bright_black(), rgb_to_ansi("input", 128, 128, 128));
        //convert bright red to ansi bright red color
        assert_eq!("input".bright_red(), rgb_to_ansi("input", 255, 0, 0));
        //convert bright green to ansi bright green color
        assert_eq!("input".bright_green(), rgb_to_ansi("input", 0, 255, 0));
        //convert bright yellow to ansi bright yellow color
        assert_eq!("input".bright_yellow(), rgb_to_ansi("input", 255, 255, 0));
        //convert bright blue to ansi bright blue color
        assert_eq!("input".bright_blue(), rgb_to_ansi("input", 0, 0, 255));
        //convert bright magenta to ansi bright magenta color
        assert_eq!("input".bright_magenta(), rgb_to_ansi("input", 255, 0, 255));
        //convert bright cyan to ansi bright cyan color
        assert_eq!("input".bright_cyan(), rgb_to_ansi("input", 0, 255, 255));
        //convert bright white to ansi bright white color
        assert_eq!("input".bright_white(), rgb_to_ansi("input", 255, 255, 255));
    }

    #[test]
    fn rgb_blue() {
        //convert a blue rgb tone to ansi blue
        assert_eq!("input".blue(), rgb_to_ansi("input", 0, 0, 88));
    }
}

///Function for fatal errors.
///
///A fatal error is an error, from which the program can no recover, meaning the only option left ist to print
/// an error message letting the user know what went wrong. For example if a non-existing file was passed in,
/// this program can not work correctly and should print an error message and exit.
///
/// This function will print the passed in error message as well as a exit message, then it will exit the program with the exit code.
/// If non is specified, it will use exit code 1 by default.
/// A list of exit code can be found here: <https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html>
///
/// # Examples
/// ```
/// let f = File::open("hello.txt");
/// let f = match f {
///     Ok(file) => file,
///     Err(error) => fatal_error(error.to_string().as_str(), Some(66)),
/// };
/// ```
pub fn fatal_error(message: &str, code: Option<i32>) -> ! {
    //This function never returns, since it always exit the program
    error!("{}", message);
    error!("Artem exited with code: {}", code.unwrap_or(1));
    process::exit(code.unwrap_or(1));
}

/// Calculate image dimension related values.
///
/// This calculates the number of columns, rows, and the tile dimensions (tile_width, tile_height) for these
/// values based on a target_size. It returns them as a tuple, the elements are in the previously named order.
/// The dimension property can be used to change what dimension will be scaled. Since terminal character are a bit higher the wide,
/// Width and Height of the output needs to be based on either one, so the other can be calculated.
///
/// # Examples
/// ```
/// assert_eq!(
/// (100, 46, 5, 11),
/// //image with a size of 512x512, split into 100 columns with no border
/// calculate_dimensions(100, 512, 512, 0.42, false, ResizingDimension::Width));
/// ```
pub fn calculate_dimensions(
    target_size: u32,
    height: u32,
    width: u32,
    scale: f64,
    border: bool,
    dimension: ResizingDimension,
) -> (u32, u32, u32, u32) {
    match dimension {
        ResizingDimension::Width => {
            //calculate dimensions based on columns
            let mut columns = if width > target_size {
                target_size
            } else {
                width
            };

            if border {
                //remove a bit of space for the border
                columns = columns.saturating_sub(2);
            }

            //calculate tiles
            let tile_width = width / columns;
            let tile_height = (tile_width as f64 / scale).floor() as u32;

            let rows = height / tile_height;

            (columns, rows, tile_width, tile_height)
        }
        ResizingDimension::Height => {
            let rows = if height > target_size {
                target_size
            } else {
                height
            };
            //calculate tiles
            let tile_height = height / rows;
            let tile_width = (tile_height as f64 * scale).ceil() as u32;

            let mut columns = width / tile_width;

            if border {
                //remove a bit of space for the border
                columns = columns.saturating_sub(2);
            }

            (columns, rows, tile_width, tile_height)
        }
    }
}

#[cfg(test)]
mod test_calculate_dimensions {
    use super::*;

    #[test]
    fn calculate_dimensions_width() {
        assert_eq!(
            (100, 46, 5, 11),
            calculate_dimensions(100, 512, 512, 0.42, false, ResizingDimension::Width)
        );
    }

    #[test]
    fn calculate_dimensions_width_119() {
        assert_eq!(
            (119, 56, 4, 9),
            calculate_dimensions(119, 512, 512, 0.42, false, ResizingDimension::Width)
        );
    }

    #[test]
    fn calculate_dimensions_height() {
        assert_eq!(
            (170, 100, 3, 5),
            calculate_dimensions(100, 512, 512, 0.42, false, ResizingDimension::Height)
        );
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_height_zero() {
        calculate_dimensions(0, 512, 512, 0.42, false, ResizingDimension::Height);
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_width_zero() {
        calculate_dimensions(0, 512, 512, 0.42, false, ResizingDimension::Width);
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_img_width_zero() {
        calculate_dimensions(100, 512, 0, 0.42, false, ResizingDimension::Width);
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_img_height_zero() {
        calculate_dimensions(100, 0, 512, 0.42, false, ResizingDimension::Height);
    }

    #[test]
    fn calculate_dimensions_scale_zero() {
        assert_eq!(
            (100, 0, 5, 4294967295),
            calculate_dimensions(100, 512, 512, 0f64, false, ResizingDimension::Width)
        );
    }

    #[test]
    fn calculate_border_smaller_columns() {
        assert_eq!(
            (98, 0, 5, 4294967295),
            calculate_dimensions(100, 512, 512, 0f64, true, ResizingDimension::Width)
        );
    }
}

///Preferred image resize direction
///
///This changes which dimensions should be used when resizing the image.
///For example, to fully use one dimension (e.g. width), the height can not be scaled
///up as well, since it already would be larger than the maximum terminal height.
///By default width will be used.
///
/// # Examples
/// ```
/// assert_eq!(ResizingDimension::Width, ResizingDimension::default());
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ResizingDimension {
    Width,
    Height,
}
//Implement `Default` as Width
impl Default for ResizingDimension {
    fn default() -> Self {
        ResizingDimension::Width
    }
}

#[cfg(test)]
mod test_dimensions_enum {
    use super::*;

    #[test]
    fn default_is_width() {
        assert_eq!(ResizingDimension::Width, ResizingDimension::default());
    }
}

/// Iterator from inclusive start to exclusive end.
///
/// Returns a iterator from start to end - 1. If `rev` is set to true,
/// it will be iterating in reverse.
///
/// # Examples
///
/// ## Forward Iterator
/// ```
/// let mut range = range(0, 2, false);
/// assert_eq!(Some(0), range.next());
/// assert_eq!(Some(1), range.next());
/// assert_eq!(None, range.next());
/// ```
///
/// ## Reverse Iterator
/// ```
/// let mut range = range(0, 2, true);
/// assert_eq!(Some(1), range.next());
/// assert_eq!(Some(0), range.next());
/// assert_eq!(None, range.next());
/// ```
///
pub fn range(start: u32, end: u32, rev: bool) -> impl Iterator<Item = u32> {
    let (mut r_start, step) = if rev {
        (end.saturating_sub(1), u32::max_value())
    } else {
        (start, 1)
    };

    std::iter::repeat_with(move || {
        let tmp = r_start;
        r_start = r_start.wrapping_add(step);
        tmp
    })
    .take(end as usize - start as usize)
}

#[cfg(test)]
mod test_range {
    use super::*;

    #[test]
    fn create_range_0_2() {
        let mut range = range(0, 2, false);
        assert_eq!(Some(0), range.next());
        assert_eq!(Some(1), range.next());
        assert_eq!(None, range.next());
    }

    #[test]
    fn create_range_2_0() {
        let mut range = range(0, 2, true);
        assert_eq!(Some(1), range.next());
        assert_eq!(Some(0), range.next());
        assert_eq!(None, range.next());
    }
}
