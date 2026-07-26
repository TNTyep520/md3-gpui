//! MD3 Radio（对应 material-web 的 `md-radio`）
//!
//! 规格：外圈 20dp、边框 2dp、选中内点 10dp；40dp 圆形触摸目标 + 状态层。

use gpui::{div, prelude::*, px, App, ElementId, IntoElement, RenderOnce, Window};

use crate::theme::{ActiveTheme, DISABLED_CONTENT_OPACITY, HOVER_OPACITY, PRESSED_OPACITY};

type SelectHandler = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// MD3 单选按钮（受控组件）
#[derive(IntoElement)]
pub struct RadioButton {
    id: ElementId,
    selected: bool,
    disabled: bool,
    on_select: Option<SelectHandler>,
}

impl RadioButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selected: false,
            disabled: false,
            on_select: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 被点击（选中）时触发
    pub fn on_select(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for RadioButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = &cx.theme().colors;
        let selected = self.selected;
        let disabled = self.disabled;

        let ring_color = if disabled {
            colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)
        } else if selected {
            colors.primary
        } else {
            colors.on_surface_variant
        };
        let dot_color = if disabled {
            colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)
        } else {
            colors.primary
        };
        let layer = if selected {
            colors.primary
        } else {
            colors.on_surface
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
            .when_some(self.on_select.filter(|_| !disabled), |el, handler| {
                el.on_click(move |_, window, cx| handler(window, cx))
            })
            .child(
                div()
                    .size(px(20.))
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .border_2()
                    .border_color(ring_color)
                    .when(selected, |el| {
                        el.child(div().size(px(10.)).rounded_full().bg(dot_color))
                    }),
            )
    }
}
