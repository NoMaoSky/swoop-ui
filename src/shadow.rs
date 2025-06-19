use bevy_ui::prelude::*;

use crate::View;

/// A trait for setting and modifying UI container shadow styles.
pub trait ShadowView: View {
    /// Returns a mutable reference to the internal shadow style list.
    fn shadow_node(&mut self) -> &mut BoxShadow;

    /// Replaces all existing shadows with the provided list.
    fn shadow(mut self, shadows: Vec<ShadowStyle>) -> Self {
        self.shadow_node().0 = shadows;
        self
    }

    /// Appends a new shadow to the current list.
    fn add_shadow(mut self, shadow: ShadowStyle) -> Self {
        self.shadow_node().0.push(shadow);
        self
    }
}
