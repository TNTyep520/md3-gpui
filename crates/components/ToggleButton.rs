use crate::icon::{Icon, IconName};
use crate::theme::ActiveTheme;
use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement, Render,
    SharedString, Styled, Window, div, prelude::*, px,
};
use std::rc::Rc;

type ChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

pub struct ToggleButton {
    id: ElementId,
    label: SharedString,
    icon: Option<IconName>,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler>,
}
pub struct ToggleButtonState {
    id: ElementId,
    label: SharedString,
    icon: Option<IconName>,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler>,
}

impl ToggleButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            checked: false,
            disabled: false,
            on_change: None,
        }
    }
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
    pub fn build(self, cx: &mut App) -> Entity<ToggleButtonState> {
        cx.new(|_| ToggleButtonState {
            id: self.id,
            label: self.label,
            icon: self.icon,
            checked: self.checked,
            disabled: self.disabled,
            on_change: self.on_change,
        })
    }
}

impl ToggleButtonState {
    pub fn checked(&self) -> bool {
        self.checked
    }
    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        if self.checked != checked {
            self.checked = checked;
            cx.notify();
        }
    }
}

impl Render for ToggleButtonState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let checked = self.checked;
        let disabled = self.disabled;
        let foreground = if disabled {
            colors.disabled_content(cx.theme().state_layer())
        } else if checked {
            colors.on_secondary_container
        } else {
            colors.on_surface_variant
        };
        let background = if checked {
            Some(colors.secondary_container)
        } else {
            None
        };
        let entity = cx.entity();
        let base = div()
            .id(self.id.clone())
            .h(px(40.))
            .px(px(16.))
            .flex()
            .items_center()
            .justify_center()
            .gap(px(8.))
            .rounded_full()
            .text_color(foreground)
            .when_some(background, |el, c| el.bg(c))
            .when(!disabled, |el| {
                el.cursor_pointer()
                    .hover(|s| s.bg(foreground.opacity(cx.theme().state_layer().hover)))
            });
        let base = if disabled {
            base
        } else {
            base.on_click(move |_, window, cx| {
                let change = entity.update(cx, |state, cx| {
                    state.checked = !state.checked;
                    cx.notify();
                    state
                        .on_change
                        .clone()
                        .map(|handler| (handler, state.checked))
                });
                if let Some((handler, checked)) = change {
                    handler(checked, window, cx);
                }
            })
        };
        base.when_some(self.icon, |el, icon| {
            el.child(Icon::new(icon).size(px(18.)))
        })
        .child(self.label.clone())
    }
}
