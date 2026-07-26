//! MD3 Switch（对应 material-web 的 `md-switch`）
//!
//! 规格：轨道 52×32dp 胶囊形；未选中手柄 16dp（outline 色）、
//! 选中手柄 24dp（on-primary 色），可选选中勾图标。

use gpui::{div, prelude::*, px, App, ElementId, IntoElement, RenderOnce, Window};

use crate::icon::{Icon, IconName};
use crate::theme::{
    hover_layer, ActiveTheme, DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY,
};

type ChangeHandler = Box<dyn Fn(bool, &mut Window, &mut App) + 'static>;

/// MD3 开关（受控组件）
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    /// 选中时在手柄内显示勾图标
    show_icon: bool,
    on_change: Option<ChangeHandler>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            show_icon: false,
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

    /// 选中时在手柄中显示勾选图标
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = &cx.theme().colors;
        let checked = self.checked;
        let disabled = self.disabled;

        let (track_bg, track_border, handle_bg, handle_size) = if disabled {
            if checked {
                (
                    colors.on_surface.opacity(DISABLED_CONTAINER_OPACITY),
                    None,
                    colors.surface,
                    px(24.),
                )
            } else {
                (
                    colors
                        .surface_container_highest
                        .opacity(DISABLED_CONTAINER_OPACITY),
                    Some(colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)),
                    colors.on_surface.opacity(DISABLED_CONTENT_OPACITY),
                    px(16.),
                )
            }
        } else if checked {
            (colors.primary, None, colors.on_primary, px(24.))
        } else {
            (
                colors.surface_container_highest,
                Some(colors.outline),
                colors.outline,
                px(16.),
            )
        };

        let hover_track = if checked {
            hover_layer(track_bg, colors.on_primary)
        } else {
            hover_layer(track_bg, colors.on_surface)
        };

        let icon_color = if checked {
            colors.on_primary_container
        } else {
            colors.surface_container_highest
        };
        let show_icon = self.show_icon && checked;

        div()
            .id(self.id)
            .w(px(52.))
            .h(px(32.))
            .flex()
            .flex_none()
            .items_center()
            .rounded_full()
            .bg(track_bg)
            .when_some(track_border, |el, color| el.border_2().border_color(color))
            .when(checked, |el| el.justify_end().pr(px(4.)))
            .when(!checked, |el| el.justify_start().pl(px(6.)))
            .when(!disabled, |el| {
                el.cursor_pointer().hover(move |s| s.bg(hover_track))
            })
            .when_some(self.on_change.filter(|_| !disabled), |el, handler| {
                el.on_click(move |_, window, cx| handler(!checked, window, cx))
            })
            .child(
                div()
                    .size(handle_size)
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(handle_bg)
                    .when(show_icon, |el| {
                        el.child(Icon::new(IconName::Check).size(px(16.)).color(icon_color))
                    }),
            )
    }
}
