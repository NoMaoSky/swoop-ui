use bevy_ui::prelude::*;

use crate::UiBase;

pub mod h_grid;
pub mod v_grid;

pub mod prelude {
    pub use super::GridContainer;
    pub use super::h_grid::*;
    pub use super::v_grid::*;
}

pub trait GridContainer: UiBase + Default {
    fn new(align_items: AlignItems, gap: Val) -> Self {
        let mut stack = Self::default();
        stack.node_node().align_items = align_items;
        stack.spacing(gap)
    }

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

    fn grid_auto_track(self, tracks: Vec<GridTrack>) -> Self;

    fn grid_template_track(self, tracks: Vec<RepeatedGridTrack>) -> Self;

    fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.node_node().justify_content = justify;
        self
    }

    fn align_items(mut self, align_items: AlignItems) -> Self {
        self.node_node().align_items = align_items;
        self
    }

    fn row_gap(mut self, gap: Val) -> Self {
        self.node_node().row_gap = gap;
        self
    }

    fn column_gap(mut self, gap: Val) -> Self {
        self.node_node().column_gap = gap;
        self
    }

    fn spacing(mut self, gap: Val) -> Self {
        let node = self.node_node();
        match node.grid_auto_flow {
            GridAutoFlow::Row | GridAutoFlow::RowDense => node.column_gap = gap,
            GridAutoFlow::Column | GridAutoFlow::ColumnDense => node.row_gap = gap,
        }

        self
    }
}
