# md3-gpui

Based on [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui), the GPU-accelerated UI framework from Zed, this is a **Material Design 3** component library. Component specs follow Google’s [material-web](https://github.com/material-components/material-web); the token, motion and theme architecture is ported from [m3fx](https://github.com/Glavo/m3fx) (Apache-2.0).

Pure gpui implementation for rendering; dynamic color uses the `mcu-*` algorithm crates (Material color utilities).

## Features

- **m3fx token system** (`Profile` × `TokenSet`)
  - `TokenSet`: color / typography / shape / elevation / motion / state-layer / component token groups with builder-level overrides
  - `Profile::Baseline2021` and `Profile::Expressive2025` (Expressive shape scale, type scale and motion scheme)
  - Dynamic color: `Theme::from_seed(seed, mode, profile)` via material-color-utilities (HCT), reproducing the material-web baseline palette
- **m3fx motion system** (`md3_gpui::motion`)
  - `MotionScheme`: six semantic roles (fast/default/slow × effects/spatial), standard + expressive presets
  - Closed-form damped spring solver with retargetable `Animatable` values (velocity-preserving retargeting)
  - 13 MD3 easing curves incl. the three-segment emphasized curve; `reduce_motion` support
- **m3fx interaction behaviors**: spring-animated state layers, pointer ripples, spring-driven switch / checkbox / radio / tab-indicator animations
- **Window-level overlay system** (`md3_gpui::overlay`): `OverlayHost` + `show_snackbar` / `show_menu` / `show_tooltip`
- **Components** (aligned with material-web component specs)

  | Category | Components |
  |---|---|
  | Buttons | `Button` (filled / tonal / elevated / outlined / text), `IconButton` (4 variants + toggle), `Fab` (3 sizes / 4 colors / extended) |
  | Selection | `Checkbox`, `RadioButton`, `Switch`, `Slider` (M3 refreshed visuals), `Chip` (assist / filter / input / suggestion) |
  | Containers | `Card` (elevated / filled / outlined), `Dialog`, `List` / `ListItem`, `Divider` |
  | Navigation | `TabBar` / `Tab` (spring-sliding indicator) |
  | Input | `TextField` (outlined, floating label, helper/error text, focus morph) |
  | Overlays | `Snackbar`, `Menu`, `Tooltip` (via `overlay::host`) |
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
            .bg(theme.colors().surface)
            .flex()
            .items_center()
            .justify_center()
            .child(
                Button::new("hello", "Hello MD3")
                    .filled()
                    .leading_icon(IconName::Favorite)
                    .on_click(|_, _, _| println!("clicked!"))
                    .build(cx), // Entity components: build once, keep the handle
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

The demo window shows all components; the header switches toggle light/dark and Baseline/Expressive themes, and the text-field page applies a live dynamic-color seed. It also demonstrates spring ripples, state layers, tab-indicator motion and the overlay system.

> On the first build, cargo will fetch and compile the full gpui repository (a large dependency). This may take a while.

## Theme customization

```rust
use md3_gpui::prelude::*;

// Baseline light/dark (seed #6750A4, Baseline2021)
Theme::set(cx, Theme::dark());

// Dynamic color from a seed, with the Expressive 2025 profile
Theme::set(cx, Theme::from_seed(0x006A6A, ThemeMode::Light, Profile::Expressive2025));

// Override whole token groups via the builder
let tokens = TokenSet::builder(Profile::Baseline2021, cx.theme().colors().clone())
    .with_motion(MotionScheme::expressive())
    .build();
let mut theme = Theme::light();
theme.set_token_set(tokens);
Theme::set(cx, theme);
```

Components read the global theme from `cx.theme()` during `render` (`theme.colors()`, `theme.typography()`, `theme.shapes()`, `theme.motion()`, ...). After replacing the theme, trigger a redraw with `cx.refresh_windows()`.

## Stateful components

Interactive components own their animation state, so they are entities created once and kept:

```rust
// Create (in new(), or the first render — never every frame):
let checkbox = Checkbox::new("agree").build(cx);
let switch = Switch::new("wifi").on_change(|checked, _, _| {}).build(cx);

// Render: entity handles are elements
div().child(checkbox.clone()).child(switch.clone())
```

Simple containers (`Card`, `Divider`, `List` / `ListItem`, `Dialog`, progress indicators) remain stateless `RenderOnce` elements.

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

- [ ] Keyboard focus ring and accessibility (focus-visible state layers, keyboard activation)
- [ ] IME / marked-text input in `TextField`; keyboard navigation for `Menu`
- [ ] Remaining m3fx families: navigation (TopAppBar / NavigationBar / Drawer / Rail), segmented buttons, badges / avatars, color pickers, date & time pickers, TableView / TreeView / Carousel
- [ ] Per-component token coverage beyond the first-phase subset
- [ ] Determinate mode for CircularProgress
- [ ] Menu / tooltip flip-over when exceeding window bounds
- [ ] Roboto font is not distributed with the library; systems without Roboto fall back to the default font

## License

Apache-2.0. Embedded icons are from [Material Symbols](https://fonts.google.com/icons) (Apache-2.0).
