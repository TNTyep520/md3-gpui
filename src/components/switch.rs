//! MD3 Switch（对应 material-web 的 `md-switch`）。
//!
//! 规格：轨道 52×32dp 胶囊形；未选中手柄 16dp（outline 色）、
//! 选中手柄 24dp（on-primary 色）。支持双图标样式（对齐 m3fx
//! `M3SwitchSkin`）：选中/未选中各一个图标槽，拇指经过中点时
//! 图标交叉淡化切换；提供图标时拇指恒为 24dp。
//!
//! 交互动画移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3SwitchSkin`（Apache-2.0，© 2026 Glavo）：手柄位置/尺寸与
//! 轨道颜色由弹簧（defaultEffects）驱动；开关只有状态层、无涟漪。
//!
//! ```ignore
//! Switch::new("wifi")
//!     .checked(true)
//!     .on_change(|checked, _, _| println!("wifi: {checked}"))
//!     .build(cx)   // -> Entity<SwitchState>
//!
//! // 带图标样式（选中/未选中各一图标，也可只给选中图标）
//! Switch::new("power").selected_icon(IconName::Check)
//!     .unselected_icon(Some(IconName::Close)).build(cx)
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::{InteractiveSurface, wire_events};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

/// MD3 开关（状态保存在 [`SwitchState`] 内）。
pub struct Switch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    /// 选中态图标槽（对齐 m3fx `selectedIcon`）。
    selected_icon: Option<IconName>,
    /// 未选中态图标槽（对齐 m3fx `unselectedIcon`）。
    unselected_icon: Option<IconName>,
    on_change: Option<ChangeHandler>,
}

/// 开关的有状态部分：手柄位置/颜色动画。
pub struct SwitchState {
    id: ElementId,
    checked: bool,
    disabled: bool,
    selected_icon: Option<IconName>,
    unselected_icon: Option<IconName>,
    on_change: Option<ChangeHandler>,
    /// 0 = 未选中，1 = 选中。
    progress: Animatable,
    surface: InteractiveSurface,
}

impl Switch {
    /// 创建开关构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            selected_icon: None,
            unselected_icon: None,
            on_change: None,
        }
    }

    /// 初始选中态。
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 选中态图标（显示在拇指内）。
    pub fn selected_icon(mut self, icon: IconName) -> Self {
        self.selected_icon = Some(icon);
        self
    }

    /// 未选中态图标（`None` 则未选中时无图标）。
    pub fn unselected_icon(mut self, icon: Option<IconName>) -> Self {
        self.unselected_icon = icon;
        self
    }

    /// 便捷方法：带图标样式（选中态显示勾图标）。
    ///
    /// 等价于 `selected_icon(IconName::Check)`；传 `false` 清除两个图标槽。
    pub fn show_icon(mut self, show: bool) -> Self {
        if show {
            self.selected_icon = Some(IconName::Check);
        } else {
            self.selected_icon = None;
            self.unselected_icon = None;
        }
        self
    }

    /// 设置切换回调（参数为新选中态）。
    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<SwitchState> {
        let checked = self.checked;
        cx.new(|_| SwitchState {
            id: self.id,
            checked,
            disabled: self.disabled,
            selected_icon: self.selected_icon,
            unselected_icon: self.unselected_icon,
            on_change: self.on_change,
            progress: Animatable::new(if checked { 1.0 } else { 0.0 }, 1.0e-3),
            surface: InteractiveSurface::new(),
        })
    }
}

impl SwitchState {
    /// 当前选中态。
    pub fn checked(&self) -> bool {
        self.checked
    }

    /// 替换切换回调（用于构造后接线）。
    pub fn set_on_change(&mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) {
        self.on_change = Some(Rc::new(handler));
    }

    /// 直接设置选中态（带动画）。
    pub fn set_checked(&mut self, checked: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.checked == checked {
            return;
        }
        self.checked = checked;
        let spec = *cx.theme().motion().spec(MotionRole::DefaultEffects);
        self.progress
            .animate_to(if checked { 1.0 } else { 0.0 }, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for SwitchState {
    fn step(&mut self, now: Instant) -> bool {
        let running = self.progress.tick(now);
        let surface_running = self.surface.step(now);
        running || surface_running
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for SwitchState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.progress.is_running() || self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let disabled = self.disabled;
        let style = crate::styles::selection::SwitchStyle::resolve(theme.token_set(), disabled);
        let state_layer = *theme.state_layer();
        let p = self.progress.value() as f32;

        let track_bg = lerp_color(style.track_off, style.track_on, p);
        let handle_bg = lerp_color(style.handle_off, style.handle_on, p);

        // 几何：无图标时手柄 16dp -> 24dp；带图标时恒为 24dp
        //（对齐 m3fx `withIconHandleSize`）
        let (track_w_tok, track_h_tok) = (style.track_size.0, style.track_size.1);
        let has_icons = self.selected_icon.is_some() || self.unselected_icon.is_some();
        let thumb_size = if has_icons {
            style.thumb_on
        } else {
            style.thumb_off + (style.thumb_on - style.thumb_off) * p
        };
        let x_start = if has_icons {
            style.thumb_margin.1
        } else {
            style.thumb_margin.0
        };
        let thumb_x = x_start + (track_w_tok - px(4.) - style.thumb_on - x_start) * p;
        let thumb_y = (track_h_tok - thumb_size) / 2.0;

        // 描边随选中淡出
        let border_alpha = (1.0 - p * 2.0).clamp(0.0, 1.0);
        let border_color = style
            .border_off
            .map(|c| lerp_color(c, gpui::Hsla::transparent_black(), 1.0 - border_alpha));

        // 图标槽：拇指越过中点时切换显示的图标，并在中点两侧交叉淡化
        //（对齐 m3fx `updateDisplayedIcon` / `updateIconOpacity`）
        let same_icons = matches!(
            (self.selected_icon, self.unselected_icon),
            (Some(a), Some(b)) if a == b
        );
        let (displayed_icon, icon_alpha) = if p >= 0.5 {
            (
                self.selected_icon,
                if same_icons {
                    1.0
                } else {
                    (2.0 * p - 1.0).clamp(0.0, 1.0)
                },
            )
        } else {
            (
                self.unselected_icon,
                if same_icons {
                    1.0
                } else {
                    (1.0 - 2.0 * p).clamp(0.0, 1.0)
                },
            )
        };
        let icon_color = if disabled {
            colors.surface_container_highest
        } else if p >= 0.5 {
            colors.on_primary_container
        } else {
            colors.on_surface_variant
        };

        let track_w = style.track_size.0;
        let track_h = style.track_size.1;

        let base = div()
            .id(self.id.clone())
            .w(track_w)
            .h(track_h)
            .relative()
            .flex_none()
            .rounded_full()
            .bg(track_bg)
            // 描边画在独立的圆环层：轨道容器本身不带 border，
            // 避免绝对定位的拇指相对 padding box 定位而整体偏移 2px
            .when_some(border_color, |el, color| {
                el.child(
                    div()
                        .absolute()
                        .inset_0()
                        .rounded_full()
                        .border_2()
                        .border_color(color),
                )
            })
            .when(!disabled, |el| el.cursor_pointer().overflow_hidden());

        let entity = cx.entity();
        let base = if disabled {
            base
        } else {
            let base = wire_events(base, &entity, theme.motion(), |s: &mut Self| &mut s.surface);
            let base = self
                .surface
                .overlay(colors.on_surface, state_layer.pressed)
                .apply(base);
            base.child(self.surface.bounds.capture_element())
        };

        let base = if disabled {
            base
        } else {
            let toggle_entity = entity.clone();
            base.on_click(move |_event, window, cx| {
                toggle_entity.update(cx, |state, cx| {
                    let next = !state.checked;
                    state.set_checked(next, window, cx);
                    if let Some(handler) = state.on_change.clone() {
                        handler(next, window, cx);
                    }
                });
            })
        };

        base.child(
            div()
                .absolute()
                .left(thumb_x)
                .top(thumb_y)
                .size(thumb_size)
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(handle_bg)
                .when_some(displayed_icon.filter(|_| icon_alpha > 0.0), |el, icon| {
                    el.child(
                        Icon::new(icon)
                            .size(style.icon_size)
                            .color(icon_color.opacity(icon_alpha)),
                    )
                }),
        )
    }
}
