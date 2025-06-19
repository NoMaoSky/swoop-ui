use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_image::prelude::*;
use bevy_ui::prelude::*;

use crate::View;

/// Provides background configuration for a UI container
pub trait BackgroundView: View {
    /// Returns a mutable reference to the current background style
    fn background_node(&mut self) -> &mut BackgroundStyle;

    /// Sets a solid color as the background
    fn background_color(mut self, color: impl Into<Color>) -> Self {
        self.background_node().color = BackgroundColor(color.into());
        self
    }

    /// Sets an image as the background
    fn background_image(mut self, image: Handle<Image>) -> Self {
        self.background_node().image = ImageNode {
            image,
            ..Default::default()
        };
        self
    }
}

/// Defines how a container should be visually styled in the background
#[derive(Bundle, Debug, Clone, Default)]
pub struct BackgroundStyle {
    /// A solid background color
    color: BackgroundColor,
    /// A textured background image
    image: ImageNode,
}
