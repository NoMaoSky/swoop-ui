use std::borrow::Cow;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

pub mod container;

pub mod prelude {
    pub use super::container::prelude::*;
    pub use super::{SwoopUiPlugin, UiBase, UiToBundle};
}

pub struct SwoopUiPlugin;

/// Provides common builder-style methods for UI configuration
pub trait UiBase {
    /// Returns a mutable reference to the entity's Name component
    fn name_node(&mut self) -> &mut Name;

    /// Returns a mutable reference to the entity's Node component
    fn node_node(&mut self) -> &mut Node;

    /// Sets the Name component
    fn name(mut self, name: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.name_node().set(name);
        self
    }

    /// Applies padding to the Node
    fn padding(mut self, padding: UiRect) -> Self
    where
        Self: Sized,
    {
        self.node_node().padding = padding;
        self
    }

    /// Sets width and height of the Node
    fn frame(mut self, width: Val, height: Val) -> Self
    where
        Self: Sized,
    {
        let node = self.node_node();
        node.width = width;
        node.height = height;
        self
    }

    /// Sets the width of the Node
    fn width(mut self, width: Val) -> Self
    where
        Self: Sized,
    {
        self.node_node().width = width;
        self
    }

    /// Sets the height of the Node
    fn height(mut self, height: Val) -> Self
    where
        Self: Sized,
    {
        self.node_node().height = height;
        self
    }
}

/// Converts a custom UI builder into a Bevy-compatible Bundle
pub trait UiToBundle {
    /// Consumes the UI builder and produces a bundle
    fn pack(self) -> impl Bundle;
}

impl Plugin for SwoopUiPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
