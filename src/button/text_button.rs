use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_ui::prelude::*;

use crate::background::BackgroundStyle;
use crate::border::{BorderStyle, BorderView};
use crate::prelude::{BackgroundView, PositionView, StackView};
use crate::shadow::{BoxShadowView, TextShadowView};
use crate::text::{TextStyle, TextView};
use crate::{View, ViewToBundle};

/// A horizontally laid-out button view that includes styling for border, background,
/// box shadow, and text shadow.
///
/// `TextButton` is a Bevy UI bundle suited for building rich, styled buttons with optional
/// text shadow effects. It uses `FlexDirection::Row` to horizontally align its children,
/// and implements positioning and style-related traits such as `BorderView`, `BackgroundView`,
/// `BoxShadowView`, `TextShadowView`, and `PositionView` (if implemented).
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
/// commands.spawn(TextButton::default()
///     .text("OK")
///     .text_color(Color::WHITE)
///     .font(my_font)
///     .background_color(Color::rgb(0.2, 0.6, 0.8)));
/// ```
#[derive(Debug, Clone)]
pub struct TextButton {
    /// Name tag for debugging or entity inspection.
    name: Name,

    /// Layout node controlling size, flex behavior, and spacing.
    node: Node,

    /// Bevy's built-in Button marker for interaction detection.
    botton: Button,

    /// Visual styling for borders (widths, colors, radius).
    border: BorderStyle,

    /// Visual background style (fill, gradient, texture).
    background: BackgroundStyle,

    /// Optional outer shadow to simulate depth or elevation.
    box_shadow: BoxShadow,

    /// Text content, color, font, size
    text: TextStyle,

    /// Inner shadow applied to the text content.
    text_shadow: TextShadow,
}

impl Default for TextButton {
    fn default() -> Self {
        Self {
            name: Name::new("TextButton"),
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
            box_shadow: BoxShadow::default(),
            text: TextStyle::button(),
            text_shadow: TextShadow {
                offset: Vec2::ZERO,
                color: Srgba::NONE.into(),
            },
        }
    }
}

impl View for TextButton {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl ViewToBundle for TextButton {
    fn pack(self) -> impl Bundle {
        (
            self.name,
            self.node,
            self.botton,
            self.border,
            self.background,
            self.box_shadow,
            children![self.text, self.text_shadow],
        )
    }
}

impl StackView for TextButton {}

impl BackgroundView for TextButton {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BorderView for TextButton {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl BoxShadowView for TextButton {
    fn box_shadow_node(&mut self) -> &mut BoxShadow {
        &mut self.box_shadow
    }
}

impl TextView for TextButton {
    fn text_node(&mut self) -> &mut crate::text::TextStyle {
        &mut self.text
    }
}

impl TextShadowView for TextButton {
    fn text_shadow_node(&mut self) -> &mut TextShadow {
        &mut self.text_shadow
    }
}

impl PositionView for TextButton {}
