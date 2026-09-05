//! Selection controls 页：Checkbox / Radio / Switch。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use md3_gpui::prelude::*;

use super::subsection;

/// Selection controls 页视图。
pub struct SelectionPage {
    pub cb_a: Entity<CheckboxState>,
    pub cb_b: Entity<CheckboxState>,
    pub cb_disabled: Entity<CheckboxState>,
    pub radios: Vec<Entity<RadioState>>,
    pub sw_dual_icon: Entity<SwitchState>,
    pub sw_selected_icon: Entity<SwitchState>,
}

impl SelectionPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let cb_a = Checkbox::new("cb-a").checked(true).build(cx);
        let cb_b = Checkbox::new("cb-b").build(cx);
        let cb_disabled = Checkbox::new("cb-dis")
            .checked(true)
            .disabled(true)
            .build(cx);
        // 第三个为禁用态演示
        let radios = (0usize..3)
            .map(|ix| {
                RadioButton::new(("radio", ix))
                    .selected(ix == 0)
                    .disabled(ix == 2)
                    .build(cx)
            })
            .collect();
        let sw_dual_icon = Switch::new("sw-dual")
            .selected_icon(IconName::Check)
            .unselected_icon(Some(IconName::Close))
            .checked(true)
            .build(cx);
        let sw_selected_icon = Switch::new("sw-selected-icon")
            .selected_icon(IconName::Favorite)
            .build(cx);

        cx.new(|_| Self {
            cb_a,
            cb_b,
            cb_disabled,
            radios,
            sw_dual_icon,
            sw_selected_icon,
        })
    }
}

impl Render for SelectionPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        subsection(
            cx,
            "Checkbox / Radio / Switch",
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(16.))
                .child(self.cb_a.clone())
                .child(self.cb_b.clone())
                .child(self.cb_disabled.clone())
                .child(div().w(px(8.)))
                .children(self.radios.clone())
                .child(div().w(px(8.)))
                .child(self.sw_dual_icon.clone())
                .child(self.sw_selected_icon.clone()),
        )
    }
}
