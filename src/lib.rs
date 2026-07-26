//! # md3-gpui
//!
//! 基于 [gpui](https://github.com/zed-industries/zed/tree/main/crates/gpui) 的
//! Material Design 3 组件库，设计规范对齐
//! [material-web](https://github.com/material-components/material-web)。
//!
//! ## 快速开始
//!
//! ```ignore
//! use gpui::*;
//! use gpui_platform::application;
//! use md3_gpui::prelude::*;
//!
//! fn main() {
//!     application()
//!         .with_assets(Md3Assets)          // 安装内嵌图标资源
//!         .run(|cx| {
//!             md3_gpui::init(cx);          // 安装默认（亮色）主题
//!             cx.open_window(WindowOptions::default(), |_, cx| {
//!                 cx.new(|_| MyApp)
//!             }).unwrap();
//!         });
//! }
//!
//! impl Render for MyApp {
//!     fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         let theme = cx.theme();
//!         div()
//!             .size_full()
//!             .bg(theme.colors.surface)
//!             .child(Button::new("hi", "Hello MD3").on_click(|_, _, _| {}))
//!     }
//! }
//! ```

pub mod assets;
pub mod components;
pub mod icon;
pub mod theme;

pub use assets::Md3Assets;
pub use components::*;
pub use icon::{Icon, IconName};
pub use theme::{ActiveTheme, Theme, ThemeMode};

use gpui::App;

/// 安装默认（亮色）主题。若已有主题则不覆盖。
pub fn init(cx: &mut App) {
    if !cx.has_global::<Theme>() {
        cx.set_global(Theme::light());
    }
}

/// 常用导出集合
pub mod prelude {
    pub use crate::assets::Md3Assets;
    pub use crate::components::*;
    pub use crate::icon::{Icon, IconName};
    pub use crate::theme::{
        ActiveTheme, ColorScheme, Elevation, Theme, ThemeMode, TypeScale, TypeStyle,
    };
}
