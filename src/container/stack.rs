use bevy_ui::prelude::*;

use crate::UiBase;

pub mod h_stack;
pub mod v_stack;

pub mod prelude {
    pub use super::StackContainer;
    pub use super::h_stack::*;
    pub use super::v_stack::*;
}

/// Provides a flexible layout interface for stack-style containers (e.g. HStack, VStack)
pub trait StackContainer: UiBase + Default {
    /// Creates a new stack container with specified alignment and spacing
    fn new(align_items: AlignItems, gap: Val) -> Self {
        let mut stack = Self::default();
        stack.node_node().align_items = align_items;
        stack.spacing(gap)
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
