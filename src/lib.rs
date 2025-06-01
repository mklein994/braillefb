//! A framebuffer that takes a `&[bool]` slice and returns 2x4 "dot" (pixel) [braille `char`s][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Braille_Patterns
//!
//!```text
//!$ cargo run --example mandelbrot --quiet
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠒⣾⣿⣷⣤⠄⠀⠀⠀⠀⠀⠀⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⠏⠀⠀⢀⠀⠀⠀⠀⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢤⣴⣀⣬⣧⣷⣷⣾⣿⣿⣿⣷⣧⣾⢀⢀⡠⠀⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢙⣿⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣾⣿⡯⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠵⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡦⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⢄⠀⠀⠀⠀⠀⣲⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠈
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣧⣵⣿⣦⣧⣄⠀⢀⣽⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⡀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⢀⣲⣿⣿⣿⣿⣿⣿⣿⣷⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠧⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢴⣶⣦⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀
//!⠈⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⢽⣿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣄⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠐⣻⣿⣿⣿⣿⣿⣿⣿⣿⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡥⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡿⢿⣿⡿⡿⠟⠁⠘⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡷⠂
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠊⠁⠁⠝⠀⠁⠀⠀⠀⣻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢬⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⠉⠈
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠐⢙⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡦⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠽⢿⠚⠿⡿⣿⣿⣿⣿⣿⣿⣿⡿⣿⠙⠘⠫⠉⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠁⢡⣾⣿⣿⡍⠁⠁⠘⠀⠀⠀⠀⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣈⣿⣿⣿⠿⠄⠀⠀⠀⠀⠀⠀⠀⠀
//!⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠽⠓⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//!```
//!
//! # Example
//!
//! ```
//! // ⣇⠽
//! let framebuffer = vec![
//!     true, false, true, true,
//!     true, false, false, true,
//!     true, false, true, true,
//!     true, true, false, false,
//! ];
//!
//! let f = braillefb::Framebuffer::new(&framebuffer, 4, 4);
//!
//! // Get individual braille `char`s
//! assert_eq!(Some('⣇'), f.get(0));
//! assert_eq!('⠽', f[1]);
//!
//! // As an iterator
//! let output: String = f.into_iter().collect();
//! assert_eq!("⣇⠽\n", &output);
//!
//! // From `[bool; 8]`
//! assert_eq!(
//!     '⢗',
//!     braillefb::to_char([
//!         true, false,
//!         true, true,
//!         true, false,
//!         false, true,
//!     ])
//! );
//! ```

use std::fmt;
use std::ops::Index;

// https://en.wikipedia.org/wiki/Braille_Patterns
//
// 1 4
// 2 5
// 3 6
// 7 8
const BIT_OFFSETS: [(usize, usize); 8] = [
    (1, 3), // 8
    (0, 3), // 7
    (1, 2), // 6
    (1, 1), // 5
    (1, 0), // 4
    (0, 2), // 3
    (0, 1), // 2
    (0, 0), // 1
];
const CHAR_WIDTH: usize = 2;
const CHAR_HEIGHT: usize = 4;

// Hardcode the list a `char`s so we can return static references from the `Index` impl
const CHARS: [char; 256] = [
    '⠀', '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇', '⠈', '⠉', '⠊', '⠋', '⠌', '⠍', '⠎', '⠏', '⠐', '⠑', '⠒',
    '⠓', '⠔', '⠕', '⠖', '⠗', '⠘', '⠙', '⠚', '⠛', '⠜', '⠝', '⠞', '⠟', '⠠', '⠡', '⠢', '⠣', '⠤', '⠥',
    '⠦', '⠧', '⠨', '⠩', '⠪', '⠫', '⠬', '⠭', '⠮', '⠯', '⠰', '⠱', '⠲', '⠳', '⠴', '⠵', '⠶', '⠷', '⠸',
    '⠹', '⠺', '⠻', '⠼', '⠽', '⠾', '⠿', '⡀', '⡁', '⡂', '⡃', '⡄', '⡅', '⡆', '⡇', '⡈', '⡉', '⡊', '⡋',
    '⡌', '⡍', '⡎', '⡏', '⡐', '⡑', '⡒', '⡓', '⡔', '⡕', '⡖', '⡗', '⡘', '⡙', '⡚', '⡛', '⡜', '⡝', '⡞',
    '⡟', '⡠', '⡡', '⡢', '⡣', '⡤', '⡥', '⡦', '⡧', '⡨', '⡩', '⡪', '⡫', '⡬', '⡭', '⡮', '⡯', '⡰', '⡱',
    '⡲', '⡳', '⡴', '⡵', '⡶', '⡷', '⡸', '⡹', '⡺', '⡻', '⡼', '⡽', '⡾', '⡿', '⢀', '⢁', '⢂', '⢃', '⢄',
    '⢅', '⢆', '⢇', '⢈', '⢉', '⢊', '⢋', '⢌', '⢍', '⢎', '⢏', '⢐', '⢑', '⢒', '⢓', '⢔', '⢕', '⢖', '⢗',
    '⢘', '⢙', '⢚', '⢛', '⢜', '⢝', '⢞', '⢟', '⢠', '⢡', '⢢', '⢣', '⢤', '⢥', '⢦', '⢧', '⢨', '⢩', '⢪',
    '⢫', '⢬', '⢭', '⢮', '⢯', '⢰', '⢱', '⢲', '⢳', '⢴', '⢵', '⢶', '⢷', '⢸', '⢹', '⢺', '⢻', '⢼', '⢽',
    '⢾', '⢿', '⣀', '⣁', '⣂', '⣃', '⣄', '⣅', '⣆', '⣇', '⣈', '⣉', '⣊', '⣋', '⣌', '⣍', '⣎', '⣏', '⣐',
    '⣑', '⣒', '⣓', '⣔', '⣕', '⣖', '⣗', '⣘', '⣙', '⣚', '⣛', '⣜', '⣝', '⣞', '⣟', '⣠', '⣡', '⣢', '⣣',
    '⣤', '⣥', '⣦', '⣧', '⣨', '⣩', '⣪', '⣫', '⣬', '⣭', '⣮', '⣯', '⣰', '⣱', '⣲', '⣳', '⣴', '⣵', '⣶',
    '⣷', '⣸', '⣹', '⣺', '⣻', '⣼', '⣽', '⣾', '⣿',
];

const OCTANTS: [char; 256] = [
    ' ', '𜺨', '𜴀', '▘', '𜴉', '𜴊', '🯦', '𜴍', '𜺫', '🮂', '𜴁', '𜴂', '𜴋', '𜴌', '𜴎', '𜴏', '𜴃', '𜴄', '𜴆',
    '𜴇', '𜴐', '𜴑', '𜴔', '𜴕', '▝', '𜴅', '𜴈', '▀', '𜴒', '𜴓', '𜴖', '𜴗', '𜴘', '𜴙', '𜴜', '𜴝', '𜴧', '𜴨',
    '𜴫', '𜴬', '𜴚', '𜴛', '𜴞', '𜴟', '𜴩', '𜴪', '𜴭', '𜴮', '🯧', '𜴠', '𜴣', '𜴤', '𜴯', '𜴰', '𜴳', '𜴴', '𜴡',
    '𜴢', '𜴥', '𜴦', '𜴱', '𜴲', '𜴵', '🮅', '𜺣', '𜴶', '𜴹', '𜴺', '▖', '𜵅', '𜵈', '▌', '𜴷', '𜴸', '𜴻', '𜴼',
    '𜵆', '𜵇', '𜵉', '𜵊', '𜴽', '𜴾', '𜵁', '𜵂', '𜵋', '𜵌', '𜵎', '𜵏', '𜴿', '𜵀', '𜵃', '𜵄', '▞', '𜵍', '𜵐',
    '▛', '𜵑', '𜵒', '𜵕', '𜵖', '𜵡', '𜵢', '𜵥', '𜵦', '𜵓', '𜵔', '𜵗', '𜵘', '𜵣', '𜵤', '𜵧', '𜵨', '𜵙', '𜵚',
    '𜵝', '𜵞', '𜵩', '𜵪', '𜵭', '𜵮', '𜵛', '𜵜', '𜵟', '𜵠', '𜵫', '𜵬', '𜵯', '𜵰', '𜺠', '𜵱', '𜵴', '𜵵', '𜶀',
    '𜶁', '𜶄', '𜶅', '𜵲', '𜵳', '𜵶', '𜵷', '𜶂', '𜶃', '𜶆', '𜶇', '𜵸', '𜵹', '𜵼', '𜵽', '𜶈', '𜶉', '𜶌', '𜶍',
    '𜵺', '𜵻', '𜵾', '𜵿', '𜶊', '𜶋', '𜶎', '𜶏', '▗', '𜶐', '𜶓', '▚', '𜶜', '𜶝', '𜶠', '𜶡', '𜶑', '𜶒', '𜶔',
    '𜶕', '𜶞', '𜶟', '𜶢', '𜶣', '𜶖', '𜶗', '𜶙', '𜶚', '𜶤', '𜶥', '𜶨', '𜶩', '▐', '𜶘', '𜶛', '▜', '𜶦', '𜶧',
    '𜶪', '𜶫', '▂', '𜶬', '𜶯', '𜶰', '𜶻', '𜶼', '𜶿', '𜷀', '𜶭', '𜶮', '𜶱', '𜶲', '𜶽', '𜶾', '𜷁', '𜷂', '𜶳',
    '𜶴', '𜶷', '𜶸', '𜷃', '𜷄', '𜷇', '𜷈', '𜶵', '𜶶', '𜶹', '𜶺', '𜷅', '𜷆', '𜷉', '𜷊', '𜷋', '𜷌', '𜷏', '𜷐',
    '▄', '𜷛', '𜷞', '▙', '𜷍', '𜷎', '𜷑', '𜷒', '𜷜', '𜷝', '𜷟', '𜷠', '𜷓', '𜷔', '𜷗', '𜷘', '𜷡', '𜷢', '▆',
    '𜷤', '𜷕', '𜷖', '𜷙', '𜷚', '▟', '𜷣', '𜷥', '█',
];

/// A framebuffer that takes a `&[bool]` slice and returns 2x4 "dot" (pixel) [braille `char`s][1].
///
/// [1]: https://en.wikipedia.org/wiki/Braille_Patterns
///
/// # Example
///
/// ```
/// # use braillefb::Framebuffer;
/// // ⣇⠽
/// let framebuffer = vec![
///     true, false, true, true,
///     true, false, false, true,
///     true, false, true, true,
///     true, true, false, false,
/// ];
///
/// let f = Framebuffer::new(&framebuffer, 4, 4);
///
/// // Get individual braille `char`s
/// assert_eq!(Some('⣇'), f.get(0));
///
/// // As an iterator
/// let output: String = f.into_iter().collect();
/// assert_eq!("⣇⠽\n", &output);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Framebuffer<'a> {
    framebuffer: &'a [bool],
    width: usize,
    height: usize,
    x_chars_count: usize,
    y_chars_count: usize,
    style: FramebufferStyle,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum FramebufferStyle {
    #[default]
    Braille,
    Octants,
}

impl<'a> Framebuffer<'a> {
    /// Create a Framebuffer instance.
    ///
    /// # Panics
    ///
    /// Panics if length of supplied `framebuffer` slice is not equal to `width * height`.
    pub fn new(framebuffer: &'a [bool], width: usize, height: usize) -> Self {
        assert_eq!(
            framebuffer.len(),
            width * height,
            "supplied slice does not match width * height"
        );

        fn round_up(input: usize, multiple: usize) -> usize {
            ((input + multiple - 1) / multiple) * multiple
        }

        let x_chars_count = (round_up(width, CHAR_WIDTH) / CHAR_WIDTH) + 1; // + 1 for linebreaks
        let y_chars_count = round_up(height, CHAR_HEIGHT) / CHAR_HEIGHT;

        Self {
            framebuffer,
            width,
            height,
            x_chars_count,
            y_chars_count,
            style: FramebufferStyle::default(),
        }
    }

    pub fn with_style(mut self, style: FramebufferStyle) -> Self {
        self.style = style;
        self
    }

    /// Get the nth braille character in the framebuffer.
    ///
    /// # Example
    ///
    /// ```
    /// # use braillefb::Framebuffer;
    /// // ⣇⠽
    /// let framebuffer = vec![
    ///     true, false, true, true,
    ///     true, false, false, true,
    ///     true, false, true, true,
    ///     true, true, false, false,
    /// ];
    ///
    /// let f = Framebuffer::new(&framebuffer, 4, 4);
    ///
    /// assert_eq!(Some('⣇'), f.get(0));
    /// assert_eq!(Some('⠽'), f.get(1));
    /// assert_eq!(Some('\n'), f.get(2)); // Linebreaks are included
    /// assert_eq!(None, f.get(3));
    /// ```
    pub fn get(&self, index: usize) -> Option<char> {
        self.get_inner(index).copied()
    }

    fn get_inner(&self, index: usize) -> Option<&'static char> {
        match self.offsets(index) {
            Offsets::Char(x_offset, y_offset) => Some(get_char(
                self.framebuffer,
                x_offset,
                y_offset,
                self.width,
                self.height,
                self.style,
            )),
            Offsets::Linebreak => Some(&'\n'),
            Offsets::End => None,
        }
    }

    /// Returns the number of braille `chars` across the image including a trailing linebreak.
    ///
    /// # Example
    ///
    /// ```
    /// # use braillefb::Framebuffer;
    /// # let framebuffer = vec![
    /// #     true, false, true, true,
    /// #     true, false, false, true,
    /// #     true, false, true, true,
    /// #     true, true, false, false,
    /// #     true, true, false, true,
    /// #     true, true, false, true,
    /// #     false, false, true, true,
    /// #     true, false, true, false,
    /// # ];
    /// // ⣇⠽
    /// // ⡛⡼
    /// let f = Framebuffer::new(&framebuffer, 4, 8);
    /// assert_eq!("⣇⠽\n⡛⡼\n", &f.to_string());
    /// assert_eq!(3, f.x_chars_count());
    /// ```
    pub fn x_chars_count(&self) -> usize {
        self.x_chars_count
    }

    /// Returns the number of braille `chars` down the image.
    ///
    /// # Example
    ///
    /// ```
    /// # use braillefb::Framebuffer;
    /// # let framebuffer = vec![
    /// #     true, false, true, true,
    /// #     true, false, false, true,
    /// #     true, false, true, true,
    /// #     true, true, false, false,
    /// #     true, true, false, true,
    /// #     true, true, false, true,
    /// #     false, false, true, true,
    /// #     true, false, true, false,
    /// # ];
    /// // ⣇⠽
    /// // ⡛⡼
    /// let f = Framebuffer::new(&framebuffer, 4, 8);
    /// assert_eq!("⣇⠽\n⡛⡼\n", &f.to_string());
    /// assert_eq!(2, f.y_chars_count());
    /// ```
    pub fn y_chars_count(&self) -> usize {
        self.y_chars_count
    }

    /// The number of `char`s (including linebreaks) that can be returned.
    pub fn len(&self) -> usize {
        self.y_chars_count * self.x_chars_count
    }

    pub fn is_empty(&self) -> bool {
        self.framebuffer.is_empty()
    }

    fn offsets(&self, index: usize) -> Offsets {
        if index > 0 && (index + 1) % self.x_chars_count == 0 {
            return Offsets::Linebreak;
        }

        let rows = index / self.x_chars_count;
        let y_offset = rows * CHAR_HEIGHT;

        if y_offset >= self.height {
            return Offsets::End;
        }

        let cols = index % self.x_chars_count;
        let x_offset = cols * CHAR_WIDTH;

        Offsets::Char(x_offset, y_offset)
    }
}

impl fmt::Display for Framebuffer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Index<usize> for Framebuffer<'_> {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_inner(index).unwrap_or_else(|| {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.len(),
                index
            )
        })
    }
}

impl<'a, 'f> IntoIterator for &'a Framebuffer<'f> {
    type Item = char;
    type IntoIter = Iter<'a, 'f>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self,
            index: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Offsets {
    Char(usize, usize),
    Linebreak,
    End,
}

/// An iterator over braille `char`s.
pub struct Iter<'a, 'i> {
    inner: &'a Framebuffer<'i>,
    index: usize,
}

impl<'a, 'i> Iterator for Iter<'a, 'i> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.offsets(self.index) {
            Offsets::Char(x_offset, y_offset) => {
                self.index += 1;
                Some(*get_char(
                    self.inner.framebuffer,
                    x_offset,
                    y_offset,
                    self.inner.width,
                    self.inner.height,
                    self.inner.style,
                ))
            }
            Offsets::Linebreak => {
                self.index += 1;
                Some('\n')
            }
            Offsets::End => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.inner.len();
        (size, Some(size))
    }
}

/// Converts a single 2x4 1-bit array into a braille `char`.
///
/// # Example
///
/// ```
/// assert_eq!(
///     '⢗',
///     braillefb::to_char([
///         true, false,
///         true, true,
///         true, false,
///         false, true,
///     ])
/// );
/// ```
pub fn to_char(f: [bool; 8]) -> char {
    *get_char(&f, 0, 0, CHAR_WIDTH, CHAR_HEIGHT, FramebufferStyle::Braille)
}

/// Converts a single 2x4 1-bit array into an octant `char`.
///
/// # Example
///
/// ```
/// assert_eq!(
///     '𜶍',
///     braillefb::to_char_octants([
///         true, false,
///         true, true,
///         true, false,
///         false, true,
///     ])
/// );
/// ```
pub fn to_char_octants(f: [bool; 8]) -> char {
    *get_char(&f, 0, 0, CHAR_WIDTH, CHAR_HEIGHT, FramebufferStyle::Octants)
}

// The x/y offsets are combined with the BIT_OFFSETS to create a u8 in the order that a
// UTF-8 braille character or octant character is represented
//
// 1 4
// 2 5
// 3 6
// 7 8
//
// 0b00000000
//   12345678
fn get_char(
    framebuffer: &[bool],
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
    style: FramebufferStyle,
) -> &'static char {
    let mut n: u8 = 0;
    for (x, y) in BIT_OFFSETS {
        n <<= 1;
        let xx = x_offset + x;
        let yy = y_offset + y;
        if xx >= width || yy >= height {
            continue;
        }
        n |= framebuffer[xx + yy * width] as u8;
    }
    match style {
        FramebufferStyle::Braille => &CHARS[n as usize],
        FramebufferStyle::Octants => &OCTANTS[n as usize],
    }
}

#[cfg(test)]
mod tests {
    use super::{get_char, to_char, Framebuffer, FramebufferStyle, Offsets};

    macro_rules! framebuffer {
        (#) => {true};
        (.) => {false};
        ($($c:tt)+) => {vec![
            $(framebuffer!($c)),+
        ]}
    }

    #[test]
    fn single_chars() {
        let framebuffer = framebuffer![
            # .
            # #
            . .
            . .
        ];
        let f = Framebuffer::new(&framebuffer, 2, 4);
        assert_eq!(Some('⠓'), f.get(0));
        assert_eq!(Some('\n'), f.get(1));
        assert_eq!(None, f.get(2));

        let framebuffer = framebuffer![
            # .
            # .
            # .
            # #
        ];
        let f = Framebuffer::new(&framebuffer, 2, 4);
        assert_eq!(Some('⣇'), f.get(0));
        assert_eq!(Some('\n'), f.get(1));
        assert_eq!(None, f.get(2));
    }

    #[test]
    fn multiple_chars() {
        // ⣇⠽
        // ⡛⡼
        let framebuffer = framebuffer![
            # . # #
            # . . #
            # . # #
            # # . .
            # # . #
            # # . #
            . . # #
            # . # .
        ];
        let f = Framebuffer::new(&framebuffer, 4, 8);

        assert_eq!(Some('⣇'), f.get(0));
        assert_eq!(Some('⠽'), f.get(1));
        assert_eq!(Some('\n'), f.get(2));
        assert_eq!(Some('⡛'), f.get(3));
        assert_eq!(Some('⡼'), f.get(4));
    }

    #[test]
    fn len() {
        // # #
        // # #
        // # #
        // # #
        // \n
        let framebuffer = vec![false; 2 * 4];
        let f = Framebuffer::new(&framebuffer, 2, 4);
        assert_eq!(2, f.len());

        // # # # #
        // # # # #
        // # # # #
        // # # # #
        // \n
        // # # # #
        // # # # #
        // # # # #
        // # # # #
        // \n
        let framebuffer = vec![false; 4 * 8];
        let f = Framebuffer::new(&framebuffer, 4, 8);
        assert_eq!(6, f.len());

        // # # # .
        // # # # .
        // # # # .
        // # # # .
        // \n
        // # # # .
        // . . . .
        // . . . .
        // . . . .
        // \n
        let framebuffer = vec![false; 3 * 5];
        let f = Framebuffer::new(&framebuffer, 3, 5);
        assert_eq!(6, f.len());
    }

    #[test]
    fn padding() {
        // ⠇⠅
        // ⠉⠁
        let framebuffer = framebuffer![
            # . #
            # . .
            # . #
            . . .
            # # #
        ];
        let f = Framebuffer::new(&framebuffer, 3, 5);

        assert_eq!(Some('⠇'), f.get(0));
        assert_eq!(Some('⠅'), f.get(1));
        assert_eq!(Some('\n'), f.get(2));

        assert_eq!(Some('⠉'), f.get(3));
        assert_eq!(Some('⠁'), f.get(4));
        assert_eq!(Some('\n'), f.get(5));
    }

    #[test]
    fn test_to_char() {
        // ⢗
        let framebuffer = framebuffer![
            # .
            # #
            # .
            . #
        ];
        assert_eq!('⢗', to_char(framebuffer.try_into().unwrap()));
    }

    #[test]
    fn test_get_char() {
        // ⠇⠅
        // ⠉⠁
        let framebuffer = framebuffer![
            # . #
            # . .
            # . #
            . . .
            # # #
        ];

        assert_eq!(
            &'⠇',
            get_char(&framebuffer, 0, 0, 3, 5, FramebufferStyle::Braille)
        );
        assert_eq!(
            &'⠅',
            get_char(&framebuffer, 2, 0, 3, 5, FramebufferStyle::Braille)
        );
        assert_eq!(
            &'⠉',
            get_char(&framebuffer, 0, 4, 3, 5, FramebufferStyle::Braille)
        );
        assert_eq!(
            &'⠁',
            get_char(&framebuffer, 2, 4, 3, 5, FramebufferStyle::Braille)
        );
    }

    #[test]
    fn offsets() {
        // 0 # 1 #
        // # # # #
        // # # # #
        // # # # #
        // 2 \n
        // 3 # 4 #
        // # # # #
        // # # # #
        // # # # #
        // 5 \n
        // 6 # 7 #
        // # # # #
        // # # # #
        // # # # #
        // 8 \n
        fn test(f: Framebuffer) {
            // Row 1
            assert_eq!(Offsets::Char(0, 0), f.offsets(0));
            assert_eq!(Offsets::Char(2, 0), f.offsets(1));
            assert_eq!(Offsets::Linebreak, f.offsets(2));

            // Row 2
            assert_eq!(Offsets::Char(0, 4), f.offsets(3));
            assert_eq!(Offsets::Char(2, 4), f.offsets(4));
            assert_eq!(Offsets::Linebreak, f.offsets(5));

            // Row 3
            assert_eq!(Offsets::Char(0, 8), f.offsets(6));
            assert_eq!(Offsets::Char(2, 8), f.offsets(7));
            assert_eq!(Offsets::Linebreak, f.offsets(8));

            assert_eq!(Offsets::End, f.offsets(9));
        }

        let framebuffer = vec![false; 4 * 12];
        let f = Framebuffer::new(&framebuffer, 4, 12);
        test(f);

        // Width - 1 and height - 1 here to test offsets work when they need padding
        let framebuffer = vec![false; 3 * 11];
        let f = Framebuffer::new(&framebuffer, 3, 11);
        test(f);
    }

    #[test]
    fn chars() {
        let chars = (0..256)
            .map(|i| char::from_u32(0x2800 + i as u32).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(super::CHARS, &chars[..]);
    }
}
