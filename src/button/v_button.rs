use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::View;
use crate::background::BackgroundStyle;
use crate::border::{BorderStyle, BorderView};
use crate::prelude::{BackgroundView, PositionView, StackView};
use crate::shadow::BoxShadowView;

/// A vertically stacked button view with customizable styling, including border, background,
/// and box shadow. Suited for UI layouts where label and icon should be stacked top-to-bottom.
///
/// By default, `VButton` is configured with:
/// - `flex_direction`: `Column`
/// - `justify_content`: `Center`
/// - `align_items`: `Center`
/// - `row_gap`: `0.0`
///
/// Implements styling traits such as `BackgroundView`, `BorderView`, `BoxShadowView`,
/// and layout behavior from `PositionView` and `StackView`.
///
/// # Example
/// ```ignore
/// commands.spawn(VButton::default()
///     .text("Submit")
///     .font(my_font)
///     .background_color(Color::ORANGE)
///     .border_radius(Val::Px(8.0)));
/// ```
#[derive(Bundle, Debug, Clone)]
pub struct VButton {
    /// Name tag for debugging or entity identification.
    name: Name,

    /// Layout configuration node using vertical flex.
    node: Node,

    /// Interaction marker enabling Bevy’s UI click behavior.
    botton: Button,

    /// Border visuals such as width, color, and radius.
    border: BorderStyle,

    /// Background visuals such as fill color or gradient.
    background: BackgroundStyle,

    /// Drop shadow rendering for elevation or depth.
    shadow: BoxShadow,
}

impl Default for VButton {
    /// Constructs a `VButton` with vertical stack layout and default styles.
    fn default() -> Self {
        Self {
            name: Name::new("VButton"),
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(0.0),
                ..Default::default()
            },
            botton: Button,
            border: BorderStyle::button(),
            background: BackgroundStyle::button(),
            shadow: BoxShadow::default(),
        }
    }
}

impl View for VButton {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl StackView for VButton {}

impl BackgroundView for VButton {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BorderView for VButton {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl BoxShadowView for VButton {
    fn box_shadow_node(&mut self) -> &mut BoxShadow {
        &mut self.shadow
    }
}

impl PositionView for VButton {}
