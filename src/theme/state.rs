//! MD3 交互状态（State Layer）令牌与颜色混合工具
//!
//! 参考: <https://m3.material.io/foundations/interaction/states>

use gpui::{Hsla, Rgba};

/// hover 状态层不透明度（8%）
pub const HOVER_OPACITY: f32 = 0.08;
/// focus 状态层不透明度（12%）
pub const FOCUS_OPACITY: f32 = 0.12;
/// pressed 状态层不透明度（12%）
pub const PRESSED_OPACITY: f32 = 0.12;
/// dragged 状态层不透明度（16%）
pub const DRAGGED_OPACITY: f32 = 0.16;
/// 禁用态容器不透明度（12%）
pub const DISABLED_CONTAINER_OPACITY: f32 = 0.12;
/// 禁用态内容（文字/图标）不透明度（38%）
pub const DISABLED_CONTENT_OPACITY: f32 = 0.38;

/// 把 `overlay` 颜色以 `opacity` 不透明度叠加到不透明的 `base` 上，
/// 返回混合后的实色。用于在实色容器上绘制 state layer。
pub fn blend(base: Hsla, overlay: Hsla, opacity: f32) -> Hsla {
    let b: Rgba = base.into();
    let o: Rgba = overlay.into();
    Rgba {
        r: b.r + (o.r - b.r) * opacity,
        g: b.g + (o.g - b.g) * opacity,
        b: b.b + (o.b - b.b) * opacity,
        a: b.a,
    }
    .into()
}

/// hover 状态层：在实色 `base` 上叠加 8% 的 `content` 色
pub fn hover_layer(base: Hsla, content: Hsla) -> Hsla {
    blend(base, content, HOVER_OPACITY)
}

/// pressed 状态层：在实色 `base` 上叠加 12% 的 `content` 色
pub fn pressed_layer(base: Hsla, content: Hsla) -> Hsla {
    blend(base, content, PRESSED_OPACITY)
}
