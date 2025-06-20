use bevy_asset::Handle;
use bevy_color::prelude::*;
use bevy_ecs::bundle::Bundle;
use bevy_text::{FontSmoothing, LineHeight, prelude::*};
use bevy_ui::prelude::*;

use crate::View;

pub mod swoop_text;

pub mod prelude {
    pub use super::TextView;
    pub use super::swoop_text::{SText, SwoopText};
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct TextStyle {
    text: Text,
    color: TextColor,
    font: TextFont,
}

/// A trait for views that support styled text content using a `TextStyle` bundle.
pub trait TextView: View {
    /// Returns a mutable reference to the inner `TextStyle` node used for text styling.
    fn text_node(&mut self) -> &mut TextStyle;

    /// Sets the string content of the text node.
    ///
    /// # Arguments
    /// * `text` - A string or string-like value representing the text to display.
    ///
    /// # Example
    /// ```
    /// view.text("Hello, world!");
    /// ```
    fn text(mut self, text: impl Into<String>) -> Self {
        self.text_node().text.0 = text.into();
        self
    }

    /// Sets the text color of the node.
    ///
    /// # Arguments
    /// * `color` - A color value (e.g., `Color::WHITE`, `Color::rgb(...)`).
    fn text_color(mut self, color: impl Into<Color>) -> Self {
        self.text_node().color.0 = color.into();
        self
    }

    /// Sets the font asset used for rendering the text.
    ///
    /// # Arguments
    /// * `font` - A handle to a `Font` asset.
    fn font(mut self, font: Handle<Font>) -> Self {
        self.text_node().font.font = font;
        self
    }

    /// Sets the size of the font in logical pixels.
    ///
    /// # Arguments
    /// * `font_size` - The font size, typically in points or pixels.
    fn font_size(mut self, font_size: f32) -> Self {
        self.text_node().font.font_size = font_size;
        self
    }

    /// Sets the line height for the text, controlling vertical spacing between lines.
    ///
    /// # Arguments
    /// * `line_height` - A `LineHeight` value representing relative or absolute spacing.
    fn line_height(mut self, line_height: LineHeight) -> Self {
        self.text_node().font.line_height = line_height;
        self
    }

    /// Sets the font smoothing mode for the text.
    ///
    /// # Arguments
    /// * `font_smoothing` - A `FontSmoothing` strategy (e.g., `FontSmoothing::None`, `Subpixel`).
    fn font_smoothing(mut self, font_smoothing: FontSmoothing) -> Self {
        self.text_node().font.font_smoothing = font_smoothing;
        self
    }
}
