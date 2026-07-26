//! MD3 FAB（对应 material-web 的 `md-fab`）
//!
//! 尺寸：Small 40dp / Standard 56dp / Large 96dp；
//! 颜色：Surface / Primary / Secondary / Tertiary；
//! 支持 Extended FAB（带文字标签）。

use gpui::{
    div, prelude::*, px, App, ClickEvent, ElementId, IntoElement, RenderOnce, SharedString, Window,
};

use crate::icon::{Icon, IconName};
use crate::theme::{hover_layer, pressed_layer, ActiveTheme, Elevation};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabSize {
    Small,
    #[default]
    Standard,
    Large,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabColor {
    Surface,
    #[default]
    Primary,
    Secondary,
    Tertiary,
}

/// MD3 悬浮操作按钮
#[derive(IntoElement)]
pub struct Fab {
    id: ElementId,
    icon: IconName,
    size: FabSize,
    color: FabColor,
    label: Option<SharedString>,
    lowered: bool,
    on_click: Option<ClickHandler>,
}

impl Fab {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            size: FabSize::default(),
            color: FabColor::default(),
            label: None,
            lowered: false,
            on_click: None,
        }
    }

    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: FabColor) -> Self {
        self.color = color;
        self
    }

    /// Extended FAB：附带文字标签（固定高 56dp）
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// 降低的 elevation（Level1 而非 Level3）
    pub fn lowered(mut self, lowered: bool) -> Self {
        self.lowered = lowered;
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

impl RenderOnce for Fab {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = &theme.colors;

        let (bg, fg) = match self.color {
            FabColor::Surface => (colors.surface_container_high, colors.primary),
            FabColor::Primary => (colors.primary_container, colors.on_primary_container),
            FabColor::Secondary => (colors.secondary_container, colors.on_secondary_container),
            FabColor::Tertiary => (colors.tertiary_container, colors.on_tertiary_container),
        };

        let extended = self.label.is_some();
        let (container, radius, icon_size) = if extended {
            (px(56.), theme.shapes.large, px(24.))
        } else {
            match self.size {
                FabSize::Small => (px(40.), theme.shapes.medium, px(24.)),
                FabSize::Standard => (px(56.), theme.shapes.large, px(24.)),
                FabSize::Large => (px(96.), theme.shapes.extra_large, px(36.)),
            }
        };

        let elevation = if self.lowered {
            Elevation::Level1
        } else {
            Elevation::Level3
        };

        let hover_bg = hover_layer(bg, fg);
        let pressed_bg = pressed_layer(bg, fg);
        let label_style = theme.typography.label_large;
        let shadow_color = colors.shadow;

        let base = div()
            .id(self.id)
            .h(container)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap(px(8.))
            .rounded(radius)
            .bg(bg)
            .text_color(fg)
            .shadow(elevation.shadows(shadow_color))
            .cursor_pointer()
            .hover(move |s| s.bg(hover_bg))
            .active(move |s| s.bg(pressed_bg));

        let base = if extended {
            label_style.apply(base.pl(px(16.)).pr(px(20.)))
        } else {
            base.w(container)
        };

        base.when_some(self.on_click, |el, handler| {
            el.on_click(move |event, window, cx| handler(event, window, cx))
        })
        .child(Icon::new(self.icon).size(icon_size))
        .when_some(self.label, |el, label| el.child(label))
    }
}
