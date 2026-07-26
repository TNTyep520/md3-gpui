//! MD3 Button（共享按钮，对应 material-web 的 `md-*-button`）
//!
//! 五种变体：Filled / Outlined / Text / Elevated / FilledTonal。
//! 规格：高 40dp、胶囊圆角、label-large 字体、水平内边距 24dp（Text 为 12dp）。
//!
//! ```ignore
//! Button::new("save", "Save")
//!     .variant(ButtonVariant::Filled)
//!     .leading_icon(IconName::Check)
//!     .on_click(|_, _, _| println!("clicked"))
//! ```

use gpui::{
    div, prelude::*, px, App, ClickEvent, ElementId, Hsla, IntoElement, RenderOnce, SharedString,
    Window,
};

use crate::icon::{Icon, IconName};
use crate::theme::{
    hover_layer, pressed_layer, ActiveTheme, Elevation, DISABLED_CONTAINER_OPACITY,
    DISABLED_CONTENT_OPACITY,
};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// 按钮变体
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Filled,
    Outlined,
    Text,
    Elevated,
    FilledTonal,
}

/// MD3 共享按钮
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::default(),
            leading_icon: None,
            trailing_icon: None,
            disabled: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn filled(self) -> Self {
        self.variant(ButtonVariant::Filled)
    }

    pub fn outlined(self) -> Self {
        self.variant(ButtonVariant::Outlined)
    }

    pub fn text(self) -> Self {
        self.variant(ButtonVariant::Text)
    }

    pub fn elevated(self) -> Self {
        self.variant(ButtonVariant::Elevated)
    }

    pub fn tonal(self) -> Self {
        self.variant(ButtonVariant::FilledTonal)
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
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

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = &theme.colors;

        // (容器色, 内容色, 是否描边, elevation)
        let (bg, fg, outlined, elevation): (Option<Hsla>, Hsla, bool, Elevation) =
            match self.variant {
                ButtonVariant::Filled => (
                    Some(colors.primary),
                    colors.on_primary,
                    false,
                    Elevation::Level0,
                ),
                ButtonVariant::FilledTonal => (
                    Some(colors.secondary_container),
                    colors.on_secondary_container,
                    false,
                    Elevation::Level0,
                ),
                ButtonVariant::Elevated => (
                    Some(colors.surface_container_low),
                    colors.primary,
                    false,
                    Elevation::Level1,
                ),
                ButtonVariant::Outlined => (None, colors.primary, true, Elevation::Level0),
                ButtonVariant::Text => (None, colors.primary, false, Elevation::Level0),
            };

        let disabled = self.disabled;
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

        // state layer：实色容器上做混色；透明容器上用带透明度的内容色
        let (hover_bg, pressed_bg) = match bg {
            Some(base) if !disabled => (hover_layer(base, fg), pressed_layer(base, fg)),
            _ => (
                fg.opacity(crate::theme::HOVER_OPACITY),
                fg.opacity(crate::theme::PRESSED_OPACITY),
            ),
        };

        let has_leading = self.leading_icon.is_some();
        let has_trailing = self.trailing_icon.is_some();
        let is_text = self.variant == ButtonVariant::Text;
        let (pl, pr) = match (is_text, has_leading, has_trailing) {
            (true, _, _) => (px(12.), px(12.)),
            (false, true, false) => (px(16.), px(24.)),
            (false, false, true) => (px(24.), px(16.)),
            (false, true, true) => (px(16.), px(16.)),
            (false, false, false) => (px(24.), px(24.)),
        };

        let label_style = theme.typography.label_large;
        let shadow_color = colors.shadow;

        let base = div()
            .id(self.id)
            .h(px(40.))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap(px(8.))
            .pl(pl)
            .pr(pr)
            .rounded_full()
            .text_color(fg);
        let base = label_style.apply(base);

        let base = if let Some(bg_color) = bg {
            base.bg(bg_color)
        } else {
            base
        };

        let base = if outlined {
            base.border_1().border_color(if disabled {
                colors.on_surface.opacity(DISABLED_CONTENT_OPACITY)
            } else {
                colors.outline
            })
        } else {
            base
        };

        let base = if elevation != Elevation::Level0 && !disabled {
            base.shadow(elevation.shadows(shadow_color))
        } else {
            base
        };

        base.when(!disabled, |el| {
            el.cursor_pointer()
                .hover(move |s| s.bg(hover_bg))
                .active(move |s| s.bg(pressed_bg))
        })
        .when_some(self.on_click.filter(|_| !disabled), |el, handler| {
            el.on_click(move |event, window, cx| handler(event, window, cx))
        })
        .when_some(self.leading_icon, |el, icon| {
            el.child(Icon::new(icon).size(px(18.)))
        })
        .child(self.label)
        .when_some(self.trailing_icon, |el, icon| {
            el.child(Icon::new(icon).size(px(18.)))
        })
    }
}
