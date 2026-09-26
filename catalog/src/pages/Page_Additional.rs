use super::{gallery, showcase_group};
use gpui::{App, IntoElement, Render, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

pub struct AdditionalPage;
impl AdditionalPage {
    pub fn new(cx: &mut App) -> gpui::Entity<Self> {
        cx.new(|_| Self)
    }
}
impl Render for AdditionalPage {
    fn render(&mut self, _: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let toolbar = FloatingToolbar::new("toolbar").children([
            IconButton::new("tool-add", IconName::Add).build(cx),
            IconButton::new("tool-edit", IconName::Edit).build(cx),
        ]);
        let rail = WideNavigationRail::new("wide-rail").children([
            Button::new("rail-home", "Home").text().build(cx),
            Button::new("rail-settings", "Settings").text().build(cx),
        ]);
        let fab_menu = FabMenu::new("fab-menu")
            .expanded(true)
            .action(Fab::new("fab-add", IconName::Add).build(cx));
        gallery([
            showcase_group(
                cx,
                "Loading & progress",
                [
                    LoadingIndicator::new("loading")
                        .size(px(40.))
                        .into_any_element(),
                    WavyProgressIndicator::new("wavy")
                        .value(0.64)
                        .into_any_element(),
                    RangeSlider::new("range", 0.2, 0.78).into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Search & secure fields",
                [
                    SearchBar::new("search")
                        .query("Search components")
                        .into_any_element(),
                    SecureTextField::new("secure", "Password")
                        .value("secret")
                        .into_any_element(),
                    DatePicker::new("date")
                        .value("2026-09-27")
                        .into_any_element(),
                    TimePicker::new("time").value("10:30").into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Toolbar & navigation",
                [
                    toolbar.into_any_element(),
                    rail.into_any_element(),
                    SwipeToDismissBox::new("dismiss", Card::new().child("Swipe content"))
                        .background(div().bg(cx.theme().colors().error_container))
                        .into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "FAB menu & scrollbar",
                [
                    fab_menu.into_any_element(),
                    div()
                        .relative()
                        .h(px(72.))
                        .w_full()
                        .child("Scrollable content")
                        .child(Scrollbar::new("scrollbar").position(0.3))
                        .into_any_element(),
                ],
            ),
        ])
    }
}
