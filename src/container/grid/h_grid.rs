use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::View;
use crate::background::{BackgroundStyle, BackgroundView};
use crate::border::{BorderStyle, BorderView};
use crate::prelude::PositionView;
use crate::shadow::BoxShadowView;

use super::GridView;

/// A horizontal grid layout container that arranges children using `GridAutoFlow::Row`.
/// Supports background and border styling as well as custom grid track configuration.
#[derive(Bundle, Debug, Clone)]
pub struct HGrid {
    /// The name component used to identify the UI node
    name: Name,
    /// The layout node controlling grid behavior and spacing
    node: Node,
    /// Border rendering style (color and radius)
    border: BorderStyle,
    /// Background rendering style (color or image)
    background: BackgroundStyle,
    /// Shadow rendering style (Vec<ShadowStyle>)
    shadow: BoxShadow,
}

impl Default for HGrid {
    fn default() -> Self {
        Self {
            name: Name::new("HGrid"),
            node: Node {
                display: Display::Grid,
                grid_auto_flow: GridAutoFlow::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: Val::Px(0.0),
                ..Default::default()
            },
            border: BorderStyle::default(),
            background: BackgroundStyle::default(),
            shadow: BoxShadow::default(),
        }
    }
}

impl View for HGrid {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl GridView for HGrid {}

impl BackgroundView for HGrid {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BorderView for HGrid {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl BoxShadowView for HGrid {
    fn box_shadow_node(&mut self) -> &mut BoxShadow {
        &mut self.shadow
    }
}

impl PositionView for HGrid {}
