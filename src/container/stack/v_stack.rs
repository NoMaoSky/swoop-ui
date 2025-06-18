use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::container::{BackgroundContainer, BackgroundStyle, BorderContainer, BorderStyle};
use crate::{UiBase, UiToBundle};

use super::StackContainer;

/// A vertical stack layout container that arranges children in a column.
/// Implements background and border styling, and conforms to StackContainer behavior.
pub struct VStack {
    /// The name component used to identify the UI node
    name: Name,
    /// The layout node controlling size, flex direction, spacing, etc.
    node: Node,
    /// Border rendering style (color and radius)
    border: BorderStyle,
    /// Background rendering style (color or image)
    background: BackgroundStyle,
}

impl Default for VStack {
    fn default() -> Self {
        Self {
            name: Name::new("VStack"),
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                row_gap: Val::Px(0.0),
                ..Default::default()
            },
            border: BorderStyle::default(),
            background: BackgroundStyle::default(),
        }
    }
}

impl UiBase for VStack {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl StackContainer for VStack {}

impl BackgroundContainer for VStack {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BorderContainer for VStack {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl UiToBundle for VStack {
    fn pack(self) -> impl Bundle {
        let name = self.name;
        let border = self.border.pack();
        let background = self.background.pack();
        (name, self.node, border, background)
    }
}
