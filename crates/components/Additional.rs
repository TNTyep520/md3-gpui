use crate::theme::ActiveTheme;
use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, Window, div, prelude::*, px, relative,
};
use std::time::Duration;

#[derive(IntoElement)]
pub struct LoadingIndicator {
    id: ElementId,
    size: gpui::Pixels,
}
impl LoadingIndicator {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: px(48.),
        }
    }
    pub fn size(mut self, size: gpui::Pixels) -> Self {
        self.size = size;
        self
    }
}
impl RenderOnce for LoadingIndicator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        div()
            .id(self.id)
            .size(self.size)
            .rounded_full()
            .border_3()
            .border_color(c.primary)
            .text_color(c.primary)
            .with_animation(
                "md3-loading",
                Animation::new(Duration::from_millis(1200)).repeat(),
                |el, _| el,
            )
    }
}

#[derive(IntoElement)]
pub struct WavyProgressIndicator {
    id: ElementId,
    value: Option<f32>,
}
impl WavyProgressIndicator {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
        }
    }
    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(value.clamp(0., 1.));
        self
    }
}
impl RenderOnce for WavyProgressIndicator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        let v = self.value.unwrap_or(0.35);
        div()
            .id(self.id)
            .h(px(6.))
            .w_full()
            .rounded_full()
            .bg(c.secondary_container)
            .child(div().h_full().w(relative(v)).rounded_full().bg(c.primary))
    }
}

#[derive(IntoElement)]
pub struct RangeSlider {
    id: ElementId,
    start: f32,
    end: f32,
}
impl RangeSlider {
    pub fn new(id: impl Into<ElementId>, start: f32, end: f32) -> Self {
        Self {
            id: id.into(),
            start: start.clamp(0., 1.),
            end: end.clamp(0., 1.),
        }
    }
    pub fn range(mut self, start: f32, end: f32) -> Self {
        self.start = start.clamp(0., 1.);
        self.end = end.clamp(0., 1.);
        self
    }
}
impl RenderOnce for RangeSlider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        div()
            .id(self.id)
            .relative()
            .h(px(44.))
            .w_full()
            .child(
                div()
                    .absolute()
                    .top(px(20.))
                    .h(px(4.))
                    .w_full()
                    .rounded_full()
                    .bg(c.secondary_container),
            )
            .child(
                div()
                    .absolute()
                    .top(px(20.))
                    .left(relative(self.start))
                    .right(relative(1. - self.end))
                    .h(px(4.))
                    .bg(c.primary),
            )
    }
}

#[derive(IntoElement)]
pub struct Scrollbar {
    id: ElementId,
    position: f32,
    thickness: gpui::Pixels,
}
impl Scrollbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            position: 0.,
            thickness: px(4.),
        }
    }
    pub fn position(mut self, p: f32) -> Self {
        self.position = p.clamp(0., 1.);
        self
    }
    pub fn thickness(mut self, t: gpui::Pixels) -> Self {
        self.thickness = t;
        self
    }
}
impl RenderOnce for Scrollbar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .absolute()
            .right_0()
            .top(relative(self.position))
            .w(self.thickness)
            .h(px(48.))
            .rounded_full()
            .bg(cx.theme().colors().on_surface_variant.opacity(0.5))
    }
}

#[derive(IntoElement)]
pub struct SecureTextField {
    id: ElementId,
    label: SharedString,
    value: SharedString,
}
impl SecureTextField {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: SharedString::default(),
        }
    }
    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }
}
impl RenderOnce for SecureTextField {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        div()
            .id(self.id)
            .h(px(56.))
            .w_full()
            .rounded(px(4.))
            .border_1()
            .border_color(c.outline)
            .px(px(16.))
            .flex()
            .items_center()
            .text_color(c.on_surface)
            .child(if self.value.is_empty() {
                self.label
            } else {
                "••••••••".into()
            })
    }
}

#[derive(IntoElement)]
pub struct SearchBar {
    id: ElementId,
    query: SharedString,
    expanded: bool,
}
impl SearchBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            query: SharedString::default(),
            expanded: false,
        }
    }
    pub fn query(mut self, q: impl Into<SharedString>) -> Self {
        self.query = q.into();
        self
    }
    pub fn expanded(mut self, e: bool) -> Self {
        self.expanded = e;
        self
    }
}
impl RenderOnce for SearchBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        div()
            .id(self.id)
            .w_full()
            .min_h(px(56.))
            .rounded_full()
            .bg(c.surface_container_high)
            .px(px(20.))
            .flex()
            .items_center()
            .text_color(c.on_surface)
            .child(if self.query.is_empty() {
                "Search".into()
            } else {
                self.query
            })
    }
}

#[derive(IntoElement)]
pub struct SwipeToDismissBox {
    id: ElementId,
    content: gpui::AnyElement,
    background: Option<gpui::AnyElement>,
}
impl SwipeToDismissBox {
    pub fn new(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            content: content.into_any_element(),
            background: None,
        }
    }
    pub fn background(mut self, b: impl IntoElement) -> Self {
        self.background = Some(b.into_any_element());
        self
    }
}
impl RenderOnce for SwipeToDismissBox {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .relative()
            .when_some(self.background, |el, b| el.child(b))
            .child(self.content)
    }
}

#[derive(IntoElement)]
pub struct FloatingToolbar {
    id: ElementId,
    children: Vec<gpui::AnyElement>,
}
impl FloatingToolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
        }
    }
}
impl ParentElement for FloatingToolbar {
    fn extend(&mut self, e: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(e)
    }
}
impl RenderOnce for FloatingToolbar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .gap(px(8.))
            .p(px(8.))
            .rounded_full()
            .bg(cx.theme().colors().surface_container_high)
            .children(self.children)
    }
}

#[derive(IntoElement)]
pub struct FabMenu {
    id: ElementId,
    expanded: bool,
    actions: Vec<gpui::AnyElement>,
}
impl FabMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            expanded: false,
            actions: Vec::new(),
        }
    }
    pub fn expanded(mut self, e: bool) -> Self {
        self.expanded = e;
        self
    }
    pub fn action(mut self, a: impl IntoElement) -> Self {
        self.actions.push(a.into_any_element());
        self
    }
}
impl RenderOnce for FabMenu {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_end()
            .gap(px(8.))
            .when(self.expanded, |el| el.children(self.actions))
    }
}

#[derive(IntoElement)]
pub struct DatePicker {
    id: ElementId,
    value: SharedString,
}
impl DatePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "Select date".into(),
        }
    }
    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }
}
impl RenderOnce for DatePicker {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .p(px(16.))
            .rounded(px(12.))
            .bg(cx.theme().colors().surface_container_high)
            .text_color(cx.theme().colors().on_surface)
            .child(self.value)
    }
}
pub type ExposedDatePicker = DatePicker;
#[derive(IntoElement)]
pub struct TimePicker {
    id: ElementId,
    value: SharedString,
}
impl TimePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "Select time".into(),
        }
    }
    pub fn value(mut self, v: impl Into<SharedString>) -> Self {
        self.value = v.into();
        self
    }
}
impl RenderOnce for TimePicker {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .p(px(16.))
            .rounded(px(12.))
            .bg(cx.theme().colors().surface_container_high)
            .text_color(cx.theme().colors().on_surface)
            .child(self.value)
    }
}
pub type ExposedTimePicker = TimePicker;

#[derive(IntoElement)]
pub struct WideNavigationRail {
    id: ElementId,
    children: Vec<gpui::AnyElement>,
}
impl WideNavigationRail {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
        }
    }
}
impl ParentElement for WideNavigationRail {
    fn extend(&mut self, e: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(e)
    }
}
impl RenderOnce for WideNavigationRail {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .w(px(256.))
            .h_full()
            .flex()
            .flex_col()
            .gap(px(8.))
            .p(px(12.))
            .bg(cx.theme().colors().surface_container)
            .children(self.children)
    }
}
