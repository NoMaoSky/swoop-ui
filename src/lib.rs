//! **Swoop UI** is a modular, ergonomic layout toolkit built on top of Bevy UI.
//! It introduces expressive layout containers like `HStack`, `VStack`, `HGrid`, and `VGrid`,
//! supporting fluent syntax for padding, spacing, border, and background styling.
//! Now only some packaged candies are generated, no additional functions, maybe they will be added later,
//! a plugin is reserved, but it has not been used yet
//!
//! Most methods implement Bundle and can be generated directly.
//! However, some packaging requires multiple levels,
//! so ViewToBundle is implemented and the pack() method is called to convert it into impl Bundle.
//!
//! # UI Layout Overview
//!
//! This crate defines a menu bar container using a horizontal stack layout:
//!
//! ```rust
//! use bevy::prelude::*;
//! use swoop_ui::prelude::*;
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn((
//!         HStack::new(AlignItems::Start, Val::Auto)
//!             .background_color(Srgba::WHITE.into())
//!             .justify_content(JustifyContent::Start)
//!             .pack(),
//!     ));
//! }
//! ```
//!
//! - `HStack` creates a horizontal layout with children aligned at the top.
//! - The background color is set using the impl Into<Color> value.
//! - Contents are left-aligned using `JustifyContent::Start`.
//!
//! Because the impl Bundle is implemented, it can be directly generated using commands.spawn

use std::borrow::Cow;
use std::fmt::Debug;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

// Background UI trait
pub mod background;
// Border UI trait
pub mod border;
// Shadow UI Trait
pub mod shadow;
// Absolute positioning
pub mod position;

/// Button
pub mod button;
/// Layouts and containers
pub mod container;
/// Text
pub mod text;

pub mod prelude {
    pub use super::background::BackgroundView;
    pub use super::border::BorderView;
    pub use super::position::PositionView;
    pub use super::shadow::{BoxShadowView, TextShadowView};
    pub use super::{SwoopUiPlugin, View, ViewToBundle};

    pub use super::button::prelude::*;
    pub use super::container::prelude::*;
    pub use super::text::prelude::*;
}

/// Reserved for future addition of system functions
pub struct SwoopUiPlugin;

/// Provides a builder-style trait for configuring UI elements
/// using a fluent interface.
///
/// Types implementing `View` must define how to access their `Name`
/// and `Node` components. This trait simplifies layout and styling
/// tasks through expressive method chaining.
pub trait View: Debug + Clone + Default {
    /// Creates a new instance using the type's `Default` implementation.
    ///
    /// This is a convenient shorthand when no customization is needed.
    fn new() -> Self {
        Self::default()
    }

    /// Constructs a new instance and sets its `Name` component.
    ///
    /// # Arguments
    /// * `name` - A name to assign to this instance, typically used for entity
    ///   identification or debugging purposes.
    fn from_name(name: impl Into<Cow<'static, str>>) -> Self {
        let mut view = Self::default();
        view.name_node().set(name);
        view
    }

    /// Provides mutable access to the underlying `Name` component.
    ///
    /// This allows for direct manipulation or inspection of the `Name`.
    fn name_node(&mut self) -> &mut Name;

    /// Provides mutable access to the underlying `Node` component.
    ///
    /// Useful when applying layout or transform properties.
    fn node_node(&mut self) -> &mut Node;

    /// Sets the `Name` component of this instance.
    ///
    /// Helpful for tracking or debugging UI entities.
    ///
    /// # Arguments
    /// * `name` - The name to assign, provided as a `&str`, `String`, or `Cow`.
    fn name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name_node().set(name);
        self
    }

    /// Sets the padding (insets) around the content of the `Node`.
    ///
    /// # Arguments
    /// * `padding` - A `UiRect` defining spacing on all four sides.
    fn padding(mut self, padding: UiRect) -> Self {
        self.node_node().padding = padding;
        self
    }

    /// Sets both the width and height of the `Node`.
    ///
    /// # Arguments
    /// * `width` - The desired width using `Val::Px`, `Val::Percent`, etc.
    /// * `height` - The desired height using the same `Val` enum.
    fn frame(mut self, width: Val, height: Val) -> Self {
        let node = self.node_node();
        node.width = width;
        node.height = height;
        self
    }

    /// Sets the width of the `Node`.
    ///
    /// # Arguments
    /// * `val` - The desired width value.
    fn width(mut self, val: Val) -> Self {
        self.node_node().width = val;
        self
    }

    /// Sets the maximum width constraint of the `Node`.
    ///
    /// Prevents the node from expanding beyond this limit.
    fn max_width(mut self, val: Val) -> Self {
        self.node_node().max_width = val;
        self
    }

    /// Sets the minimum width constraint of the `Node`.
    ///
    /// Ensures the node does not shrink below this value.
    fn min_width(mut self, val: Val) -> Self {
        self.node_node().min_width = val;
        self
    }

    /// Sets the height of the `Node`.
    ///
    /// # Arguments
    /// * `val` - The desired height value.
    fn height(mut self, val: Val) -> Self {
        self.node_node().height = val;
        self
    }

    /// Sets the maximum height constraint of the `Node`.
    ///
    /// Prevents the node from growing beyond this value.
    fn max_height(mut self, val: Val) -> Self {
        self.node_node().max_height = val;
        self
    }

    /// Sets the minimum height constraint of the `Node`.
    ///
    /// Ensures the node maintains at least this size.
    fn min_height(mut self, val: Val) -> Self {
        self.node_node().min_height = val;
        self
    }

    /// Sets the `flex_grow` factor of the `Node`.
    ///
    /// Determines how much the node should grow relative to its siblings.
    fn flex_grow(mut self, grow: f32) -> Self {
        self.node_node().flex_grow = grow;
        self
    }

    /// Sets the `flex_shrink` factor of the `Node`.
    ///
    /// Controls how much the node should shrink when space is constrained.
    fn flex_shrink(mut self, shrink: f32) -> Self {
        self.node_node().flex_shrink = shrink;
        self
    }

    /// Assigns this node to a specific grid row in a grid layout.
    ///
    /// # Arguments
    /// * `row` - The grid row position.
    fn grid_row(mut self, row: GridPlacement) -> Self {
        self.node_node().grid_row = row;
        self
    }

    /// Assigns this node to a specific grid column in a grid layout.
    ///
    /// # Arguments
    /// * `column` - The grid column position.
    fn grid_column(mut self, column: GridPlacement) -> Self {
        self.node_node().grid_column = column;
        self
    }
}

pub trait ViewToBundle: View {
    fn pack(self) -> impl Bundle;
}

impl Plugin for SwoopUiPlugin {
    fn build(&self, _: &mut App) {}
}
