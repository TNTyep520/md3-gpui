//! MD3 Tabs（对应 material-web 的 `md-tabs` / `md-primary-tab`）。
//!
//! Primary tabs：高 48dp（带图标 64dp），底部 3dp 圆角指示条。
//!
//! 指示条滑动动画移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3TabBarSkin`（Apache-2.0，© 2026 Glavo）：选中指示条以
//! fastSpatial 弹簧在标签间滑动。
//!
//! ```ignore
//! TabBar::new("tabs")
//!     .tabs([Tab::new("One"), Tab::new("Two")])
//!     .selected(0)
//!     .on_change(|ix, _, _| {})
//!     .build(cx)   // -> Entity<TabBarState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};

use crate::icon::{Icon, IconName};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::theme::{ActiveTheme, HOVER_OPACITY, PRESSED_OPACITY};

type ChangeHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// 单个标签页描述
pub struct Tab {
    /// 标签文本。
    pub label: SharedString,
    /// 可选图标。
    pub icon: Option<IconName>,
}

impl Tab {
    /// 创建标签描述。
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
        }
    }

    /// 设置图标。
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// MD3 标签栏构建器（`.build(cx)` 产出 [`TabBarState`]）。
pub struct TabBar {
    id: ElementId,
    tabs: Vec<Tab>,
    selected: usize,
    on_change: Option<ChangeHandler>,
}

/// 标签栏的有状态部分：指示条滑动动画。
pub struct TabBarState {
    id: ElementId,
    tabs: Vec<Tab>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    /// 指示条位置（以标签下标为单位，弹簧驱动）。
    indicator: Animatable,
    driver: AnimationDriver,
}

impl TabBar {
    /// 创建标签栏构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            selected: 0,
            on_change: None,
        }
    }

    /// 追加一个标签。
    pub fn tab(mut self, tab: Tab) -> Self {
        self.tabs.push(tab);
        self
    }

    /// 批量追加标签。
    pub fn tabs(mut self, tabs: impl IntoIterator<Item = Tab>) -> Self {
        self.tabs.extend(tabs);
        self
    }

    /// 初始选中下标。
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// 选中标签变化回调，参数为新选中的下标。
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<TabBarState> {
        let selected = self.selected.min(self.tabs.len().saturating_sub(1));
        cx.new(|_| TabBarState {
            id: self.id,
            tabs: self.tabs,
            selected,
            on_change: self.on_change,
            indicator: Animatable::new(selected as f64, 1.0e-3),
            driver: AnimationDriver::default(),
        })
    }
}

impl TabBarState {
    /// 当前选中下标。
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// 切换到指定标签（指示条弹簧滑动）。
    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.tabs.len() || index == self.selected {
            return;
        }
        self.selected = index;
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.indicator
            .animate_to(index as f64, &spec, Instant::now());
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for TabBarState {
    fn step(&mut self, now: Instant) -> bool {
        self.indicator.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for TabBarState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let selected = self.selected;
        let on_change = self.on_change.clone();
        let entity = cx.entity();
        let style = TabBarStyle::resolve(
            theme.token_set(),
            self.tabs.iter().any(|tab| tab.icon.is_some()),
        );
        let label_style = style.label;
        let has_icons = self.tabs.iter().any(|t| t.icon.is_some());
        let height = style.bar_height(has_icons);
        let indicator_pos = self.indicator.value() as f32;

        let primary = style.selected_item_color;
        let on_surface_variant = style.unselected_item_color;
        let surface = style.container_color;
        let outline_variant = style.divider_color;

        div()
            .id(self.id.clone())
            .w_full()
            .flex()
            .overflow_hidden()
            .bg(surface)
            .border_b_1()
            .border_color(outline_variant)
            .children(self.tabs.iter().enumerate().map(|(ix, tab)| {
                let is_selected = ix == selected;
                let fg = if is_selected {
                    primary
                } else {
                    on_surface_variant
                };
                let layer = if is_selected {
                    primary
                } else {
                    colors.on_surface
                };
                let on_change = on_change.clone();
                let click_entity = entity.clone();
                let tab_el = div()
                    .id((SharedString::from(format!("{}-tab", self.id)), ix))
                    .relative()
                    .flex_1()
                    .h(height)
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(style.gap)
                    .cursor_pointer()
                    .text_color(fg)
                    .hover(move |s| s.bg(layer.opacity(HOVER_OPACITY)))
                    .active(move |s| s.bg(layer.opacity(PRESSED_OPACITY)))
                    // 点击:组件内部先完成选中(弹簧滑动),
                    // 状态真正变化才触发一次 on_change
                    .on_click(move |_, window, cx| {
                        click_entity.update(cx, |state, cx| {
                            let changed = ix != state.selected;
                            state.select(ix, window, cx);
                            if changed && let Some(handler) = on_change.clone() {
                                handler(ix, window, cx);
                            }
                        });
                    })
                    .when_some(tab.icon, |el, icon| {
                        el.child(Icon::new(icon).size(style.icon_size))
                    });
                let tab_el = label_style.apply(tab_el).child(tab.label.clone());
                // 选中指示条：3dp 高、圆角上边、宽度收窄。
                // 画在选中标签内部并以弹簧位置做相对偏移
                //（一个标签宽度 = 1.0 个 relative 单位，0.5 为标签中心，
                // 滑动时随偏移跨标签平移，由容器 overflow_hidden 裁剪）。
                tab_el.when(is_selected, |el| {
                    el.child(
                        div()
                            .absolute()
                            .bottom_0()
                            .left(relative(0.5 + indicator_pos - ix as f32))
                            .w(px(48.))
                            .ml(px(-24.))
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

pub use appearance::TabBarStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// MD3 标签栏样式。
    #[derive(Clone, Copy, Debug)]
    pub struct TabBarStyle {
        /// 栏背景色。
        pub container_color: Hsla,
        /// 底部分隔线色。
        pub divider_color: Hsla,
        /// 选中项内容色。
        pub selected_item_color: Hsla,
        /// 未选中项内容色。
        pub unselected_item_color: Hsla,
        /// 指示条颜色。
        pub indicator_color: Hsla,
        /// 指示条高/宽。
        pub indicator_size: (Pixels, Pixels),
        /// 无图标时栏高。
        pub height: Pixels,
        /// 带图标时栏高。
        pub height_with_icon: Pixels,
        /// 图标尺寸。
        pub icon_size: Pixels,
        /// 图标与文字间距。
        pub gap: Pixels,
        /// hover 状态层不透明度。
        pub hover_opacity: f32,
        /// 按压状态层不透明度。
        pub pressed_opacity: f32,
        /// 标签字型。
        pub label: crate::theme::TypeStyle,
    }
    impl TabBarStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet, _has_icons: bool) -> Self {
            let colors = &tokens.colors;
            Self {
                container_color: colors.surface,
                divider_color: colors.outline_variant,
                selected_item_color: colors.primary,
                unselected_item_color: colors.on_surface_variant,
                indicator_color: colors.primary,
                indicator_size: (px(3.), px(48.)),
                height: px(48.),
                height_with_icon: px(64.),
                icon_size: px(24.),
                gap: px(4.),
                hover_opacity: crate::theme::HOVER_OPACITY,
                pressed_opacity: crate::theme::PRESSED_OPACITY,
                label: tokens.typography.label_large,
            }
        }
    }
    impl TabBarStyle {
        /// 当前使用的栏高。
        pub fn bar_height(&self, has_icons: bool) -> Pixels {
            if has_icons {
                self.height_with_icon
            } else {
                self.height
            }
        }
    }
}
