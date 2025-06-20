use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::View;
use crate::background::BackgroundStyle;
use crate::border::{BorderStyle, BorderView};
use crate::prelude::{BackgroundView, PositionView, StackView};
use crate::shadow::BoxShadowView;

/// A horizontally laid-out button view with customizable border, background, and shadow.
///
/// `HButton` is a styled UI component composed using Bevy's ECS `Bundle`.
/// By default, it uses a horizontal `FlexDirection::Row` layout with centered alignment,
/// and includes styling traits such as `BorderView`, `BackgroundView`, `BoxShadowView`, and `PositionView`.
///
/// This component is ideal for creating rich, composable buttons in declarative UI.
///
/// # Default Layout
/// - `display`: `Flex`
/// - `flex_direction`: `Row`
/// - `justify_content`: `Center`
/// - `align_items`: `Center`
/// - `column_gap`: `0.0`
///
/// # Example
/// ```ignore
/// commands.spawn(HButton::default()
///     .text("Click me")
///     .font(my_font)
///     .border_width(Val::Px(2.0))
///     .background_color(Color::BLUE));
/// ```
#[derive(Bundle, Debug, Clone)]
pub struct HButton {
    /// Identifier for debugging and inspection.
    name: Name,

    /// Layout node defining size, flex behavior, and spacing.
    node: Node,

    /// Button marker component used by Bevy's UI interaction system.
    botton: Button,

    /// Style information for borders.
    border: BorderStyle,

    /// Background styling (color, gradient, image, etc.).
    background: BackgroundStyle,

    /// Optional box shadow styling for depth and elevation.
    shadow: BoxShadow,
}

impl Default for HButton {
    /// Creates a default `HButton` with centered horizontal layout and themed styling.
    fn default() -> Self {
        Self {
            name: Name::new("HButton"),
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

impl BoxShadowView for HButton {
    fn box_shadow_node(&mut self) -> &mut BoxShadow {
        &mut self.shadow
    }
}

impl PositionView for HButton {}
