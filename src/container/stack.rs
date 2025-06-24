use bevy_ui::prelude::*;

use crate::View;

/// Horizontal Flex Layout
pub mod h_stack;
/// Vertical Flex Layout
pub mod v_stack;

pub mod prelude {
    pub use super::StackView;
    pub use super::h_stack::HStack;
    pub use super::v_stack::VStack;
}

/// Provides a flexible layout interface for stack-style containers (e.g. HStack, VStack).
///
/// This trait is intended for views that manage their children using a `FlexDirection`-based
/// layout model. It offers a fluent builder-style interface for configuring core layout
/// properties like spacing, alignment, and wrapping.
pub trait StackView: View {
    /// Creates a new instance with a specified `JustifyContent` alignment along the main axis.
    ///
    /// # Arguments
    /// * `justify_content` - Defines how free space is distributed along the main axis.
    fn from_justify_content(justify_content: JustifyContent) -> Self {
        let mut stack = Self::default();
        stack.node_node().justify_content = justify_content;
        stack
    }

    /// Creates a new instance with a specified `AlignItems` alignment along the cross axis.
    ///
    /// # Arguments
    /// * `align_items` - Defines how child elements are aligned perpendicular to the main axis.
    fn from_align_items(align_items: AlignItems) -> Self {
        let mut stack = Self::default();
        stack.node_node().align_items = align_items;
        stack
    }

    /// Sets the direction of content flow within the stack (e.g. horizontal or vertical).
    ///
    /// # Arguments
    /// * `direction` - The flex direction, such as `Row` or `Column`.
    fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.node_node().flex_direction = direction;
        self
    }

    /// Sets the wrapping behavior of the flex container.
    ///
    /// Determines whether children wrap to the next line or stay on a single line.
    ///
    /// # Arguments
    /// * `wrap` - The wrapping strategy, such as `NoWrap` or `Wrap`.
    fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.node_node().flex_wrap = wrap;
        self
    }

    /// Defines how extra space along the main axis is distributed.
    ///
    /// # Arguments
    /// * `justify` - The justification mode (e.g. `Center`, `SpaceBetween`).
    fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.node_node().justify_content = justify;
        self
    }

    /// Defines how children are aligned along the cross axis.
    ///
    /// # Arguments
    /// * `align_items` - The alignment mode (e.g. `Start`, `Stretch`).
    fn align_items(mut self, align_items: AlignItems) -> Self {
        self.node_node().align_items = align_items;
        self
    }

    /// Sets the vertical spacing between rows when wrapping is enabled.
    ///
    /// # Arguments
    /// * `gap` - The spacing between rows (`Val::Px`, `Val::Percent`, etc.).
    fn row_gap(mut self, gap: Val) -> Self {
        self.node_node().row_gap = gap;
        self
    }

    /// Sets the horizontal spacing between columns when wrapping is enabled.
    ///
    /// # Arguments
    /// * `gap` - The spacing between columns (`Val::Px`, `Val::Percent`, etc.).
    fn column_gap(mut self, gap: Val) -> Self {
        self.node_node().column_gap = gap;
        self
    }
}
