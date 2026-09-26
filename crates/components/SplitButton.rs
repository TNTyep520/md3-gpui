use crate::components::{Button, ButtonVariant, IconButton, IconButtonVariant};
use crate::icon::IconName;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    Window, div, prelude::*, px,
};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct SplitButton {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    trailing: IconName,
    on_click: Option<ClickHandler>,
    menu: Option<AnyElement>,
}

impl SplitButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Filled,
            trailing: IconName::ChevronRight,
            on_click: None,
            menu: None,
        }
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn filled(self) -> Self {
        self.variant(ButtonVariant::Filled)
    }
    pub fn outlined(self) -> Self {
        self.variant(ButtonVariant::Outlined)
    }
    pub fn tonal(self) -> Self {
        self.variant(ButtonVariant::FilledTonal)
    }
    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing = icon;
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
    pub fn menu(mut self, menu: impl IntoElement) -> Self {
        self.menu = Some(menu.into_any_element());
        self
    }
}

impl RenderOnce for SplitButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut primary =
            Button::new((self.id.clone(), "primary"), self.label).variant(self.variant);
        if let Some(handler) = self.on_click {
            primary = primary.on_click(handler);
        }
        let primary = primary.build(cx);
        let trailing = IconButton::new((self.id, "menu"), self.trailing)
            .variant(IconButtonVariant::Standard)
            .build(cx);
        div()
            .flex()
            .items_center()
            .gap(px(1.))
            .child(primary)
            .child(trailing)
            .when_some(self.menu, |el, menu| el.child(menu))
    }
}
