//! MD3 Switch（对应 material-web 的 `md-switch`）。
//!
//! 规格：轨道 52×32dp 胶囊形；未选中手柄 16dp（outline 色）、
//! 选中手柄 24dp（on-primary 色），可选选中勾图标。
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
    /// 选中时在手柄内显示勾图标
    show_icon: bool,
    on_change: Option<ChangeHandler>,
}

/// 开关的有状态部分：手柄位置/颜色动画。
pub struct SwitchState {
    id: ElementId,
    checked: bool,
    disabled: bool,
    show_icon: bool,
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
            show_icon: false,
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

    /// 选中时在手柄中显示勾选图标。
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
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
            show_icon: self.show_icon,
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
        let tokens = theme.component().switch;
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let p = self.progress.value() as f32;

        // 端点颜色随 disabled 变化，进度驱动插值
        let (track_a, track_b, handle_a, handle_b, border_color) = if disabled {
            (
                colors
                    .surface_container_highest
                    .opacity(state_layer.disabled_container),
                colors.on_surface.opacity(state_layer.disabled_container),
                colors.on_surface.opacity(state_layer.disabled_content),
                colors.surface,
                Some(colors.on_surface.opacity(state_layer.disabled_content)),
            )
        } else {
            (
                colors.surface_container_highest,
                colors.primary,
                colors.outline,
                colors.on_primary,
                Some(colors.outline),
            )
        };
        let track_bg = lerp_color(track_a, track_b, p);
        let handle_bg = lerp_color(handle_a, handle_b, p);

        // 几何：手柄从 (6, 16dp) 移动到 (24, 24dp)
        let thumb_size = px(tokens.unselected_thumb_size)
            + (px(tokens.thumb_size) - px(tokens.unselected_thumb_size)) * p;
        let thumb_x = px(6.) + (px(tokens.track_width - 4. - tokens.thumb_size) - px(6.)) * p;
        let thumb_y = (px(tokens.track_height) - thumb_size) / 2.0;

        // 描边随选中淡出
        let border_alpha = (1.0 - p * 2.0).clamp(0.0, 1.0);
        let border_color = border_color
            .map(|c| lerp_color(c, gpui::Hsla::transparent_black(), 1.0 - border_alpha));

        // 勾图标透明度
        let icon_alpha = if self.show_icon {
            ((p - 0.5) * 2.0).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let icon_color = if disabled {
            colors.surface_container_highest
        } else {
            colors.on_primary_container
        };

        let track_w = px(tokens.track_width);
        let track_h = px(tokens.track_height);

        let base = div()
            .id(self.id.clone())
            .w(track_w)
            .h(track_h)
            .relative()
            .flex_none()
            .rounded_full()
            .bg(track_bg)
            .when_some(border_color, |el, color| el.border_2().border_color(color))
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
                .when(icon_alpha > 0.0, |el| {
                    el.child(
                        Icon::new(IconName::Check)
                            .size(px(tokens.icon_size))
                            .color(icon_color.opacity(icon_alpha)),
                    )
                }),
        )
    }
}
