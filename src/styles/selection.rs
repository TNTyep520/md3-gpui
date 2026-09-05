//! 选择控件样式（对应 m3fx `styles/controls/selection.css`）。

use gpui::{Hsla, Pixels, px};

use crate::theme::{ColorScheme, TokenSet};

/// MD3 Switch 样式（端点颜色 + 几何；进度插值在组件内完成）。
#[derive(Clone, Copy, Debug)]
pub struct SwitchStyle {
    /// 未选中轨道色。
    pub track_off: Hsla,
    /// 选中轨道色。
    pub track_on: Hsla,
    /// 未选中手柄色。
    pub handle_off: Hsla,
    /// 选中手柄色。
    pub handle_on: Hsla,
    /// 未选中描边色（`None` 无描边）。
    pub border_off: Option<Hsla>,
    /// 轨道宽/高。
    pub track_size: (Pixels, Pixels),
    /// 选中手柄直径。
    pub thumb_on: Pixels,
    /// 未选中手柄直径。
    pub thumb_off: Pixels,
    /// 图标尺寸。
    pub icon_size: Pixels,
    /// 手柄与轨道边缘的间距（无图标/带图标）。
    pub thumb_margin: (Pixels, Pixels),
    /// 状态层基色。
    pub state_layer_color: Hsla,
    /// 按压档状态层不透明度。
    pub state_layer_opacity: f32,
}

impl SwitchStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet, disabled: bool) -> Self {
        let colors = &tokens.colors;
        let state = &tokens.state_layer;
        let switch = &tokens.component.switch;
        if disabled {
            Self {
                track_off: colors
                    .surface_container_highest
                    .opacity(state.disabled_container),
                track_on: colors.on_surface.opacity(state.disabled_container),
                handle_off: colors.on_surface.opacity(state.disabled_content),
                handle_on: colors.surface,
                border_off: Some(colors.on_surface.opacity(state.disabled_content)),
                track_size: (px(switch.track_width), px(switch.track_height)),
                thumb_on: px(switch.thumb_size),
                thumb_off: px(switch.unselected_thumb_size),
                icon_size: px(switch.icon_size),
                thumb_margin: (px(6.), px(4.)),
                state_layer_color: colors.on_surface,
                state_layer_opacity: state.pressed,
            }
        } else {
            Self {
                track_off: colors.surface_container_highest,
                track_on: colors.primary,
                handle_off: colors.outline,
                handle_on: colors.on_primary,
                border_off: Some(colors.outline),
                track_size: (px(switch.track_width), px(switch.track_height)),
                thumb_on: px(switch.thumb_size),
                thumb_off: px(switch.unselected_thumb_size),
                icon_size: px(switch.icon_size),
                thumb_margin: (px(6.), px(4.)),
                state_layer_color: colors.on_surface,
                state_layer_opacity: state.pressed,
            }
        }
    }
}

/// MD3 Checkbox 样式。
#[derive(Clone, Copy, Debug)]
pub struct CheckboxStyle {
    /// 勾选填充色（error 态为 error 色）。
    pub accent: Hsla,
    /// 勾选图标准色。
    pub on_accent: Hsla,
    /// 未选中边框色。
    pub outline: Hsla,
    /// 方框边长。
    pub box_size: Pixels,
    /// 方框圆角。
    pub corner_radius: Pixels,
    /// 边框宽度。
    pub border_width: Pixels,
    /// 勾图标尺寸。
    pub mark_size: Pixels,
    /// 触摸目标边长。
    pub touch_target: Pixels,
    /// 状态层基色。
    pub state_layer_color: Hsla,
    /// 按压档状态层不透明度。
    pub state_layer_opacity: f32,
    /// 禁用态内容不透明度对应的颜色。
    pub disabled_content: Hsla,
}

impl CheckboxStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet, error: bool, disabled: bool) -> Self {
        let colors = &tokens.colors;
        let state = &tokens.state_layer;
        let (accent, on_accent, outline) = if error {
            (colors.error, colors.on_error, colors.error)
        } else {
            (colors.primary, colors.on_primary, colors.on_surface_variant)
        };
        Self {
            accent,
            on_accent,
            outline,
            box_size: px(18.),
            corner_radius: px(2.),
            border_width: px(2.),
            mark_size: px(16.),
            touch_target: px(40.),
            state_layer_color: accent,
            state_layer_opacity: state.pressed,
            disabled_content: if disabled {
                colors.on_surface.opacity(state.disabled_content)
            } else {
                colors.on_surface
            },
        }
    }
}

/// MD3 RadioButton 样式。
#[derive(Clone, Copy, Debug)]
pub struct RadioStyle {
    /// 未选中外圈色。
    pub ring_off: Hsla,
    /// 选中外圈色。
    pub ring_on: Hsla,
    /// 内点色。
    pub dot: Hsla,
    /// 外圈直径。
    pub ring_size: Pixels,
    /// 边框宽度。
    pub border_width: Pixels,
    /// 内点直径。
    pub dot_size: Pixels,
    /// 触摸目标边长。
    pub touch_target: Pixels,
    /// 状态层基色。
    pub state_layer_color: Hsla,
    /// 按压档状态层不透明度。
    pub state_layer_opacity: f32,
    /// 禁用态内容色。
    pub disabled_content: Hsla,
}

impl RadioStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet, disabled: bool) -> Self {
        let colors = &tokens.colors;
        let state = &tokens.state_layer;
        Self {
            ring_off: if disabled {
                colors.on_surface.opacity(state.disabled_content)
            } else {
                colors.on_surface_variant
            },
            ring_on: if disabled {
                colors.on_surface.opacity(state.disabled_content)
            } else {
                colors.primary
            },
            dot: if disabled {
                colors.on_surface.opacity(state.disabled_content)
            } else {
                colors.primary
            },
            ring_size: px(20.),
            border_width: px(2.),
            dot_size: px(10.),
            touch_target: px(40.),
            state_layer_color: colors.primary,
            state_layer_opacity: state.pressed,
            disabled_content: colors.on_surface.opacity(state.disabled_content),
        }
    }
}

/// MD3 Slider 样式。
#[derive(Clone, Copy, Debug)]
pub struct SliderStyle {
    /// 活动轨道色。
    pub active_track: Hsla,
    /// 非活动轨道色。
    pub inactive_track: Hsla,
    /// 手柄色。
    pub handle: Hsla,
    /// 轨道高度。
    pub track_height: Pixels,
    /// 手柄宽/高。
    pub handle_size: (Pixels, Pixels),
    /// 轨道容器高。
    pub container_height: Pixels,
}

impl SliderStyle {
    /// 由令牌推导默认样式。
    pub fn resolve(tokens: &TokenSet, disabled: bool) -> Self {
        let colors = &tokens.colors;
        let state = &tokens.state_layer;
        let slider = &tokens.component.slider;
        Self {
            active_track: if disabled {
                colors.on_surface.opacity(state.disabled_content)
            } else {
                colors.primary
            },
            inactive_track: if disabled {
                colors.on_surface.opacity(state.disabled_container)
            } else {
                colors.secondary_container
            },
            handle: if disabled {
                colors.on_surface.opacity(state.disabled_content)
            } else {
                colors.primary
            },
            track_height: px(slider.track_height),
            handle_size: (px(slider.handle_width), px(slider.handle_height)),
            container_height: px(44.),
        }
    }
}

// ColorScheme 在 resolve 内经 tokens.colors 使用；显式引用避免文档遗漏
#[allow(dead_code)]
type _ColorSchemeRef = ColorScheme;
