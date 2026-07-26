//! MD3 IconButton（对应 material-web 的 `md-icon-button` 系列）
//!
//! 变体：Standard / Filled / FilledTonal / Outlined。
//! 规格：容器 40×40dp、图标 24dp、圆形。支持 toggle（选中态）。

use gpui::{div, prelude::*, px, App, ClickEvent, ElementId, IntoElement, RenderOnce, Window};

use crate::icon::{Icon, IconName};
use crate::theme::{
    hover_layer, pressed_layer, ActiveTheme, DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY,
    HOVER_OPACITY, PRESSED_OPACITY,
};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonVariant {
    #[default]
    Standard,
    Filled,
    FilledTonal,
    Outlined,
}

/// MD3 图标按钮
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: IconName,
    variant: IconButtonVariant,
    selected: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            variant: IconButtonVariant::default(),
            selected: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: IconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn filled(self) -> Self {
        self.variant(IconButtonVariant::Filled)
    }

    pub fn tonal(self) -> Self {
        self.variant(IconButtonVariant::FilledTonal)
    }

    pub fn outlined(self) -> Self {
        self.variant(IconButtonVariant::Outlined)
    }

    /// toggle 选中态
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = &cx.theme().colors;
        let disabled = self.disabled;
        let selected = self.selected;

        let (bg, fg, outlined) = match (self.variant, selected) {
            (IconButtonVariant::Standard, false) => (None, colors.on_surface_variant, false),
            (IconButtonVariant::Standard, true) => (None, colors.primary, false),
            (IconButtonVariant::Filled, false) => (
                Some(colors.surface_container_highest),
                colors.primary,
                false,
            ),
            (IconButtonVariant::Filled, true) => (Some(colors.primary), colors.on_primary, false),
            (IconButtonVariant::FilledTonal, false) => (
                Some(colors.surface_container_highest),
                colors.on_surface_variant,
                false,
            ),
            (IconButtonVariant::FilledTonal, true) => (
                Some(colors.secondary_container),
                colors.on_secondary_container,
                false,
            ),
            (IconButtonVariant::Outlined, false) => (None, colors.on_surface_variant, true),
            (IconButtonVariant::Outlined, true) => (
                Some(colors.inverse_surface),
                colors.inverse_on_surface,
                false,
            ),
        };

        let fg = if disabled {
            colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)
        } else {
            fg
        };
        let bg = if disabled {
            bg.map(|_| colors.on_surface.opacity(DISABLED_CONTAINER_OPACITY))
        } else {
            bg
        };

        let (hover_bg, pressed_bg) = match bg {
            Some(base) if !disabled => (hover_layer(base, fg), pressed_layer(base, fg)),
            _ => (fg.opacity(HOVER_OPACITY), fg.opacity(PRESSED_OPACITY)),
        };

        div()
            .id(self.id)
            .size(px(40.))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .rounded_full()
            .text_color(fg)
            .when_some(bg, |el, bg_color| el.bg(bg_color))
            .when(outlined, |el| {
                el.border_1().border_color(if disabled {
                    colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)
                } else {
                    colors.outline
                })
            })
            .when(!disabled, |el| {
                el.cursor_pointer()
                    .hover(move |s| s.bg(hover_bg))
                    .active(move |s| s.bg(pressed_bg))
            })
            .when_some(self.on_click.filter(|_| !disabled), |el, handler| {
                el.on_click(move |event, window, cx| handler(event, window, cx))
            })
            .child(Icon::new(self.icon).size(px(24.)))
    }
}
