use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_image::prelude::*;
use bevy_ui::prelude::*;

use crate::UiToBundle;

pub mod h_stack;
pub mod v_stack;

pub mod prelude {
    pub use super::h_stack::*;
    pub use super::v_stack::*;
    pub use super::{
        BackgroundContainer, BackgroundStyle, BorderContainer, BorderStyle, StackContainer,
    };
}

pub trait StackContainer: Default {
    fn node_node(&mut self) -> &mut Node;

    fn new(align_items: AlignItems, gap: Val) -> Self
    where
        Self: Sized,
    {
        let mut stack = Self::default();
        stack.node_node().align_items = align_items;
        stack.spacing(gap)
    }

    fn justify_content(mut self, justify: JustifyContent) -> Self
    where
        Self: Sized,
    {
        self.node_node().justify_content = justify;
        self
    }

    fn align_items(mut self, align_items: AlignItems) -> Self
    where
        Self: Sized,
    {
        self.node_node().align_items = align_items;
        self
    }

    fn spacing(mut self, gap: Val) -> Self
    where
        Self: Sized,
    {
        let node = self.node_node();
        match node.flex_direction {
            FlexDirection::Row => node.column_gap = gap,
            FlexDirection::Column => node.row_gap = gap,
            _ => {}
        }
        self
    }
}

pub trait BackgroundContainer {
    fn background_node(&mut self) -> &mut BackgroundStyle;

    fn background_color(mut self, color: impl Into<Color>) -> Self
    where
        Self: Sized,
    {
        *self.background_node() = BackgroundStyle::Color(color.into());
        self
    }

    fn background_image(mut self, image: Handle<Image>) -> Self
    where
        Self: Sized,
    {
        *self.background_node() = BackgroundStyle::Image(image);
        self
    }
}

pub enum BackgroundStyle {
    Color(Color),
    Image(Handle<Image>),
}

impl UiToBundle for BackgroundStyle {
    fn pack(self) -> impl Bundle {
        match self {
            BackgroundStyle::Color(color) => (
                BackgroundColor(color),
                ImageNode::solid_color(Srgba::NONE.into()),
            ),
            BackgroundStyle::Image(handle) => {
                (BackgroundColor(Srgba::NONE.into()), ImageNode::new(handle))
            }
        }
    }
}

impl Default for BackgroundStyle {
    fn default() -> Self {
        BackgroundStyle::Color(Srgba::NONE.into())
    }
}

pub trait BorderContainer {
    fn border_node(&mut self) -> &mut BorderStyle;

    fn border_color(mut self, border_color: impl Into<Color>) -> Self
    where
        Self: Sized,
    {
        self.border_node().border_color.0 = border_color.into();
        self
    }

    fn border_radius(mut self, border_radius: BorderRadius) -> Self
    where
        Self: Sized,
    {
        self.border_node().border_radius = border_radius;
        self
    }
}

pub struct BorderStyle {
    border_radius: BorderRadius,
    border_color: BorderColor,
}

impl UiToBundle for BorderStyle {
    fn pack(self) -> impl Bundle {
        (self.border_radius, self.border_color)
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            border_radius: BorderRadius::ZERO,
            border_color: BorderColor(Srgba::NONE.into()),
        }
    }
}
