//! MD3 Chips（对应 material-web 的 `md-*-chip`）
//!
//! 变体：Assist / Filter / Input / Suggestion。
//! 规格：高 32dp、圆角 8dp、label-large 字体。
//! Filter chip 选中时显示勾图标 + secondary-container 底色；
//! Input chip 可带尾部移除按钮。

use gpui::{
    div, prelude::*, px, App, ClickEvent, ElementId, IntoElement, RenderOnce, SharedString, Window,
};

use crate::icon::{Icon, IconName};
use crate::theme::{
    hover_layer, pressed_layer, ActiveTheme, DISABLED_CONTENT_OPACITY, HOVER_OPACITY,
    PRESSED_OPACITY,
};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChipVariant {
    #[default]
    Assist,
    Filter,
    Input,
    Suggestion,
}

/// MD3 纸片组件
#[derive(IntoElement)]
pub struct Chip {
    id: ElementId,
    label: SharedString,
    variant: ChipVariant,
    selected: bool,
    disabled: bool,
    elevated: bool,
    leading_icon: Option<IconName>,
    on_click: Option<ClickHandler>,
    on_remove: Option<ClickHandler>,
}

impl Chip {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ChipVariant::default(),
            selected: false,
            disabled: false,
            elevated: false,
            leading_icon: None,
            on_click: None,
            on_remove: None,
        }
    }

    pub fn variant(mut self, variant: ChipVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn assist(self) -> Self {
        self.variant(ChipVariant::Assist)
    }

    pub fn filter(self) -> Self {
        self.variant(ChipVariant::Filter)
    }

    pub fn input(self) -> Self {
        self.variant(ChipVariant::Input)
    }

    pub fn suggestion(self) -> Self {
        self.variant(ChipVariant::Suggestion)
    }

    /// 选中态（主要用于 Filter chip）
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 悬浮样式（elevated，无描边 + 阴影）
    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Input chip 的尾部移除按钮回调（设置后显示 × 按钮）
    pub fn on_remove(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_remove = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Chip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = &theme.colors;
        let disabled = self.disabled;
        let selected = self.selected;

        let fg = if disabled {
            colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)
        } else if selected {
            colors.on_secondary_container
        } else {
            colors.on_surface
        };
        let icon_color = if disabled {
            fg
        } else if selected {
            colors.on_secondary_container
        } else {
            colors.primary
        };

        let bg = if selected {
            Some(colors.secondary_container)
        } else if self.elevated {
            Some(colors.surface_container_low)
        } else {
            None
        };

        let (hover_bg, pressed_bg) = match bg {
            Some(base) if !disabled => (hover_layer(base, fg), pressed_layer(base, fg)),
            _ => (fg.opacity(HOVER_OPACITY), fg.opacity(PRESSED_OPACITY)),
        };

        // Filter chip 选中时自动带勾图标
        let leading = if self.variant == ChipVariant::Filter && selected {
            Some(IconName::Check)
        } else {
            self.leading_icon
        };

        let has_leading = leading.is_some();
        let has_trailing = self.on_remove.is_some();
        let label_style = theme.typography.label_large;
        let outline_color = if disabled {
            colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)
        } else {
            colors.outline_variant
        };
        let show_outline = bg.is_none() && !self.elevated;

        let base = div()
            .id(self.id)
            .h(px(32.))
            .flex()
            .flex_none()
            .items_center()
            .gap(px(8.))
            .rounded(theme.shapes.small)
            .pl(if has_leading { px(8.) } else { px(16.) })
            .pr(if has_trailing { px(8.) } else { px(16.) })
            .text_color(fg);
        let base = label_style.apply(base);

        base.when_some(bg, |el, bg_color| el.bg(bg_color))
            .when(show_outline, |el| el.border_1().border_color(outline_color))
            .when(!disabled, |el| {
                el.cursor_pointer()
                    .hover(move |s| s.bg(hover_bg))
                    .active(move |s| s.bg(pressed_bg))
            })
            .when_some(self.on_click.filter(|_| !disabled), |el, handler| {
                el.on_click(move |event, window, cx| handler(event, window, cx))
            })
            .when_some(leading, |el, icon| {
                el.child(Icon::new(icon).size(px(18.)).color(icon_color))
            })
            .child(self.label)
            .when_some(self.on_remove.filter(|_| !disabled), |el, handler| {
                el.child(
                    div()
                        .id("chip-remove")
                        .size(px(18.))
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .cursor_pointer()
                        .hover(move |s| s.bg(fg.opacity(HOVER_OPACITY)))
                        .on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            handler(event, window, cx)
                        })
                        .child(Icon::new(IconName::Close).size(px(16.)).color(fg)),
                )
            })
    }
}
