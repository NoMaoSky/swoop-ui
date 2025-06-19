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

/// Provides a flexible layout interface for stack-style containers (e.g. HStack, VStack)
pub trait StackView: View {
    fn from_justify_content(justify_content: JustifyContent) -> Self {
        let mut stack = Self::default();
        stack.node_node().justify_content = justify_content;
        stack
    }

    fn from_align_items(align_items: AlignItems) -> Self {
        let mut stack = Self::default();
        stack.node_node().align_items = align_items;
        stack
    }

    /// Reverses the current flex direction (e.g. Row ⇄ RowReverse, Column ⇄ ColumnReverse)
    fn reverse(mut self) -> Self {
        let node = self.node_node();
        let direction = match node.flex_direction {
            FlexDirection::Row => FlexDirection::RowReverse,
            FlexDirection::Column => FlexDirection::ColumnReverse,
            FlexDirection::RowReverse => FlexDirection::Row,
            FlexDirection::ColumnReverse => FlexDirection::Column,
        };
        node.flex_direction = direction;

        self
    }

    /// Sets the wrapping behavior of the flex container
    fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.node_node().flex_wrap = wrap;
        self
    }

    /// Defines how extra space along the main axis is distributed
    fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.node_node().justify_content = justify;
        self
    }

    /// Defines how children are aligned along the cross axis
    fn align_items(mut self, align_items: AlignItems) -> Self {
        self.node_node().align_items = align_items;
        self
    }

    /// Sets vertical spacing between rows
    fn row_gap(mut self, gap: Val) -> Self {
        self.node_node().row_gap = gap;
        self
    }

    /// Sets horizontal spacing between columns
    fn column_gap(mut self, gap: Val) -> Self {
        self.node_node().column_gap = gap;
        self
    }

    /// Automatically applies spacing based on current flex direction
    fn spacing(mut self, gap: Val) -> Self {
        let node = self.node_node();
        match node.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => node.column_gap = gap,
            FlexDirection::Column | FlexDirection::ColumnReverse => node.row_gap = gap,
        }

        self
    }
}
