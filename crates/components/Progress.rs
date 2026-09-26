//! MD3 Progress Indicators（对应 material-web 的 `md-linear-progress` / `md-circular-progress`）
//!
//! - LinearProgress：4dp 轨道；`value(Some(f))` 为确定进度，`None` 为不确定动画。
//! - CircularProgress：48dp 旋转圆弧（不确定进度）。

use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, RenderOnce, Transformation, Window, div,
    ease_in_out, percentage, prelude::*, px, relative, svg,
};
use std::time::Duration;

use crate::theme::ActiveTheme;

/// MD3 线性进度条
#[derive(IntoElement)]
pub struct LinearProgress {
    id: ElementId,
    /// Some(0.0..=1.0) 确定进度；None 为不确定动画
    value: Option<f32>,
}

impl LinearProgress {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(value.clamp(0., 1.));
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.value = None;
        self
    }
}

impl RenderOnce for LinearProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = LinearProgressStyle::resolve(cx.theme().token_set());
        let active = style.active_color;
        let track = style.track_color;

        let container = div()
            .id(self.id)
            .w_full()
            .h(style.height)
            .rounded(style.corner_radius)
            .bg(track)
            .overflow_hidden();

        match self.value {
            Some(value) => {
                container.child(div().h_full().w(relative(value)).rounded_full().bg(active))
            }
            None => container.child(
                div().relative().size_full().child(
                    div()
                        .absolute()
                        .top_0()
                        .h_full()
                        .w(relative(0.4))
                        .rounded_full()
                        .bg(active)
                        .with_animation(
                            "md3-linear-indeterminate",
                            Animation::new(Duration::from_millis(1200))
                                .repeat()
                                .with_easing(ease_in_out),
                            |el, delta| el.left(relative(-0.4 + delta * 1.4)),
                        ),
                ),
            ),
        }
    }
}

/// MD3 环形进度指示器（不确定进度）
#[derive(IntoElement)]
pub struct CircularProgress {
    size: gpui::Pixels,
}

impl CircularProgress {
    pub fn new() -> Self {
        Self { size: px(48.) }
    }

    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = size;
        self
    }
}

impl Default for CircularProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for CircularProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = CircularProgressStyle::resolve(cx.theme().token_set()).color;
        svg()
            .path(crate::assets::PROGRESS_ARC_SVG_PATH)
            .size(self.size)
            .text_color(color)
            .with_animation(
                "md3-circular-indeterminate",
                Animation::new(Duration::from_millis(1000)).repeat(),
                |el, delta| el.with_transformation(Transformation::rotate(percentage(delta))),
            )
    }
}

pub use appearance::{CircularProgressStyle, LinearProgressStyle};

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// 线性进度条样式。
    #[derive(Clone, Copy, Debug)]
    pub struct LinearProgressStyle {
        /// 活动条颜色。
        pub active_color: Hsla,
        /// 轨道颜色。
        pub track_color: Hsla,
        /// 高度。
        pub height: Pixels,
        /// 圆角。
        pub corner_radius: Pixels,
    }
    impl LinearProgressStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet) -> Self {
            let colors = &tokens.colors;
            Self {
                active_color: colors.primary,
                track_color: colors.secondary_container,
                height: px(4.),
                corner_radius: tokens.shapes.full,
            }
        }
    }
    /// 环形进度指示器样式。
    #[derive(Clone, Copy, Debug)]
    pub struct CircularProgressStyle {
        /// 颜色。
        pub color: Hsla,
        /// 默认尺寸。
        pub size: Pixels,
    }
    impl CircularProgressStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet) -> Self {
            Self {
                color: tokens.colors.primary,
                size: px(48.),
            }
        }
    }
}
