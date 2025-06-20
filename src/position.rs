use bevy_ui::prelude::*;

use crate::View;

/// A trait that provides convenient absolute positioning methods for views.
pub trait PositionView: View {
    /// Sets the top offset of the view in absolute positioning mode.
    ///
    /// # Arguments
    /// * `y` - Vertical distance from the top edge of the parent container.
    fn top(mut self, y: Val) -> Self {
        let node = self.node_node();
        node.position_type = PositionType::Absolute;
        node.top = y;
        self
    }

    /// Sets the left offset of the view in absolute positioning mode.
    ///
    /// # Arguments
    /// * `x` - Horizontal distance from the left edge of the parent container.
    fn left(mut self, x: Val) -> Self {
        let node = self.node_node();
        node.position_type = PositionType::Absolute;
        node.left = x;
        self
    }

    /// Sets the right offset of the view in absolute positioning mode.
    ///
    /// # Arguments
    /// * `x` - Distance from the right edge of the parent container.
    fn right(mut self, x: Val) -> Self {
        let node = self.node_node();
        node.position_type = PositionType::Absolute;
        node.right = x;
        self
    }

    /// Sets the bottom offset of the view in absolute positioning mode.
    ///
    /// # Arguments
    /// * `y` - Distance from the bottom edge of the parent container.
    fn bottom(mut self, y: Val) -> Self {
        let node = self.node_node();
        node.position_type = PositionType::Absolute;
        node.bottom = y;
        self
    }

    /// Sets both the left and top offsets of the view in absolute positioning mode.
    ///
    /// # Arguments
    /// * `x` - Horizontal distance from the left edge.
    /// * `y` - Vertical distance from the top edge.
    fn position(mut self, x: Val, y: Val) -> Self {
        let node = self.node_node();
        node.position_type = PositionType::Absolute;
        node.left = x;
        node.top = y;
        self
    }
}
