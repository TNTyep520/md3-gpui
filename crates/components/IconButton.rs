//! MD3 IconButton（对应 material-web 的 `md-icon-button` 系列）。
//!
//! 变体：Standard / Filled / FilledTonal / Outlined。
//! 规格：容器 40×40dp、图标 24dp、圆形。支持 toggle（选中态）。
//!
//! 交互行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3IconButtonSkin` / `M3IconToggleButtonSkin`（Apache-2.0，© 2026 Glavo）。
//!
//! ```ignore
//! IconButton::new("fav", IconName::Favorite)
//!     .selected(is_fav)
//!     .on_click(|_, _, _| {})
//!     .build(cx)   // -> Entity<IconButtonState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, ClickEvent, Context, ElementId, Entity, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{AnimatedComponent, AnimationDriver};
use crate::theme::ActiveTheme;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// 图标按钮变体。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonVariant {
    /// 标准无容器图标按钮。
    #[default]
    Standard,
    /// 实心图标按钮。
    Filled,
    /// 次级色调实心图标按钮。
    FilledTonal,
    /// 描边图标按钮。
    Outlined,
}

/// MD3 图标按钮构建器（`.build(cx)` 产出 [`IconButtonState`]）。
pub struct IconButton {
    id: ElementId,
    icon: IconName,
    variant: IconButtonVariant,
    selected: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

/// 图标按钮的有状态部分。
pub struct IconButtonState {
    id: ElementId,
    icon: IconName,
    variant: IconButtonVariant,
    selected: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
    surface: InteractiveSurface,
}

impl IconButton {
    /// 创建图标按钮构建器。
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

    /// 设置变体。
    pub fn variant(mut self, variant: IconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Filled 变体。
    pub fn filled(self) -> Self {
        self.variant(IconButtonVariant::Filled)
    }

    /// FilledTonal 变体。
    pub fn tonal(self) -> Self {
        self.variant(IconButtonVariant::FilledTonal)
    }

    /// Outlined 变体。
    pub fn outlined(self) -> Self {
        self.variant(IconButtonVariant::Outlined)
    }

    /// toggle 选中态。
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
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
    pub fn build(self, cx: &mut App) -> Entity<IconButtonState> {
        cx.new(|_| IconButtonState {
            id: self.id,
            icon: self.icon,
            variant: self.variant,
            selected: self.selected,
            disabled: self.disabled,
            on_click: self.on_click,
            surface: InteractiveSurface::new(),
        })
    }
}

impl IconButtonState {
    /// 组件最近的边界（窗口坐标），可用于菜单等弹层锚定。
    pub fn bounds(&self) -> gpui::Bounds<gpui::Pixels> {
        self.surface.bounds.get()
    }
}

impl AnimatedComponent for IconButtonState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for IconButtonState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let selected = self.selected;

        let style = IconButtonStyle::resolve(theme.token_set(), self.variant, selected);
        let (bg, fg) = (style.container_color, style.content_color);

        let fg = if disabled {
            colors.disabled_content(&state_layer)
        } else {
            fg
        };
        let bg = if disabled {
            bg.map(|_| colors.disabled_container(&state_layer))
        } else {
            bg
        };

        let base = div()
            .id(self.id.clone())
            .size(style.size)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .rounded_full()
            .text_color(fg)
            .when_some(bg, |el, bg_color| el.bg(bg_color))
            .when_some(style.outline_color, |el, outline| {
                el.border_1().border_color(if disabled {
                    colors.on_surface.opacity(state_layer.disabled_container)
                } else {
                    outline
                })
            })
            .when(!disabled, |el| el.cursor_pointer().overflow_hidden());

        let entity = cx.entity();
        let base = if disabled {
            base
        } else {
            crate::interaction::wire(
                &self.surface,
                base,
                &entity,
                theme.motion(),
                |s: &mut Self| &mut s.surface,
                fg,
                state_layer.pressed,
                gpui::px(999.),
            )
        };

        let base = if disabled {
            base
        } else if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        base.child(Icon::new(self.icon).size(style.icon_size))
    }
}

pub use appearance::IconButtonStyle;

mod appearance {
    use super::IconButtonVariant;
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// MD3 图标按钮样式（对应 button.css 的 `.m3-icon-button` 段落）。
    #[derive(Clone, Debug)]
    pub struct IconButtonStyle {
        /// 容器色（`None` 为透明）。
        pub container_color: Option<Hsla>,
        /// 图标色。
        pub content_color: Hsla,
        /// 描边色（`Some` 启用 1dp 描边）。
        pub outline_color: Option<Hsla>,
        /// 容器边长（正方形）。
        pub size: Pixels,
        /// 图标尺寸。
        pub icon_size: Pixels,
        /// 圆角（圆形）。
        pub corner_radius: Pixels,
        /// 状态层/涟漪基色。
        pub state_layer_color: Hsla,
        /// 按压档状态层不透明度。
        pub state_layer_opacity: f32,
        /// 禁用态内容色。
        pub disabled_content_color: Hsla,
    }
    impl IconButtonStyle {
        /// 由令牌推导默认样式。`selected` 为 toggle 选中态。
        pub fn resolve(tokens: &TokenSet, variant: IconButtonVariant, selected: bool) -> Self {
            let colors = &tokens.colors;
            let state = &tokens.state_layer;

            let (container, content, outline) = match (variant, selected) {
                (IconButtonVariant::Standard, false) => (None, colors.on_surface_variant, None),
                (IconButtonVariant::Standard, true) => (None, colors.primary, None),
                (IconButtonVariant::Filled, false) => {
                    (Some(colors.surface_container_highest), colors.primary, None)
                }
                (IconButtonVariant::Filled, true) => {
                    (Some(colors.primary), colors.on_primary, None)
                }
                (IconButtonVariant::FilledTonal, false) => (
                    Some(colors.surface_container_highest),
                    colors.on_surface_variant,
                    None,
                ),
                (IconButtonVariant::FilledTonal, true) => (
                    Some(colors.secondary_container),
                    colors.on_secondary_container,
                    None,
                ),
                (IconButtonVariant::Outlined, false) => {
                    (None, colors.on_surface_variant, Some(colors.outline))
                }
                (IconButtonVariant::Outlined, true) => (
                    Some(colors.inverse_surface),
                    colors.inverse_on_surface,
                    None,
                ),
            };

            Self {
                container_color: container,
                content_color: content,
                outline_color: outline,
                size: px(40.),
                icon_size: px(24.),
                corner_radius: tokens.shapes.full,
                state_layer_color: content,
                state_layer_opacity: state.pressed,
                disabled_content_color: colors.disabled_content(state),
            }
        }
    }
}
