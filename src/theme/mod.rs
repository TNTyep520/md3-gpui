//! MD3 主题系统：颜色 / 字体 / 形状 / 高度 / 状态层令牌的聚合。
//!
//! 用法：
//! ```ignore
//! use md3_gpui::prelude::*;
//!
//! application().run(|cx| {
//!     md3_gpui::init(cx);                 // 安装默认亮色主题
//!     // 或者：Theme::set(cx, Theme::dark());
//! });
//!
//! // 在任意 render 中：
//! let theme = cx.theme();
//! div().bg(theme.colors.surface)
//! ```

mod color;
mod elevation;
mod shape;
mod state;
mod typography;

pub use color::{hex, ColorScheme};
pub use elevation::Elevation;
pub use shape::Shapes;
pub use state::*;
pub use typography::{TypeScale, TypeStyle};

use gpui::{App, Global, SharedString};

/// 主题模式
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

/// MD3 主题（作为 gpui Global 存储）
#[derive(Clone, Debug)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ColorScheme,
    pub typography: TypeScale,
    pub shapes: Shapes,
    /// 全局字体族（默认 Roboto，缺失时由系统回退）
    pub font_family: SharedString,
}

impl Global for Theme {}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ColorScheme::light(),
            typography: TypeScale::default(),
            shapes: Shapes::default(),
            font_family: "Roboto".into(),
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ColorScheme::dark(),
            typography: TypeScale::default(),
            shapes: Shapes::default(),
            font_family: "Roboto".into(),
        }
    }

    /// 读取全局主题
    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    /// 替换全局主题（切换亮/暗色后请触发一次重绘，如 `cx.refresh_windows()` 或视图 `cx.notify()`）
    pub fn set(cx: &mut App, theme: Theme) {
        cx.set_global(theme);
    }
}

/// 便捷 trait：`cx.theme()`
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}
