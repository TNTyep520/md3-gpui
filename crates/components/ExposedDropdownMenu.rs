use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, Window, div, prelude::*, px,
};

#[derive(IntoElement)]
pub struct ExposedDropdownMenu {
    id: ElementId,
    field: AnyElement,
    menu: Option<AnyElement>,
    expanded: bool,
}

impl ExposedDropdownMenu {
    pub fn new(id: impl Into<ElementId>, field: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            field: field.into_any_element(),
            menu: None,
            expanded: false,
        }
    }
    pub fn menu(mut self, menu: impl IntoElement) -> Self {
        self.menu = Some(menu.into_any_element());
        self
    }
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }
}

impl RenderOnce for ExposedDropdownMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let expanded = self.expanded;
        div()
            .id(self.id)
            .relative()
            .w_full()
            .when(expanded, |el| el.pb(px(220.)))
            .child(self.field)
            .when(expanded, |el| {
                el.when_some(self.menu, |el, menu| {
                    el.child(div().absolute().top(px(48.)).left_0().right_0().child(menu))
                })
            })
    }
}
