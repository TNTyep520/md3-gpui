//! MD3 Tabs（对应 material-web 的 `md-tabs` / `md-primary-tab`）
//!
//! Primary tabs：高 48dp（带图标 64dp），底部 3dp 圆角指示条。
//! 受控组件：由 `selected` + `on_change` 驱动。

use gpui::{div, prelude::*, px, App, ElementId, IntoElement, RenderOnce, SharedString, Window};
use std::rc::Rc;

use crate::icon::{Icon, IconName};
use crate::theme::{ActiveTheme, HOVER_OPACITY, PRESSED_OPACITY};

type ChangeHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// 单个标签页描述
pub struct Tab {
    pub label: SharedString,
    pub icon: Option<IconName>,
}

impl Tab {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// MD3 标签栏
#[derive(IntoElement)]
pub struct TabBar {
    id: ElementId,
    tabs: Vec<Tab>,
    selected: usize,
    on_change: Option<ChangeHandler>,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            selected: 0,
            on_change: None,
        }
    }

    pub fn tab(mut self, tab: Tab) -> Self {
        self.tabs.push(tab);
        self
    }

    pub fn tabs(mut self, tabs: impl IntoIterator<Item = Tab>) -> Self {
        self.tabs.extend(tabs);
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// 选中标签变化回调，参数为新选中的下标
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for TabBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = &theme.colors;
        let selected = self.selected;
        let on_change = self.on_change;
        let label_style = theme.typography.title_small;
        let has_icons = self.tabs.iter().any(|t| t.icon.is_some());
        let height = if has_icons { px(64.) } else { px(48.) };

        let primary = colors.primary;
        let on_surface = colors.on_surface;
        let on_surface_variant = colors.on_surface_variant;
        let surface = colors.surface;
        let outline_variant = colors.outline_variant;

        div()
            .id(self.id)
            .w_full()
            .flex()
            .bg(surface)
            .border_b_1()
            .border_color(outline_variant)
            .children(self.tabs.into_iter().enumerate().map(|(ix, tab)| {
                let is_selected = ix == selected;
                let fg = if is_selected {
                    primary
                } else {
                    on_surface_variant
                };
                let layer = if is_selected { primary } else { on_surface };
                let on_change = on_change.clone();
                let tab_el = div()
                    .id(("md3-tab", ix))
                    .relative()
                    .flex_1()
                    .h(height)
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(4.))
                    .cursor_pointer()
                    .text_color(fg)
                    .hover(move |s| s.bg(layer.opacity(HOVER_OPACITY)))
                    .active(move |s| s.bg(layer.opacity(PRESSED_OPACITY)))
                    .when_some(on_change, |el, handler| {
                        el.on_click(move |_, window, cx| handler(ix, window, cx))
                    })
                    .when_some(tab.icon, |el, icon| el.child(Icon::new(icon).size(px(24.))));
                let tab_el = label_style.apply(tab_el).child(tab.label);
                // 选中指示条：3dp 高、圆角上边、宽度收窄居中
                tab_el.when(is_selected, |el| {
                    el.child(
                        div()
                            .absolute()
                            .bottom_0()
                            .left_0()
                            .right_0()
                            .flex()
                            .justify_center()
                            .child(
                                div()
                                    .h(px(3.))
                                    .w(px(48.))
                                    .rounded_tl(px(3.))
                                    .rounded_tr(px(3.))
                                    .bg(primary),
                            ),
                    )
                })
            }))
    }
}
