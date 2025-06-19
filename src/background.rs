use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_image::prelude::*;
use bevy_ui::prelude::*;

use crate::{View, ViewPack};

/// Provides background configuration for a UI container
pub trait BackgroundView: View {
    /// Returns a mutable reference to the current background style
    fn background_node(&mut self) -> &mut BackgroundStyle;

    /// Sets a solid color as the background
    fn background_color(mut self, color: impl Into<Color>) -> Self {
        *self.background_node() = BackgroundStyle::Color(color.into());
        self
    }

    /// Sets an image as the background
    fn background_image(mut self, image: Handle<Image>) -> Self {
        *self.background_node() = BackgroundStyle::Image(image);
        self
    }
}

/// Defines how a container should be visually styled in the background
#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundStyle {
    /// A solid background color
    Color(Color),
    /// A textured background image
    Image(Handle<Image>),
}

impl ViewPack for BackgroundStyle {
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
