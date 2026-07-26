//! MD3 高度（Elevation）令牌
//!
//! 阴影参数取自 MD3 官方 elevation 规范（level 0–5）。
//! 参考: <https://m3.material.io/styles/elevation/tokens>

use gpui::{point, px, BoxShadow, Hsla, Pixels};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elevation {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

impl Elevation {
    /// 生成该 elevation 等级对应的一组 BoxShadow。
    /// `shadow_color` 一般传 `theme.colors.shadow`（黑色）。
    pub fn shadows(self, shadow_color: Hsla) -> Vec<BoxShadow> {
        let key = |y: f32, blur: f32, spread: f32| BoxShadow {
            color: shadow_color.opacity(0.30),
            offset: point(px(0.), px(y)),
            blur_radius: px(blur),
            spread_radius: px(spread),
            inset: false,
        };
        let ambient = |y: f32, blur: f32, spread: f32| BoxShadow {
            color: shadow_color.opacity(0.15),
            offset: point(px(0.), px(y)),
            blur_radius: px(blur),
            spread_radius: px(spread),
            inset: false,
        };
        match self {
            Elevation::Level0 => vec![],
            Elevation::Level1 => vec![key(1., 2., 0.), ambient(1., 3., 1.)],
            Elevation::Level2 => vec![key(1., 2., 0.), ambient(2., 6., 2.)],
            Elevation::Level3 => vec![key(1., 3., 0.), ambient(4., 8., 3.)],
            Elevation::Level4 => vec![key(2., 3., 0.), ambient(6., 10., 4.)],
            Elevation::Level5 => vec![key(4., 4., 0.), ambient(8., 12., 6.)],
        }
    }

    /// 该等级对应的 dp 值（信息性）
    pub fn dp(self) -> Pixels {
        match self {
            Elevation::Level0 => px(0.),
            Elevation::Level1 => px(1.),
            Elevation::Level2 => px(3.),
            Elevation::Level3 => px(6.),
            Elevation::Level4 => px(8.),
            Elevation::Level5 => px(12.),
        }
    }
}
