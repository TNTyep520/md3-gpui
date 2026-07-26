//! MD3 Checkbox（对应 material-web 的 `md-checkbox`）
//!
//! 规格：复选框 18×18dp、圆角 2dp、边框 2dp；40dp 圆形触摸目标 + 状态层。
//! 受控组件：由 `checked` + `on_change` 驱动。

use gpui::{div, prelude::*, px, App, ElementId, IntoElement, RenderOnce, Window};

use crate::icon::{Icon, IconName};
use crate::theme::{ActiveTheme, DISABLED_CONTENT_OPACITY, HOVER_OPACITY, PRESSED_OPACITY};

type ChangeHandler = Box<dyn Fn(bool, &mut Window, &mut App) + 'static>;

/// MD3 复选框
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    checked: bool,
    disabled: bool,
    error: bool,
    on_change: Option<ChangeHandler>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            error: false,
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 错误状态（使用 error 配色）
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// 勾选状态变化回调，参数为新的 checked 值
    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = &cx.theme().colors;
        let checked = self.checked;
        let disabled = self.disabled;

        let accent = if self.error {
            colors.error
        } else {
            colors.primary
        };
        let on_accent = if self.error {
            colors.on_error
        } else {
            colors.on_primary
        };
        let outline = if self.error {
            colors.error
        } else {
            colors.on_surface_variant
        };

        // 状态层颜色（40dp 圆形触摸目标上的 hover/pressed）
        let layer = if checked { accent } else { colors.on_surface };

        let (box_bg, box_border, mark) = if disabled {
            if checked {
                (
                    Some(colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)),
                    None,
                    Some(colors.surface),
                )
            } else {
                (
                    None,
                    Some(colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)),
                    None,
                )
            }
        } else if checked {
            (Some(accent), None, Some(on_accent))
        } else {
            (None, Some(outline), None)
        };

        div()
            .id(self.id)
            .size(px(40.))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .rounded_full()
            .when(!disabled, |el| {
                el.cursor_pointer()
                    .hover(move |s| s.bg(layer.opacity(HOVER_OPACITY)))
                    .active(move |s| s.bg(layer.opacity(PRESSED_OPACITY)))
            })
            .when_some(self.on_change.filter(|_| !disabled), |el, handler| {
                el.on_click(move |_, window, cx| handler(!checked, window, cx))
            })
            .child(
                div()
                    .size(px(18.))
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .rounded(px(2.))
                    .when_some(box_bg, |el, bg| el.bg(bg))
                    .when_some(box_border, |el, color| el.border_2().border_color(color))
                    .when_some(mark, |el, color| {
                        el.child(Icon::new(IconName::Check).size(px(16.)).color(color))
                    }),
            )
    }
}
