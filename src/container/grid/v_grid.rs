use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::background::{BackgroundStyle, BackgroundView};
use crate::border::{BorderStyle, BorderView};
use crate::shadow::ShadowView;
use crate::{View, ViewPack};

use super::GridView;

/// A vertical grid layout container that arranges children using `GridAutoFlow::Column`.
/// Supports background and border styling, and allows for dynamic grid track configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct VGrid {
    /// The name component used to identify the UI node
    name: Name,
    /// The layout node controlling grid structure, flow, and spacing
    node: Node,
    /// Border rendering style (color and radius)
    border: BorderStyle,
    /// Background rendering style (color or image)
    background: BackgroundStyle,
    /// Shadow rendering style (Vec<ShadowStyle>)
    shadow: BoxShadow,
}

impl Default for VGrid {
    fn default() -> Self {
        Self {
            name: Name::new("VGrid"),
            node: Node {
                display: Display::Grid,
                grid_auto_flow: GridAutoFlow::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                row_gap: Val::Px(0.0),
                ..Default::default()
            },
            border: BorderStyle::default(),
            background: BackgroundStyle::default(),
            shadow: BoxShadow::default(),
        }
    }
}

impl View for VGrid {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl GridView for VGrid {
    fn grid_auto_track(mut self, tracks: Vec<GridTrack>) -> Self {
        self.node_node().grid_auto_rows = tracks;
        self
    }

    fn grid_template_track(mut self, tracks: Vec<RepeatedGridTrack>) -> Self {
        self.node_node().grid_template_rows = tracks;
        self
    }
}

impl BackgroundView for VGrid {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BorderView for VGrid {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl ShadowView for VGrid {
    fn shadow_node(&mut self) -> &mut BoxShadow {
        &mut self.shadow
    }
}

impl ViewPack for VGrid {
    fn pack(self) -> impl Bundle {
        let name = self.name;
        let border = self.border.pack();
        let background = self.background.pack();
        (name, self.node, border, background, self.shadow)
    }
}
