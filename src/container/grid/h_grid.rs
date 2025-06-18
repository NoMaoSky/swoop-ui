use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::container::{BackgroundContainer, BackgroundStyle, BorderContainer, BorderStyle};
use crate::{UiBase, UiToBundle};

use super::GridContainer;

pub struct HGrid {
    name: Name,
    node: Node,
    border: BorderStyle,
    background: BackgroundStyle,
}

impl Default for HGrid {
    fn default() -> Self {
        Self {
            name: Name::new("HGrid"),
            node: Node {
                display: Display::Grid,
                grid_auto_flow: GridAutoFlow::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..Default::default()
            },
            border: BorderStyle::default(),
            background: BackgroundStyle::default(),
        }
    }
}

impl UiBase for HGrid {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl GridContainer for HGrid {
    fn grid_auto_track(mut self, tracks: Vec<GridTrack>) -> Self {
        self.node_node().grid_auto_columns = tracks;
        self
    }

    fn grid_template_track(mut self, tracks: Vec<RepeatedGridTrack>) -> Self {
        self.node_node().grid_template_columns = tracks;
        self
    }
}

impl BackgroundContainer for HGrid {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BorderContainer for HGrid {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl UiToBundle for HGrid {
    fn pack(self) -> impl Bundle {
        let name = self.name;
        let border = self.border.pack();
        let background = self.background.pack();
        (name, self.node, border, background)
    }
}
