use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::View;

/// Provides border configuration for a UI container
pub trait BorderView: View {
    /// Returns a mutable reference to the current border style
    fn border_node(&mut self) -> &mut BorderStyle;

    /// Sets the border size
    fn border(mut self, border: UiRect) -> Self {
        self.node_node().border = border;
        self
    }

    /// Sets the border color
    fn border_color(mut self, border_color: impl Into<Color>) -> Self {
        self.border_node().border_color.0 = border_color.into();
        self
    }

    /// Sets the border radius
    fn border_radius(mut self, border_radius: BorderRadius) -> Self {
        self.border_node().border_radius = border_radius;
        self
    }
}

/// Describes the style for rendering borders around a UI container
#[derive(Bundle, Debug, Clone)]
pub struct BorderStyle {
    /// The corner radius for the border
    border_radius: BorderRadius,
    /// The color of the border
    border_color: BorderColor,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            border_radius: BorderRadius::ZERO,
            border_color: BorderColor(Srgba::NONE.into()),
        }
    }
}
