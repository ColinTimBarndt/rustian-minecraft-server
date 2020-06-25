use super::chat_components::ChatColor;
use std::convert::From;
use std::default::Default;

#[derive(Copy, Clone, Debug, Hash)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

impl Color {
  // Standard colors
  pub const BLACK: Color = u32_to_color(0x000000);
  pub const BLUE: Color = u32_to_color(0x0000ff);
  pub const GREEN: Color = u32_to_color(0x00ff00);
  pub const AQUA: Color = u32_to_color(0x00ffff);
  pub const RED: Color = u32_to_color(0xff0000);
  pub const PURPLE: Color = u32_to_color(0xff00ff);
  pub const YELLOW: Color = u32_to_color(0xffff00);
  pub const WHITE: Color = u32_to_color(0xffffff);
  // Standard gray
  pub const LIGHT_GRAY: Color = u32_to_color(0xbfbfbf);
  pub const GRAY: Color = u32_to_color(0x7f7f7f);
  pub const DARK_GRAY: Color = u32_to_color(0x404040);
  // Minecraft chat colors
  pub const CHAT_DARK_BLUE: Color = u32_to_color(0x0000aa);
  pub const CHAT_DARK_GREEN: Color = u32_to_color(0x00aa00);
  pub const CHAT_DARK_AQUA: Color = u32_to_color(0x00aaaa);
  pub const CHAT_DARK_RED: Color = u32_to_color(0xaa0000);
  pub const CHAT_DARK_PURPLE: Color = u32_to_color(0xaa00aa);
  pub const CHAT_GOLD: Color = u32_to_color(0xffaa00);
  pub const CHAT_GRAY: Color = u32_to_color(0xaaaaaa);
  pub const CHAT_DARK_GRAY: Color = u32_to_color(0x555555);
  pub const CHAT_BLUE: Color = u32_to_color(0x5555ff);
  pub const CHAT_GREEN: Color = u32_to_color(0x55ff55);
  pub const CHAT_AQUA: Color = u32_to_color(0x55ffff);
  pub const CHAT_RED: Color = u32_to_color(0xff5555);
  pub const CHAT_LIGHT_PURPLE: Color = u32_to_color(0xff55ff);
  pub const CHAT_YELLOW: Color = u32_to_color(0xffff55);

  pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
    Self {
      red: r,
      green: g,
      blue: b,
    }
  }

  pub fn shade(&self) -> u8 {
    (self.red + self.green + self.blue) / 3
  }

  /// Gets the [luminance](https://en.wikipedia.org/wiki/Relative_luminance) of this color
  pub fn relative_luminance(&self) -> f32 {
    f32::min(
      1f32,
      (self.red as f32) * 0.2126f32
        + (self.green as f32) * 0.7152f32
        + (self.blue as f32) * 0.0722f32,
    )
  }

  /// Returns a string with the format `#rrggbb`
  pub fn to_hex_code(&self) -> String {
    format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
  }
}

impl Default for Color {
  fn default() -> Self {
    Self::BLACK
  }
}

impl From<u32> for Color {
  fn from(rgb: u32) -> Self {
    Self {
      red: ((rgb & 0x00ff0000) >> 16) as u8,
      green: ((rgb & 0x0000ff00) >> 8) as u8,
      blue: (rgb & 0x000000ff) as u8,
    }
  }
}

impl From<Color> for u32 {
  fn from(color: Color) -> u32 {
    ((color.red as u32) << 16) | ((color.green as u32) << 8) | color.blue as u32
  }
}

impl From<ChatColor> for Color {
  fn from(color: ChatColor) -> Color {
    use ChatColor::*;
    match color {
      Black => Color::BLACK,
      DarkBlue => Color::CHAT_DARK_BLUE,
      DarkGreen => Color::CHAT_DARK_GREEN,
      DarkAqua => Color::CHAT_DARK_AQUA,
      DarkRed => Color::CHAT_DARK_RED,
      DarkPurple => Color::CHAT_DARK_PURPLE,
      Gold => Color::CHAT_GOLD,
      Gray => Color::CHAT_GRAY,
      DarkGray => Color::CHAT_DARK_GRAY,
      Blue => Color::CHAT_BLUE,
      Green => Color::CHAT_GREEN,
      Aqua => Color::CHAT_AQUA,
      Red => Color::CHAT_RED,
      LightPurple => Color::CHAT_LIGHT_PURPLE,
      Yellow => Color::CHAT_YELLOW,
      White => Color::WHITE,
      Custom(color) => color,
    }
  }
}

impl From<Color> for ChatColor {
  fn from(color: Color) -> ChatColor {
    ChatColor::Custom(color)
  }
}

const fn u32_to_color(rgb: u32) -> Color {
  Color {
    red: ((rgb & 0x00ff0000) >> 16) as u8,
    green: ((rgb & 0x0000ff00) >> 8) as u8,
    blue: (rgb & 0x000000ff) as u8,
  }
}
