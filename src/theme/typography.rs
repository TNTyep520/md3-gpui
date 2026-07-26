//! MD3 字体排印比例（Type Scale）
//!
//! 对应 material-web 的 `md-sys-typescale` 令牌。
//! 参考: <https://m3.material.io/styles/typography/type-scale-tokens>

use gpui::{px, FontWeight, Pixels, Styled};

/// 单个字体样式令牌
#[derive(Clone, Copy, Debug)]
pub struct TypeStyle {
    pub size: Pixels,
    pub line_height: Pixels,
    pub weight: FontWeight,
}

impl TypeStyle {
    pub const fn new(size: f32, line_height: f32, weight: FontWeight) -> Self {
        Self {
            size: px(size),
            line_height: px(line_height),
            weight,
        }
    }

    /// 把该字体令牌应用到任意 Styled 元素上
    pub fn apply<E: Styled>(&self, el: E) -> E {
        el.text_size(self.size)
            .line_height(self.line_height)
            .font_weight(self.weight)
    }
}

/// MD3 完整 Type Scale（display / headline / title / body / label × large / medium / small）
#[derive(Clone, Debug)]
pub struct TypeScale {
    pub display_large: TypeStyle,
    pub display_medium: TypeStyle,
    pub display_small: TypeStyle,
    pub headline_large: TypeStyle,
    pub headline_medium: TypeStyle,
    pub headline_small: TypeStyle,
    pub title_large: TypeStyle,
    pub title_medium: TypeStyle,
    pub title_small: TypeStyle,
    pub body_large: TypeStyle,
    pub body_medium: TypeStyle,
    pub body_small: TypeStyle,
    pub label_large: TypeStyle,
    pub label_medium: TypeStyle,
    pub label_small: TypeStyle,
}

impl Default for TypeScale {
    fn default() -> Self {
        Self {
            display_large: TypeStyle::new(57., 64., FontWeight::NORMAL),
            display_medium: TypeStyle::new(45., 52., FontWeight::NORMAL),
            display_small: TypeStyle::new(36., 44., FontWeight::NORMAL),
            headline_large: TypeStyle::new(32., 40., FontWeight::NORMAL),
            headline_medium: TypeStyle::new(28., 36., FontWeight::NORMAL),
            headline_small: TypeStyle::new(24., 32., FontWeight::NORMAL),
            title_large: TypeStyle::new(22., 28., FontWeight::NORMAL),
            title_medium: TypeStyle::new(16., 24., FontWeight::MEDIUM),
            title_small: TypeStyle::new(14., 20., FontWeight::MEDIUM),
            body_large: TypeStyle::new(16., 24., FontWeight::NORMAL),
            body_medium: TypeStyle::new(14., 20., FontWeight::NORMAL),
            body_small: TypeStyle::new(12., 16., FontWeight::NORMAL),
            label_large: TypeStyle::new(14., 20., FontWeight::MEDIUM),
            label_medium: TypeStyle::new(12., 16., FontWeight::MEDIUM),
            label_small: TypeStyle::new(11., 16., FontWeight::MEDIUM),
        }
    }
}
