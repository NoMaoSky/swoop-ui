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

pub trait UiBase {
    fn name_node(&mut self) -> &mut Name;

    fn node_node(&mut self) -> &mut Node;

    fn name(mut self, name: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.name_node().set(name);
        self
    }

    fn padding(mut self, padding: UiRect) -> Self
    where
        Self: Sized,
    {
        self.node_node().padding = padding;
        self
    }

    fn frame(mut self, width: Val, height: Val) -> Self
    where
        Self: Sized,
    {
        let node = self.node_node();
        node.width = width;
        node.height = height;
        self
    }

    fn width(mut self, width: Val) -> Self
    where
        Self: Sized,
    {
        self.node_node().width = width;
        self
    }

    fn height(mut self, height: Val) -> Self
    where
        Self: Sized,
    {
        self.node_node().height = height;
        self
    }
}

pub trait UiToBundle {
    fn pack(self) -> impl Bundle;
}

impl Plugin for SwoopUiPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
