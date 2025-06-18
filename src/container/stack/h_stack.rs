use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::container::{BackgroundContainer, BackgroundStyle, BorderContainer, BorderStyle};
use crate::{UiBase, UiToBundle};

use super::StackContainer;

pub struct HStack {
    name: Name,
    node: Node,
    border: BorderStyle,
    background: BackgroundStyle,
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
                column_gap: Val::Px(8.0),
                ..Default::default()
            },
            border: BorderStyle::default(),
            background: BackgroundStyle::default(),
        }
    }
}

impl UiBase for HStack {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl StackContainer for HStack {}

impl BackgroundContainer for HStack {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BorderContainer for HStack {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl UiToBundle for HStack {
    fn pack(self) -> impl Bundle {
        let name = self.name;
        let border = self.border.pack();
        let background = self.background.pack();
        (name, self.node, border, background)
    }
}
