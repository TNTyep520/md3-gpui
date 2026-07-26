# md3-gpui

Based on [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui), the GPU-accelerated UI framework from Zed, this is a **Material Design 3** component library that follows Google’s [material-web](https://github.com/material-components/material-web) design tokens and specifications.

Pure gpui implementation with no other UI library dependencies.

## Features

- **Complete MD3 theme token system**
  - Colors: all MD3 color roles (primary / secondary / tertiary / error / surface series / outline, etc.), built-in baseline light and dark schemes, replaceable for dynamic theming
  - Typography: full type scale (display / headline / title / body / label × large / medium / small)
  - Shapes: extra-small → extra-large corner radius tokens
  - Elevation: Level 0–5 shadow options
  - State layers: hover / focus / pressed / dragged opacity and blend utilities
- **Components** (aligned with material-web component specs)

  | Category | Components |
  |---|---|
  | Buttons | `Button` (filled / tonal / elevated / outlined / text), `IconButton` (4 variants + toggle), `Fab` (3 sizes / 4 colors / extended) |
  | Selection | `Checkbox`, `RadioButton`, `Switch`, `Slider` (M3 refreshed visuals), `Chip` (assist / filter / input / suggestion) |
  | Containers | `Card` (elevated / filled / outlined), `Dialog`, `List` / `ListItem`, `Divider` |
  | Navigation | `TabBar` / `Tab` |
  | Progress | `LinearProgress` (determinate / indeterminate), `CircularProgress` |

- Embedded Material Symbols icon subset (`Icon` / `IconName`, Apache-2.0)

## Quick Start

```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
md3-gpui = { path = "../md3-gpui" }

# The application entry point also needs a platform layer:
gpui_platform = { git = "https://github.com/zed-industries/zed", features = [
    "font-kit", "x11", "wayland", "runtime_shaders",
] }
```

> Note: zed does not tag gpui releases. It is recommended to pin the same `rev = "<commit>"` for both `gpui` and `gpui_platform` in your `Cargo.toml`.

```rust
use gpui::*;
use gpui_platform::application;
use md3_gpui::prelude::*;

struct MyApp;

impl Render for MyApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .size_full()
            .bg(theme.colors.surface)
            .flex()
            .items_center()
            .justify_center()
            .child(
                Button::new("hello", "Hello MD3")
                    .filled()
                    .leading_icon(IconName::Favorite)
                    .on_click(|_, _, _| println!("clicked!")),
            )
    }
}

fn main() {
    application()
        .with_assets(Md3Assets)   // Icon assets (use Md3Assets::with_fallback if you already have your own AssetSource)
        .run(|cx| {
            md3_gpui::init(cx);   // Install the default light theme
            cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| MyApp))
                .unwrap();
            cx.activate(true);
        });
}
```

## Run the demo

```bash
cargo run --example demo
```

The demo window shows all components and includes a top-right switch to toggle light/dark theme.

> On the first build, cargo will fetch and compile the full gpui repository (a large dependency). This may take a while.

## Theme customization

```rust
use md3_gpui::prelude::*;
use md3_gpui::theme::hex;

// Switch to dark theme
Theme::set(cx, Theme::dark());

// Customize brand color by replacing roles in the ColorScheme
let mut theme = Theme::light();
theme.colors.primary = hex(0x006A6A);
theme.colors.on_primary = hex(0xFFFFFF);
Theme::set(cx, theme);
```

Components read the global theme from `cx.theme()` during `render`. After replacing the theme, trigger a redraw with `cx.notify()` to apply the new theme.

## Design mapping

| md3-gpui | material-web | Key specs |
|---|---|---|
| `Button` | `md-filled-button`, etc. | 40dp height, capsule shape, label-large, 24dp horizontal padding |
| `IconButton` | `md-icon-button`, etc. | 40dp container, 24dp icon |
| `Fab` | `md-fab` | 40 / 56 / 96dp, 12 / 16 / 28dp corner radius, Level3 shadow |
| `Checkbox` | `md-checkbox` | 18dp box, 2dp corner radius, 40dp touch target |
| `RadioButton` | `md-radio` | 20dp outer ring, 10dp inner dot |
| `Switch` | `md-switch` | 52×32dp track, 16 / 24dp thumb |
| `Slider` | `md-slider` | M3 refreshed: 16dp track + 4×44dp thumb |
| `Chip` | `md-*-chip` | 32dp height, 8dp corner radius |
| `Card` | labs card | 12dp corner radius, elevated uses Level1 shadow |
| `Dialog` | `md-dialog` | 280–560dp width, 28dp corners, Level3, 32% scrim |
| `ListItem` | `md-list-item` | single line 56dp / two-line 72dp |
| `TabBar` | `md-tabs` | 48dp height (64dp with icon), 3dp indicator |
| `LinearProgress` | `md-linear-progress` | 4dp track |

## Known limitations / roadmap

- [ ] Ripple effect (currently approximated by MD3 state layer hover/pressed blend)
- [ ] Keyboard focus ring and accessibility
- [ ] TextField, Menu, Select, Snackbar, NavigationBar / Drawer
- [ ] Dynamic color generation based on HCT (Material You from seed color)
- [ ] Determinate mode for CircularProgress
- [ ] Roboto font is not distributed with the library; systems without Roboto fall back to the default font

## License

Apache-2.0. Embedded icons are from [Material Symbols](https://fonts.google.com/icons) (Apache-2.0).
