use gpui::{App, Entity, IntoElement, Render, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

pub struct ButtonsExtendedPage {
    toggle_a: Entity<ToggleButtonState>,
    toggle_b: Entity<ToggleButtonState>,
    menu: Entity<MenuState>,
}

impl ButtonsExtendedPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let menu = MenuState::new()
            .item(MenuItem::new("Edit").icon(IconName::Edit))
            .item(MenuItem::new("Duplicate").icon(IconName::Add))
            .item(MenuItem::new("Delete").icon(IconName::Delete))
            .build(cx);
        let toggle_a = ToggleButton::new("toggle-bold", "Bold")
            .icon(IconName::Edit)
            .build(cx);
        let toggle_b = ToggleButton::new("toggle-favorite", "Favorite")
            .icon(IconName::Favorite)
            .checked(true)
            .build(cx);
        cx.new(|_| Self {
            toggle_a,
            toggle_b,
            menu,
        })
    }
}

impl Render for ButtonsExtendedPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let disabled = ToggleButton::new("toggle-disabled", "Disabled")
            .disabled(true)
            .build(cx);
        let group = ButtonGroup::new("actions").children([
            Button::new("group-1", "Day").tonal().build(cx),
            Button::new("group-2", "Week").outlined().build(cx),
            Button::new("group-3", "Month").outlined().build(cx),
        ]);
        let vertical = ButtonGroup::new("vertical-actions").vertical().children([
            Button::new("v-1", "Add").filled().build(cx),
            Button::new("v-2", "Remove").outlined().build(cx),
        ]);
        let segmented = SegmentedButtonRow::new("extended-segmented")
            .buttons([
                SegmentedButton::new("Day").selected(true),
                SegmentedButton::new("Week"),
                SegmentedButton::new("Month"),
            ])
            .build(cx);
        let dropdown = ExposedDropdownMenu::new(
            "exposed-menu",
            Button::new("exposed-field", "Choose an action")
                .outlined()
                .build(cx),
        )
        .menu(self.menu.clone())
        .expanded(true);
        gallery([
            showcase_group(
                cx,
                "Toggle buttons",
                [
                    self.toggle_a.clone().into_any_element(),
                    self.toggle_b.clone().into_any_element(),
                    disabled.into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Button group",
                [group.into_any_element(), vertical.into_any_element()],
            ),
            showcase_group(
                cx,
                "Split button",
                [
                    SplitButton::new("split", "Save")
                        .filled()
                        .menu(self.menu.clone())
                        .into_any_element(),
                    dropdown.into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Segmented button",
                [
                    segmented.into_any_element(),
                    div().h(px(1.)).into_any_element(),
                ],
            ),
        ])
    }
}
