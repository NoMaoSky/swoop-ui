use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_image::prelude::*;
use bevy_ui::prelude::*;

use crate::UiToBundle;

pub mod grid;
pub mod stack;

pub mod prelude {
    pub use super::grid::prelude::*;
    pub use super::stack::prelude::*;
    pub use super::{BackgroundContainer, BackgroundStyle, BorderContainer, BorderStyle};
}

/// Provides background configuration for a UI container
pub trait BackgroundContainer {
    /// Returns a mutable reference to the current background style
    fn background_node(&mut self) -> &mut BackgroundStyle;

    /// Sets a solid color as the background
    fn background_color(mut self, color: impl Into<Color>) -> Self
    where
        Self: Sized,
    {
        *self.background_node() = BackgroundStyle::Color(color.into());
        self
    }

    /// Sets an image as the background
    fn background_image(mut self, image: Handle<Image>) -> Self
    where
        Self: Sized,
    {
        *self.background_node() = BackgroundStyle::Image(image);
        self
    }
}

/// Defines how a container should be visually styled in the background
pub enum BackgroundStyle {
    /// A solid background color
    Color(Color),
    /// A textured background image
    Image(Handle<Image>),
}

impl UiToBundle for BackgroundStyle {
    fn pack(self) -> impl Bundle {
        match self {
            BackgroundStyle::Color(color) => (
                BackgroundColor(color),
                ImageNode::solid_color(Srgba::NONE.into()),
            ),
            BackgroundStyle::Image(handle) => {
                (BackgroundColor(Srgba::NONE.into()), ImageNode::new(handle))
            }
        }
    }
}

impl Default for BackgroundStyle {
    fn default() -> Self {
        BackgroundStyle::Color(Srgba::NONE.into())
    }
}

/// Provides border configuration for a UI container
pub trait BorderContainer {
    /// Returns a mutable reference to the current border style
    fn border_node(&mut self) -> &mut BorderStyle;

    /// Sets the border color
    fn border_color(mut self, border_color: impl Into<Color>) -> Self
    where
        Self: Sized,
    {
        self.border_node().border_color.0 = border_color.into();
        self
    }

    /// Sets the border radius
    fn border_radius(mut self, border_radius: BorderRadius) -> Self
    where
        Self: Sized,
    {
        self.border_node().border_radius = border_radius;
        self
    }
}

/// Describes the style for rendering borders around a UI container
pub struct BorderStyle {
    /// The corner radius for the border
    border_radius: BorderRadius,
    /// The color of the border
    border_color: BorderColor,
}

impl UiToBundle for BorderStyle {
    fn pack(self) -> impl Bundle {
        (self.border_radius, self.border_color)
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            border_radius: BorderRadius::ZERO,
            border_color: BorderColor(Srgba::NONE.into()),
        }
    }
}
