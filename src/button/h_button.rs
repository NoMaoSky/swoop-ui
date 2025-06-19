use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::View;
use crate::border::{BorderStyle, BorderView};
use crate::prelude::{BackgroundStyle, BackgroundView, StackView};
use crate::shadow::ShadowView;

#[derive(Bundle, Debug, Clone)]
pub struct HButton {
    name: Name,
    node: Node,
    botton: Button,
    border: BorderStyle,
    background: BackgroundStyle,
    shadow: BoxShadow,
}

impl Default for HButton {
    fn default() -> Self {
        Self {
            name: Name::new("SwoopButton"),
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(0.0),
                ..Default::default()
            },
            botton: Button,
            border: BorderStyle::button(),
            background: BackgroundStyle::button(),
            shadow: BoxShadow::default(),
        }
    }
}

impl View for HButton {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl StackView for HButton {}

impl BackgroundView for HButton {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BorderView for HButton {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl ShadowView for HButton {
    fn shadow_node(&mut self) -> &mut BoxShadow {
        &mut self.shadow
    }
}
