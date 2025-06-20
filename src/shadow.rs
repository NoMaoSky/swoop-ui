use bevy_color::prelude::*;
use bevy_math::prelude::*;
use bevy_ui::prelude::*;

use crate::View;

/// A trait for setting and modifying UI container shadow styles.
pub trait BoxShadowView: View {
    /// Returns a mutable reference to the internal shadow style list.
    fn box_shadow_node(&mut self) -> &mut BoxShadow;

    /// Replaces all existing shadows with the provided list.
    fn shadow(mut self, shadows: Vec<ShadowStyle>) -> Self {
        self.box_shadow_node().0 = shadows;
        self
    }

    /// Appends a new shadow to the current list.
    fn add_shadow(mut self, shadow: ShadowStyle) -> Self {
        self.box_shadow_node().0.push(shadow);
        self
    }
}

/// A trait for views that support text shadow styling.
///
/// Provides methods to configure shadow offset and color for text-based views.
pub trait TextShadowView: View {
    /// Returns a mutable reference to the underlying `TextShadow` node,
    /// which stores shadow-related styling information.
    fn text_shadow_node(&mut self) -> &mut TextShadow;

    /// Sets the offset of the text shadow.
    ///
    /// # Arguments
    /// * `offset` - A `Vec2` specifying the horizontal and vertical offset of the shadow.
    ///
    /// # Example
    /// ```
    /// view.text_shadow_offset(Vec2::new(2.0, -2.0));
    /// ```
    fn text_shadow_offset(mut self, offset: Vec2) -> Self {
        self.text_shadow_node().offset = offset;
        self
    }

    /// Sets the color of the text shadow.
    ///
    /// # Arguments
    /// * `color` - A color value (e.g., `Color::rgba(...)`) that defines the shadow’s appearance.
    ///
    /// # Example
    /// ```
    /// view.text_shadow_color(Color::BLACK.with_a(0.5));
    /// ```
    fn text_shadow_color(mut self, color: impl Into<Color>) -> Self {
        self.text_shadow_node().color = color.into();
        self
    }
}
