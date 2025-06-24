use bevy_ui::prelude::*;

use crate::View;

/// Horizontal Grid Layout
pub mod h_grid;
/// Vertical Grid Layout
pub mod v_grid;

pub mod prelude {
    pub use super::GridView;
    pub use super::h_grid::HGrid;
    pub use super::v_grid::VGrid;
}

/// Provides a fluent interface for configuring grid layout containers.
///
/// This trait is intended for views that participate in CSS-like grid layouts.
/// It enables precise definition of tracks (rows and columns), gaps, flow behavior,
/// and alignment along both axes, using a builder-style API.
pub trait GridView: View {
    /// Sets the flow algorithm used to place items in the grid.
    ///
    /// # Arguments
    /// * `flow` – Determines item placement strategy, such as `Row`, `Column`, `RowDense`, or `ColumnDense`.
    fn grid_auto_flow(mut self, flow: GridAutoFlow) -> Self {
        self.node_node().grid_auto_flow = flow;
        self
    }

    /// Specifies the sizing behavior for automatically created columns.
    ///
    /// Applies when children overflow the defined template or no template columns are set.
    ///
    /// # Arguments
    /// * `tracks` – A vector of `GridTrack` values that define implicit column sizing.
    fn grid_auto_columns(mut self, tracks: Vec<GridTrack>) -> Self {
        self.node_node().grid_auto_columns = tracks;
        self
    }

    /// Specifies the sizing behavior for automatically created rows.
    ///
    /// Applies when content exceeds the defined row template.
    ///
    /// # Arguments
    /// * `tracks` – A vector of `GridTrack` values that define implicit row sizing.
    fn grid_auto_rows(mut self, tracks: Vec<GridTrack>) -> Self {
        self.node_node().grid_auto_rows = tracks;
        self
    }

    /// Defines the column track template for this grid.
    ///
    /// Explicitly sets the layout behavior of each column.
    ///
    /// # Arguments
    /// * `tracks` – A vector of `RepeatedGridTrack` values to control each column's size and repeat behavior.
    fn grid_template_columns(mut self, tracks: Vec<RepeatedGridTrack>) -> Self {
        self.node_node().grid_template_columns = tracks;
        self
    }

    /// Defines the row track template for this grid.
    ///
    /// Explicitly sets the layout behavior of each row.
    ///
    /// # Arguments
    /// * `tracks` – A vector of `RepeatedGridTrack` values to control each row's size and repeat behavior.
    fn grid_template_rows(mut self, tracks: Vec<RepeatedGridTrack>) -> Self {
        self.node_node().grid_template_rows = tracks;
        self
    }

    /// Aligns all grid content along the main axis (horizontal in LTR layouts).
    ///
    /// # Arguments
    /// * `justify` – Justification mode (e.g. `Start`, `Center`, `SpaceBetween`).
    fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.node_node().justify_content = justify;
        self
    }

    /// Aligns grid items along the cross axis (vertical in LTR layouts).
    ///
    /// # Arguments
    /// * `align_items` – Alignment mode for items within their cells.
    fn align_items(mut self, align_items: AlignItems) -> Self {
        self.node_node().align_items = align_items;
        self
    }

    /// Sets the vertical spacing between grid rows.
    ///
    /// # Arguments
    /// * `gap` – The space to insert between rows, using `Val::Px`, `Val::Percent`, etc.
    fn row_gap(mut self, gap: Val) -> Self {
        self.node_node().row_gap = gap;
        self
    }

    /// Sets the horizontal spacing between grid columns.
    ///
    /// # Arguments
    /// * `gap` – The space to insert between columns, using `Val::Px`, `Val::Percent`, etc.
    fn column_gap(mut self, gap: Val) -> Self {
        self.node_node().column_gap = gap;
        self
    }
}
