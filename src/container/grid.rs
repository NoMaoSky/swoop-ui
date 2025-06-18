use bevy_ui::prelude::*;

use crate::UiBase;

/// Horizontal Grid Layout
pub mod h_grid;
/// Vertical Grid Layout
pub mod v_grid;

pub mod prelude {
    pub use super::GridContainer;
    pub use super::h_grid::*;
    pub use super::v_grid::*;
}

/// Provides a fluent interface for configuring grid layout containers
pub trait GridContainer: UiBase + Default {
    /// Creates a new grid container with specified alignment and spacing
    fn new(align_items: AlignItems, gap: Val) -> Self {
        let mut stack = Self::default();
        stack.node_node().align_items = align_items;
        stack.spacing(gap)
    }

    /// Toggles the grid flow between Row ⇄ RowDense or Column ⇄ ColumnDense
    fn reverse(mut self) -> Self {
        let node = self.node_node();
        let direction = match node.grid_auto_flow {
            GridAutoFlow::Row => GridAutoFlow::RowDense,
            GridAutoFlow::Column => GridAutoFlow::ColumnDense,
            GridAutoFlow::RowDense => GridAutoFlow::Row,
            GridAutoFlow::ColumnDense => GridAutoFlow::Column,
        };
        node.grid_auto_flow = direction;

        self
    }

    /// Sets automatic track sizing (e.g. `auto`, `min-content`, or fixed `px/%`)
    fn grid_auto_track(self, tracks: Vec<GridTrack>) -> Self;

    /// Sets explicit template tracks (e.g. `repeat`, `minmax`, or fixed spans)
    fn grid_template_track(self, tracks: Vec<RepeatedGridTrack>) -> Self;

    /// Aligns content along the main axis
    fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.node_node().justify_content = justify;
        self
    }

    /// Aligns items along the cross axis
    fn align_items(mut self, align_items: AlignItems) -> Self {
        self.node_node().align_items = align_items;
        self
    }

    /// Sets vertical spacing between grid rows
    fn row_gap(mut self, gap: Val) -> Self {
        self.node_node().row_gap = gap;
        self
    }

    /// Sets horizontal spacing between grid columns
    fn column_gap(mut self, gap: Val) -> Self {
        self.node_node().column_gap = gap;
        self
    }

    /// Applies gap spacing based on the grid’s auto flow direction
    fn spacing(mut self, gap: Val) -> Self {
        let node = self.node_node();
        match node.grid_auto_flow {
            GridAutoFlow::Row | GridAutoFlow::RowDense => node.column_gap = gap,
            GridAutoFlow::Column | GridAutoFlow::ColumnDense => node.row_gap = gap,
        }

        self
    }
}
