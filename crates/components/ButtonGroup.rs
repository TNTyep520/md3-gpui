use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, Window, div, prelude::*, px,
};

/// A horizontal Material button group with a shared touch target.
#[derive(IntoElement)]
pub struct ButtonGroup {
    id: ElementId,
    children: Vec<AnyElement>,
    vertical: bool,
    spacing: gpui::Pixels,
}

impl ButtonGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            vertical: false,
            spacing: px(8.),
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }
    pub fn spacing(mut self, spacing: gpui::Pixels) -> Self {
        self.spacing = spacing;
        self
    }
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl ParentElement for ButtonGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .when(self.vertical, |el| el.flex_col())
            .gap(self.spacing)
            .children(self.children)
    }
}
