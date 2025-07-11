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

/// A bundle representing styled text appearance and layout configuration.
///
/// `TextStyle` encapsulates layout, content, font, and color properties typically used
/// in rendering text elements within a UI system.
///
/// This bundle is intended to be used in UI components that require customizable
/// textual styling such as font sizing, alignment, line breaking, and color.
///
/// # Fields
/// - `layout`: Controls text alignment, line breaking, and justification rules.
/// - `text`: The actual string content to be displayed.
/// - `color`: The color of the rendered text.
/// - `font`: Font face, size, and smoothing attributes.
///
/// ```
#[derive(Bundle, Debug, Clone)]
pub struct TextStyle {
    /// Layout preferences such as alignment and line-breaking behavior.
    layout: TextLayout,

    /// The textual content to render.
    text: Text,

    /// The color used to draw the text.
    color: TextColor,

    /// Font properties including handle, size, and smoothing settings.
    font: TextFont,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            layout: TextLayout {
                justify: JustifyText::Center,
                linebreak: LineBreak::NoWrap,
            },
            text: Text::default(),
            color: TextColor(Srgba::BLACK.into()),
            font: TextFont {
                font_size: 16.0,
                ..Default::default()
            },
        }
    }
}

impl TextStyle {
    pub fn button() -> Self {
        Self {
            layout: TextLayout {
                justify: JustifyText::Center,
                linebreak: LineBreak::NoWrap,
            },
            ..Default::default()
        }
    }
}

/// A trait for views that support styled text content using a `TextStyle` bundle.
pub trait TextView: View {
    /// Returns a mutable reference to the inner `TextStyle` node used for text styling.
    fn text_node(&mut self) -> &mut TextStyle;

    /// Sets the text alignment mode within the layout container.
    ///
    /// Controls horizontal alignment of multiline text, such as left-aligned, right-aligned,
    /// centered, or justified.
    ///
    /// # Arguments
    /// * `justify_text` - A [`JustifyText`] variant indicating the desired alignment mode.
    ///
    /// # Example
    /// ```ignore
    /// view.text_alignment(JustifyText::Center);
    /// ```
    fn text_alignment(mut self, justify_text: JustifyText) -> Self {
        self.text_node().layout.justify = justify_text;
        self
    }

    /// Sets the line-breaking behavior of the text content.
    ///
    /// Determines how and when lines wrap, such as breaking on word boundaries,
    /// characters, or disabling wrapping altogether.
    ///
    /// # Arguments
    /// * `line_break` - A [`LineBreak`] variant specifying the wrapping strategy.
    ///
    /// # Example
    /// ```ignore
    /// view.text_linebreak(LineBreak::WordWrap);
    /// ```
    fn text_linebreak(mut self, line_break: LineBreak) -> Self {
        self.text_node().layout.linebreak = line_break;
        self
    }

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
