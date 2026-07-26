//! MD3 形状（Shape）令牌
//!
//! 对应 material-web 的 `md-sys-shape` 圆角令牌。
//! `full`（胶囊形）请直接使用 gpui 的 `.rounded_full()`。

use gpui::{px, Pixels};

#[derive(Clone, Copy, Debug)]
pub struct Shapes {
    pub none: Pixels,
    pub extra_small: Pixels,
    pub small: Pixels,
    pub medium: Pixels,
    pub large: Pixels,
    pub extra_large: Pixels,
}

impl Default for Shapes {
    fn default() -> Self {
        Self {
            none: px(0.),
            extra_small: px(4.),
            small: px(8.),
            medium: px(12.),
            large: px(16.),
            extra_large: px(28.),
        }
    }
}
