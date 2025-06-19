//! **Swoop UI** is a modular, ergonomic layout toolkit built on top of Bevy UI.
//! It introduces expressive layout containers like `HStack`, `VStack`, `HGrid`, and `VGrid`,
//! supporting fluent syntax for padding, spacing, border, and background styling.
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
//! The layout is finalized with `.pack()` and passed into `commands.spawn` to be spawn in the entity world.

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

/// Layouts and containers
pub mod container;

pub mod prelude {
    pub use super::background::*;
    pub use super::border::*;
    pub use super::container::prelude::*;
    pub use super::shadow::*;
    pub use super::{SwoopUiPlugin, View};
}

/// Reserved for future addition of system functions
pub struct SwoopUiPlugin;

/// Provides common builder-style methods for UI configuration
pub trait View: Bundle + Debug + Clone + Default {
    /// Returns a mutable reference to the entity's Name component
    fn name_node(&mut self) -> &mut Name;

    /// Returns a mutable reference to the entity's Node component
    fn node_node(&mut self) -> &mut Node;

    /// Sets the Name component
    fn name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name_node().set(name);
        self
    }

    /// Applies padding to the Node
    fn padding(mut self, padding: UiRect) -> Self {
        self.node_node().padding = padding;
        self
    }

    /// Sets width and height of the Node
    fn frame(mut self, width: Val, height: Val) -> Self {
        let node = self.node_node();
        node.width = width;
        node.height = height;
        self
    }

    /// Sets the width of the Node
    fn width(mut self, width: Val) -> Self {
        self.node_node().width = width;
        self
    }

    /// Sets the height of the Node
    fn height(mut self, height: Val) -> Self {
        self.node_node().height = height;
        self
    }
}

impl Plugin for SwoopUiPlugin {
    fn build(&self, _: &mut App) {}
}
