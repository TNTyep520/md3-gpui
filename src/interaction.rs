//! 交互表面核心：状态层淡入淡出与涟漪。
//!
//! 行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3StateLayer` 与 `M3LabeledButtonSkinBase` 的交互生命周期
//! （Apache-2.0，© 2026 Glavo）：
//!
//! - 状态层（hover / pressed）以弹簧动画淡入淡出
//!   （进入用 fastEffects，退出用 defaultEffects）；
//! - 按压时在指针处生成涟漪（半径扩张、释放后淡出），
//!   指针移出组件时取消按压态并淡出涟漪。

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use gpui::{
    AnyElement, Bounds, Canvas, Div, Entity, Hsla, InteractiveElement as _, IntoElement as _,
    MouseButton, ParentElement as _, Pixels, Point, Stateful, StatefulInteractiveElement as _,
    Styled, canvas, div, point, px,
};

use crate::motion::{Animatable, AnimationDriver, MotionRole, MotionScheme, lerp_color};

/// hover 状态层在按压档位上的进度系数（0.08 / 0.10 = 0.8）。
const HOVER_PROGRESS: f64 = 0.8;

/// 组件边界的共享句柄：paint 阶段由捕获元素写入，
/// 事件处理器在下一帧之前读取。
#[derive(Clone, Debug, Default)]
pub struct BoundsHandle(Rc<RefCell<Bounds<Pixels>>>);

impl BoundsHandle {
    /// 创建边界句柄。
    pub fn new() -> Self {
        Self::default()
    }

    /// 读取最近一次捕获的边界（未捕获时为零）。
    pub fn get(&self) -> Bounds<Pixels> {
        *self.0.borrow()
    }

    /// 构造写入本句柄的 canvas 子元素（绝对定位铺满父容器）。
    pub fn capture_element(&self) -> Canvas<()> {
        let handle = self.0.clone();
        canvas(
            move |bounds, _window, _cx| {
                *handle.borrow_mut() = bounds;
            },
            |_, _, _, _| {},
        )
        .absolute()
        .inset_0()
    }
}

/// 状态层 + 涟漪的共享状态。组件把它作为字段内嵌：
/// 事件处理器调用 [`InteractiveSurface::on_press`] 等方法，
/// render 中通过 [`InteractiveSurface::overlay`] 生成覆盖层。
#[derive(Clone, Debug)]
pub struct InteractiveSurface {
    /// 组件边界句柄（paint 阶段捕获）。
    pub bounds: BoundsHandle,
    /// 是否处于 hover。
    pub hovered: bool,
    /// 是否处于按压。
    pub pressed: bool,
    /// 是否启用涟漪（m3fx 中 Switch/Checkbox/Radio 等选择控件
    /// 只有状态层、没有涟漪；默认启用）。
    pub ripple_enabled: bool,
    state_layer_opacity: Animatable,
    ripple: Option<Ripple>,
    driver: AnimationDriver,
}

#[derive(Clone, Copy, Debug)]
struct Ripple {
    /// 涟漪圆心（组件本地坐标）。
    origin: Point<Pixels>,
    /// 涟漪半径（px）。
    radius: Animatable,
    /// 涟漪不透明度系数（1.0 → 0.0，释放后衰减）。
    fade: Animatable,
}

impl Default for InteractiveSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveSurface {
    /// 创建交互表面状态。
    pub fn new() -> Self {
        Self {
            bounds: BoundsHandle::new(),
            hovered: false,
            pressed: false,
            ripple_enabled: true,
            state_layer_opacity: Animatable::new(0.0, 1.0e-3),
            ripple: None,
            driver: AnimationDriver::default(),
        }
    }

    /// 内嵌驱动器（供 [`crate::motion::AnimatedComponent::driver_mut`] 返回）。
    pub fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }

    /// 推进状态层与涟漪动画；返回是否仍在动画中。
    pub fn step(&mut self, now: Instant) -> bool {
        self.state_layer_opacity.tick(now);
        let mut ripple_done = false;
        if let Some(ripple) = &mut self.ripple {
            let radius_running = ripple.radius.tick(now);
            let fade_running = ripple.fade.tick(now);
            if !radius_running && !fade_running && ripple.fade.value() <= 0.0 {
                ripple_done = true;
            }
        }
        if ripple_done {
            self.ripple = None;
        }
        self.is_animating()
    }

    /// 是否仍在动画中（状态层或涟漪）。
    pub fn is_animating(&self) -> bool {
        self.state_layer_opacity.is_running()
            || self.ripple.as_ref().is_some_and(|r| {
                r.radius.is_running() || r.fade.is_running() || r.fade.value() > 0.0
            })
    }

    /// 指针进入/离开。离开时同时取消按压态（对齐 m3fx 的 disarm-on-exit）。
    pub fn set_hovered(&mut self, hovered: bool, motion: &MotionScheme, now: Instant) {
        self.hovered = hovered;
        if !hovered {
            self.pressed = false;
        }
        self.animate_state_layer_to(self.target_opacity(), motion, now);
    }

    /// 指针按下（`position` 为窗口坐标）；在按压点生成涟漪。
    pub fn on_press(&mut self, position: Point<Pixels>, motion: &MotionScheme, now: Instant) {
        self.pressed = true;
        self.state_layer_opacity
            .animate_to(1.0, motion.spec(MotionRole::FastEffects), now);

        if !self.ripple_enabled {
            return;
        }
        // 最大半径取圆心到组件最远角的距离
        let bounds = self.bounds.get();
        let local_x = f32::from(position.x - bounds.origin.x);
        let local_y = f32::from(position.y - bounds.origin.y);
        let width = f32::from(bounds.size.width);
        let height = f32::from(bounds.size.height);
        let dx2 = |x: f32, y: f32| f64::from(x * x + y * y);
        let max_radius = f64::max(
            f64::max(dx2(local_x, local_y), dx2(width - local_x, local_y)),
            f64::max(
                dx2(local_x, height - local_y),
                dx2(width - local_x, height - local_y),
            ),
        )
        .sqrt();
        let mut ripple = Ripple {
            origin: point(px(local_x), px(local_y)),
            radius: Animatable::new(0.0, 1.0e-2),
            fade: Animatable::new(1.0, 1.0e-2),
        };
        ripple
            .radius
            .animate_to(max_radius, motion.spec(MotionRole::FastEffects), now);
        self.ripple = Some(ripple);
    }

    /// 指针在组件内释放。
    pub fn on_release(&mut self, motion: &MotionScheme, now: Instant) {
        self.pressed = false;
        self.animate_state_layer_to(self.target_opacity(), motion, now);
        self.fade_ripple(motion, now);
    }

    /// 指针在组件外释放：取消按压并淡出涟漪。
    pub fn on_cancel(&mut self, motion: &MotionScheme, now: Instant) {
        self.pressed = false;
        self.animate_state_layer_to(self.target_opacity(), motion, now);
        self.fade_ripple(motion, now);
    }

    fn fade_ripple(&mut self, motion: &MotionScheme, now: Instant) {
        if let Some(ripple) = &mut self.ripple {
            ripple
                .fade
                .animate_to(0.0, motion.spec(MotionRole::DefaultEffects), now);
        }
    }

    /// 状态层不透明度目标（相对按压档位的进度）。
    fn target_opacity(&self) -> f64 {
        if self.pressed {
            1.0
        } else if self.hovered {
            HOVER_PROGRESS
        } else {
            0.0
        }
    }

    fn animate_state_layer_to(&mut self, target: f64, motion: &MotionScheme, now: Instant) {
        let role = if target > self.state_layer_opacity.value() {
            MotionRole::FastEffects
        } else {
            MotionRole::DefaultEffects
        };
        self.state_layer_opacity
            .animate_to(target, motion.spec(role), now);
    }

    /// 生成状态层与涟漪的渲染产物。
    ///
    /// `base_color` 为状态层/涟漪基色（通常是组件的 content/on-container
    /// 色）；`pressed_opacity` 为满档（按压）状态层不透明度，取自主题
    /// state layer 令牌（如 0.10）；hover 按 8%/10% 比例映射。
    pub fn overlay(&self, base_color: Hsla, pressed_opacity: f32) -> InteractiveOverlay {
        let layer_alpha = f32::clamp(
            self.state_layer_opacity.value() as f32 * pressed_opacity,
            0.0,
            1.0,
        );
        let ripples: Vec<AnyElement> = self
            .ripple
            .iter()
            .filter(|r| r.fade.value() > 0.0 && r.radius.value() > 0.0)
            .map(|r| {
                let radius = r.radius.value() as f32;
                let alpha = f32::clamp(r.fade.value() as f32 * pressed_opacity, 0.0, 1.0);
                let color = lerp_color(Hsla::transparent_black(), base_color, alpha);
                div()
                    .absolute()
                    .left(r.origin.x - px(radius))
                    .top(r.origin.y - px(radius))
                    .size(px(radius * 2.0))
                    .rounded_full()
                    .bg(color)
                    .into_any_element()
            })
            .collect();
        InteractiveOverlay {
            state_layer_color: if layer_alpha > 0.0 {
                Some(lerp_color(
                    Hsla::transparent_black(),
                    base_color,
                    layer_alpha,
                ))
            } else {
                None
            },
            ripples,
        }
    }
}

/// 状态层与涟漪的渲染产物。
pub struct InteractiveOverlay {
    /// 状态层颜色（透明时为 None）。
    state_layer_color: Option<Hsla>,
    /// 涟漪圆元素。
    ripples: Vec<AnyElement>,
}

impl InteractiveOverlay {
    /// 把覆盖层子元素挂到容器上（容器应为绝对定位锚点并裁剪溢出）。
    pub fn apply(self, container: Stateful<Div>) -> Stateful<Div> {
        let mut container = container;
        if let Some(color) = self.state_layer_color {
            container = container.child(div().absolute().inset_0().bg(color));
        }
        for ripple in self.ripples {
            container = container.child(ripple);
        }
        container
    }
}

/// 把 hover / press / release / cancel 事件接到内嵌
/// [`InteractiveSurface`] 的组件元素上。
///
/// `entity` 为组件自身的 Entity 句柄，`motion` 来自主题
/// （`theme.motion()`）。事件回调只改状态并触发重绘。
pub fn wire_events<T: 'static>(
    el: Stateful<Div>,
    entity: &Entity<T>,
    motion: &MotionScheme,
    access: impl Fn(&mut T) -> &mut InteractiveSurface + 'static + Copy,
) -> Stateful<Div> {
    let hover_entity = entity.clone();
    let press_entity = entity.clone();
    let release_entity = entity.clone();
    let cancel_entity = entity.clone();
    let motion_hover = motion.clone();
    let motion_press = motion.clone();
    let motion_release = motion.clone();
    let motion_cancel = motion.clone();
    el.on_hover(move |hovered, _window, cx| {
        hover_entity.update(cx, |state, cx| {
            let now = Instant::now();
            access(state).set_hovered(*hovered, &motion_hover, now);
            cx.notify();
        });
    })
    .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
        press_entity.update(cx, |state, cx| {
            let now = Instant::now();
            let position = event.position;
            access(state).on_press(position, &motion_press, now);
            cx.notify();
        });
    })
    .on_mouse_up(MouseButton::Left, move |_event, _window, cx| {
        release_entity.update(cx, |state, cx| {
            let now = Instant::now();
            access(state).on_release(&motion_release, now);
            cx.notify();
        });
    })
    .on_mouse_up_out(MouseButton::Left, move |_event, _window, cx| {
        cancel_entity.update(cx, |state, cx| {
            let now = Instant::now();
            access(state).on_cancel(&motion_cancel, now);
            cx.notify();
        });
    })
}
