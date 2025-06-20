use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::View;
use crate::background::{BackgroundStyle, BackgroundView};
use crate::border::{BorderStyle, BorderView};
use crate::prelude::PositionView;
use crate::shadow::BoxShadowView;

use super::StackView;

/// A horizontal stack layout container that arranges children in a row.
/// It supports background and border styling, and conforms to StackContainer behavior.
#[derive(Bundle, Debug, Clone)]
pub struct HStack {
    /// The name component used to identify the UI node
    name: Name,
    /// The layout node controlling size, flex direction, spacing, etc.
    node: Node,
    /// Border rendering style (color and radius)
    border: BorderStyle,
    /// Background rendering style (color or image)
    background: BackgroundStyle,
    /// Shadow rendering style (Vec<ShadowStyle>)
    shadow: BoxShadow,
}

impl Default for HStack {
    fn default() -> Self {
        Self {
            name: Name::new("HStack"),
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
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

impl View for HStack {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl StackView for HStack {}

impl BackgroundView for HStack {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BorderView for HStack {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl BoxShadowView for HStack {
    fn box_shadow_node(&mut self) -> &mut BoxShadow {
        &mut self.shadow
    }
}

impl PositionView for HStack {}
