# 🧱 Swoop UI

**Swoop UI** is a modular, ergonomic layout toolkit built on top of Bevy UI. It introduces expressive layout containers like `HStack`, `VStack`, `HGrid`, and `VGrid`, supporting fluent syntax for padding, spacing, border, and background styling.

## ✨ Features

- ✅ Chainable builder-style API (e.g. `.frame().padding().background_color()`)
- ✅ Grid and Stack layout containers with uniform interface
- ✅ Modular traits like `UiBase`, `StackContainer`, `GridContainer`, etc.
- ✅ Seamless integration with Bevy's ECS via `.pack()` → `impl Bundle`

---

## 📦 Example

```rust
use swoop_ui::prelude::*;

fn setup_ui(mut commands: Commands) {
    let layout = HStack::new(AlignItems::Center, Val::Px(12.0))
        .frame(Val::Percent(100.0), Val::Px(64.0))
        .padding(UiRect::horizontal(Val::Px(16.0)))
        .background_color(Color::rgb(0.1, 0.1, 0.2))
        .border_color(Color::WHITE)
        .border_radius(BorderRadius::all(Val::Px(8.0)))
        .name("MainHeader");

    commands.spawn(layout.pack());
}
```

| bevy  | swoop-ui |
|-------|----------|
| 0.16  | 0.1      |
