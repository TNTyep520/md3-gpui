//! Cards 页。

use gpui::{App, Entity, IntoElement, Render, Window, div, prelude::*, px};
use md3_gpui::prelude::*;

use super::{demo_card, subsection};

/// Cards 页视图。
pub struct CardsPage;

impl CardsPage {
    pub fn new(_cx: &mut App) -> Entity<Self> {
        _cx.new(|_| Self)
    }
}

impl Render for CardsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        subsection(
            cx,
            "Cards",
            div()
                .flex()
                .flex_wrap()
                .gap(px(12.))
                .child(demo_card(cx, Card::new().elevated(), "Elevated card"))
                .child(demo_card(cx, Card::new().filled(), "Filled card"))
                .child(demo_card(cx, Card::new().outlined(), "Outlined card")),
        )
    }
}
