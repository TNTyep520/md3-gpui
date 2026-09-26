//! MD3 Divider（对应 material-web 的 `md-divider`）
//!
//! 1dp 分割线，颜色 outline-variant，支持水平/垂直与 inset。

use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*};

use crate::theme::ActiveTheme;

/// MD3 分割线
#[derive(IntoElement)]
pub struct Divider {
    vertical: bool,
    inset: bool,
}

impl Divider {
    /// 水平分割线
    pub fn horizontal() -> Self {
        Self {
            vertical: false,
            inset: false,
        }
    }

    /// 垂直分割线
    pub fn vertical() -> Self {
        Self {
            vertical: true,
            inset: false,
        }
    }

    /// 两端缩进 16dp
    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }
}

impl RenderOnce for Divider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = DividerStyle::resolve(cx.theme().token_set(), self.inset);
        let color = style.color;
        if self.vertical {
            // 外层占位，内层着色，避免 margin 溢出
            div()
                .w(style.thickness)
                .h_full()
                .flex_none()
                .when(self.inset, |el| el.py(style.inset))
                .child(div().w(style.thickness).h_full().bg(color))
        } else {
            div()
                .h(style.thickness)
                .w_full()
                .flex_none()
                .when(self.inset, |el| el.px(style.inset))
                .child(div().h(style.thickness).w_full().bg(color))
        }
    }
}

pub use appearance::DividerStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// MD3 分隔线样式。
    #[derive(Clone, Copy, Debug)]
    pub struct DividerStyle {
        /// 颜色。
        pub color: Hsla,
        /// 厚度。
        pub thickness: Pixels,
        /// inset 缩进。
        pub inset: Pixels,
    }
    impl DividerStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet, inset: bool) -> Self {
            Self {
                color: tokens.colors.outline_variant,
                thickness: px(1.),
                inset: if inset { px(16.) } else { px(0.) },
            }
        }
    }
}
