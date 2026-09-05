//! Icon buttons & FAB 页。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use md3_gpui::prelude::*;

use super::subsection;

/// Icon buttons & FAB 页视图。
pub struct IconButtonsFabPage {
    pub ib_standard: Entity<IconButtonState>,
    pub ib_filled: Entity<IconButtonState>,
    pub ib_tonal: Entity<IconButtonState>,
    pub ib_outlined: Entity<IconButtonState>,
    pub fab_small: Entity<FabState>,
    pub fab_std: Entity<FabState>,
    pub fab_ext: Entity<FabState>,
}

impl IconButtonsFabPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let ib_standard = IconButton::new("ib-standard", IconName::Favorite).build(cx);
        let ib_filled = IconButton::new("ib-filled", IconName::Edit)
            .filled()
            .build(cx);
        let ib_tonal = IconButton::new("ib-tonal", IconName::Settings)
            .tonal()
            .build(cx);
        let ib_outlined = IconButton::new("ib-outlined", IconName::MoreVert)
            .outlined()
            .build(cx);
        let fab_small = Fab::new("fab-small", IconName::Edit)
            .size(FabSize::Small)
            .build(cx);
        let fab_std = Fab::new("fab-std", IconName::Add).build(cx);
        let fab_ext = Fab::new("fab-ext", IconName::Add)
            .color(FabColor::Tertiary)
            .label("Compose")
            .build(cx);

        cx.new(|_| Self {
            ib_standard,
            ib_filled,
            ib_tonal,
            ib_outlined,
            fab_small,
            fab_std,
            fab_ext,
        })
    }
}

impl Render for IconButtonsFabPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        subsection(
            cx,
            "Icon buttons & FAB",
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.))
                .child(self.ib_standard.clone())
                .child(self.ib_filled.clone())
                .child(self.ib_tonal.clone())
                .child(self.ib_outlined.clone())
                .child(self.fab_small.clone())
                .child(self.fab_std.clone())
                .child(self.fab_ext.clone()),
        )
    }
}
