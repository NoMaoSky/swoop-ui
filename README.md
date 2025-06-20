# 🧱 Swoop UI
[![Crates.io](https://img.shields.io/crates/v/swoop-ui)](https://crates.io/crates/swoop-ui)
[![docs](https://docs.rs/swoop-ui/badge.svg)](https://docs.rs/swoop-ui/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/NoMaoSky/swoop-ui/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/swoop-ui)](https://crates.io/crates/swoop-ui)
[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)

# I found that impl Bundle is the best way, right?
> Most methods implement Bundle and can be generated directly.
> However, some packaging requires multiple levels,
> so ViewToBundle is implemented and the pack() method is called to convert it into impl Bundle.

**Swoop UI** is a modular, ergonomic layout toolkit built on top of Bevy UI. It introduces expressive layout containers like `HStack`, `VStack`, `HGrid`, and `VGrid`, supporting fluent syntax for padding, spacing, border, and background styling.
Now only some packaged candies are generated, no additional functions, maybe they will be added later, a plugin is reserved, but it has not been used yet

| Type       | Flex container | Grid container | Button  |
|------------|----------------|----------------|---------|
| Horizontal | HStack         | HGrid          | HButton |
| Vertical   | VStack         | VGrid          | VButton |

## Trait function support

| Type       | StackView | GridView | BorderView | BackgroundView | BoxShadowView | PositionView | TextView | TextShadowView |
|------------|-----------|----------|------------|----------------|---------------|--------------|----------|----------------|
| HStack     |     ✅     |     ✅    |      ✅     |        ✅       |       ✅       |       ✅      |          |                |
| VStack     |     ✅     |     ✅    |      ✅     |        ✅       |       ✅       |       ✅      |          |                |
| HGrid      |     ✅     |     ✅    |      ✅     |        ✅       |       ✅       |       ✅      |          |                |
| VGrid      |     ✅     |     ✅    |      ✅     |        ✅       |       ✅       |       ✅      |          |                |
| HButton    |     ✅     |     ✅    |      ✅     |        ✅       |       ✅       |       ✅      |          |                |
| VButton    |     ✅     |     ✅    |      ✅     |        ✅       |       ✅       |       ✅      |          |                |
| Text       |           |          |      ✅     |        ✅       |       ✅       |       ✅      |     ✅    |        ✅       |
| TextButton |           |          |      ✅     |        ✅       |       ✅       |       ✅      |     ✅    |        ✅       |

## ✨ Features

- ✅ Chainable builder-style API (e.g. `.frame().padding().background_color()`)
- ✅ Grid and Stack layout containers with uniform interface
- ✅ Modular traits like `View`, `StackView`, `GridView`, `BorderView`, `BackgroundView`, `ShadowView`, etc.

---

## 📦 Example

```rust
use swoop_ui::prelude::*;

fn setup_ui(mut commands: Commands) {
    commands.spawn(
        HStack::new()
            .name("MainHeader1")
            .frame(Val::Percent(100.0), Val::Px(64.0))
            .padding(UiRect::horizontal(Val::Px(16.0)))
            .background_color(Color::rgb(0.1, 0.1, 0.2))
            .border_color(Color::WHITE)
            .border_radius(BorderRadius::all(Val::Px(8.0)))
    );   
}
```
## Bevy Engine version compatibility

| bevy  | swoop-ui |
|-------|----------|
| 0.16  | 0.1      |
