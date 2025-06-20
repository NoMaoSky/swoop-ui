use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

use crate::View;
use crate::background::BackgroundStyle;
use crate::border::{BorderStyle, BorderView};
use crate::prelude::{BackgroundView, PositionView};
use crate::shadow::{BoxShadowView, TextShadowView};

pub type SText = SwoopText;

/// A styled text view bundle that supports layout, borders, background,
/// shadows, and positioning within the UI system.
///
/// This component integrates common UI traits to provide a rich,
/// CSS-inspired view abstraction.
#[derive(Bundle, Debug, Clone)]
pub struct SwoopText {
    /// Name for debugging or entity identification.
    name: Name,

    /// Layout and sizing node (e.g., width, height, margin).
    node: Node,

    /// Border styling (width, color, radius, etc.).
    border: BorderStyle,

    /// Background styling (color, image, etc.).
    background: BackgroundStyle,

    /// Outer box shadow styling (e.g., drop shadow).
    box_shadow: BoxShadow,

    /// Inner text shadow styling.
    text_shadow: TextShadow,
}

impl Default for SwoopText {
    /// Returns a `SwoopText` instance with default visual styles.
    fn default() -> Self {
        Self {
            name: Name::new("SwoopText"),
            node: Node {
                ..Default::default()
            },
            border: BorderStyle::default(),
            background: BackgroundStyle::default(),
            box_shadow: BoxShadow::default(),
            text_shadow: TextShadow::default(),
        }
    }
}

impl View for SwoopText {
    fn name_node(&mut self) -> &mut Name {
        &mut self.name
    }

    fn node_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl BorderView for SwoopText {
    fn border_node(&mut self) -> &mut BorderStyle {
        &mut self.border
    }
}

impl BackgroundView for SwoopText {
    fn background_node(&mut self) -> &mut BackgroundStyle {
        &mut self.background
    }
}

impl BoxShadowView for SwoopText {
    fn box_shadow_node(&mut self) -> &mut BoxShadow {
        &mut self.box_shadow
    }
}

impl TextShadowView for SwoopText {
    fn text_shadow_node(&mut self) -> &mut TextShadow {
        &mut self.text_shadow
    }
}

impl PositionView for SwoopText {}
