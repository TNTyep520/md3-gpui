//! MD3 Button（共享按钮，对应 material-web 的 `md-*-button`）。
//!
//! 五种变体：Filled / Outlined / Text / Elevated / FilledTonal。
//! 规格：高 40dp、胶囊圆角、label-large 字体。
//!
//! 交互行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3ButtonSkin` / `M3LabeledButtonSkinBase`（Apache-2.0，© 2026 Glavo）：
//! 状态层弹簧淡入淡出 + 指针涟漪（按压缩放仅对 Elevated 变体生效，
//! 首期未实现）。
//!
//! ```ignore
//! use md3_gpui::prelude::*;
//!
//! // 在视图的 render 中（cx 为 &mut Context<V>）：
//! Button::new("save", "Save")
//!     .variant(ButtonVariant::Filled)
//!     .leading_icon(IconName::Check)
//!     .on_click(|_, _, _| println!("clicked"))
//!     .build(cx)                      // -> Entity<ButtonState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, ClickEvent, Context, ElementId, Entity, Hsla, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled,
    Window, div, prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::{InteractiveSurface, wire_events};
use crate::motion::{AnimatedComponent, AnimationDriver};
use crate::theme::{ActiveTheme, Elevation};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// 按钮变体
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// 实心主色按钮。
    #[default]
    Filled,
    /// 描边按钮。
    Outlined,
    /// 纯文字按钮。
    Text,
    /// 带高度的按钮。
    Elevated,
    /// 次级色调实心按钮。
    FilledTonal,
}

/// MD3 共享按钮构建器（`.build(cx)` 产出 [`ButtonState`]）。
pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

/// 按钮的有状态部分：交互表面（状态层/涟漪）与动画。
pub struct ButtonState {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    disabled: bool,
    on_click: Option<ClickHandler>,
    surface: InteractiveSurface,
}

impl Button {
    /// 创建按钮构建器。
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

    /// 设置变体。
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Filled 变体。
    pub fn filled(self) -> Self {
        self.variant(ButtonVariant::Filled)
    }

    /// Outlined 变体。
    pub fn outlined(self) -> Self {
        self.variant(ButtonVariant::Outlined)
    }

    /// Text 变体。
    pub fn text(self) -> Self {
        self.variant(ButtonVariant::Text)
    }

    /// Elevated 变体。
    pub fn elevated(self) -> Self {
        self.variant(ButtonVariant::Elevated)
    }

    /// FilledTonal 变体。
    pub fn tonal(self) -> Self {
        self.variant(ButtonVariant::FilledTonal)
    }

    /// 设置前导图标。
    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    /// 设置尾随图标。
    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置点击回调。
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<ButtonState> {
        cx.new(|_| ButtonState {
            id: self.id,
            label: self.label,
            variant: self.variant,
            leading_icon: self.leading_icon,
            trailing_icon: self.trailing_icon,
            disabled: self.disabled,
            on_click: self.on_click,
            surface: InteractiveSurface::new(),
        })
    }
}

impl ButtonState {
    /// 组件最近的边界（窗口坐标），可用于菜单等弹层锚定。
    pub fn bounds(&self) -> gpui::Bounds<gpui::Pixels> {
        self.surface.bounds.get()
    }

    /// 替换点击回调（用于构造后接线）。
    pub fn set_on_click(&mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) {
        self.on_click = Some(Rc::new(handler));
    }
}

impl AnimatedComponent for ButtonState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for ButtonState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 动画循环：仍有动画时调度下一帧
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let button_tokens = theme.component().button;
        let state_layer = *theme.state_layer();

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
            colors.on_surface.opacity(state_layer.disabled_content)
        } else {
            fg
        };
        let bg = if disabled {
            bg.map(|_| colors.on_surface.opacity(state_layer.disabled_container))
        } else {
            bg
        };

        // 状态层/涟漪基色：统一用内容色（实色/透明容器同方案），
        // 等价于 m3fx 把 contentColor 交给 M3StateLayer 作为 state layer paint
        let layer_color = fg;
        let pressed_opacity = state_layer.pressed;

        let has_leading = self.leading_icon.is_some();
        let has_trailing = self.trailing_icon.is_some();
        let is_text = self.variant == ButtonVariant::Text;
        let (pl, pr) = match (is_text, has_leading, has_trailing) {
            (true, _, _) => (
                px(button_tokens.text_horizontal_padding),
                px(button_tokens.text_horizontal_padding),
            ),
            (false, true, false) => (
                px(button_tokens.horizontal_padding_with_icon),
                px(button_tokens.horizontal_padding),
            ),
            (false, false, true) => (
                px(button_tokens.horizontal_padding),
                px(button_tokens.horizontal_padding_with_icon),
            ),
            (false, true, true) => (
                px(button_tokens.horizontal_padding_with_icon),
                px(button_tokens.horizontal_padding_with_icon),
            ),
            (false, false, false) => (
                px(button_tokens.horizontal_padding),
                px(button_tokens.horizontal_padding),
            ),
        };

        let label_style = theme.typography().label_large;
        let shadow_color = colors.shadow;
        let icon_size = px(button_tokens.icon_size);
        let gap = px(button_tokens.icon_gap);

        let base = div()
            .id(self.id.clone())
            .h(px(button_tokens.height))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap(gap)
            .pl(pl)
            .pr(pr)
            .rounded_full()
            .text_color(fg)
            .when(!disabled, |el| el.cursor_pointer().overflow_hidden());
        let base = label_style.apply(base);

        let base = if let Some(bg_color) = bg {
            base.bg(bg_color)
        } else {
            base
        };

        let base = if outlined {
            base.border_1().border_color(if disabled {
                colors.on_surface.opacity(state_layer.disabled_content)
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

        let entity = cx.entity();
        let base = if disabled {
            base
        } else {
            let base = wire_events(base, &entity, theme.motion(), |s: &mut Self| &mut s.surface);
            let overlay = self.surface.overlay(layer_color, pressed_opacity);
            let base = overlay.apply(base);
            base.child(self.surface.bounds.capture_element())
        };

        let base = if disabled {
            base
        } else if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        base.when_some(self.leading_icon, |el, icon| {
            el.child(Icon::new(icon).size(icon_size))
        })
        .child(self.label.clone())
        .when_some(self.trailing_icon, |el, icon| {
            el.child(Icon::new(icon).size(icon_size))
        })
    }
}
