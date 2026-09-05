//! Chips 页。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use md3_gpui::prelude::*;

use super::subsection;

/// Chips 页视图。
pub struct ChipsPage {
    pub chip_assist: Entity<ChipState>,
    pub chip_filters: Vec<Entity<ChipState>>,
    pub chip_input: Entity<ChipState>,
    pub chip_suggestion: Entity<ChipState>,
}

impl ChipsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let chip_assist = Chip::new("chip-assist", "Assist")
            .assist()
            .leading_icon(IconName::Info)
            .build(cx);
        let chip_filters = ["Alpha", "Beta", "Gamma"]
            .into_iter()
            .enumerate()
            .map(|(ix, label)| Chip::new(("chip-filter", ix), label).filter().build(cx))
            .collect();
        let chip_input = Chip::new("chip-input", "Rust")
            .input()
            .on_remove(|_, window, cx| {
                md3_gpui::overlay::show_snackbar(
                    window,
                    cx,
                    md3_gpui::overlay::Snackbar::new("Chip removed"),
                    None,
                );
            })
            .build(cx);
        let chip_suggestion = Chip::new("chip-suggestion", "Suggestion")
            .suggestion()
            .elevated(true)
            .build(cx);

        cx.new(|_| Self {
            chip_assist,
            chip_filters,
            chip_input,
            chip_suggestion,
        })
    }
}

impl Render for ChipsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        subsection(
            cx,
            "Chips",
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(8.))
                .child(self.chip_assist.clone())
                .children(self.chip_filters.clone())
                .child(self.chip_input.clone())
                .child(self.chip_suggestion.clone()),
        )
    }
}
