//! md3-gpui 组件演示
//!
//! 左侧导航选择组件类别，右侧展示对应组件。
//! 运行：`cargo run --example demo`

use gpui::{
    div, prelude::*, px, size, AnyElement, App, Bounds, Context, Entity, IntoElement, Render,
    SharedString, TitlebarOptions, Window, WindowBounds, WindowOptions,
};
use gpui_platform::application;
use md3_gpui::prelude::*;

/// 页面清单：(导航标题, 副标题, 图标)
const PAGES: &[(&str, &str, IconName)] = &[
    (
        "Components overview",
        "Browse every implemented Material Design 3 component in md3-gpui",
        IconName::Home,
    ),
    (
        "Buttons",
        "Common button variants: filled, tonal, elevated, outlined and text",
        IconName::Add,
    ),
    (
        "Icon buttons & FAB",
        "Icon button variants and floating action button sizes",
        IconName::Favorite,
    ),
    (
        "Selection controls",
        "Checkbox, radio button and switch",
        IconName::Check,
    ),
    (
        "Chips",
        "Assist, filter, input and suggestion chips",
        IconName::Info,
    ),
    (
        "Slider & Progress",
        "Slider with linear and circular progress indicators",
        IconName::Settings,
    ),
    (
        "Tabs",
        "Primary tabs with icons and selection indicator",
        IconName::Menu,
    ),
    (
        "Cards",
        "Elevated, filled and outlined cards",
        IconName::Star,
    ),
    (
        "Lists",
        "List items with icons, supporting text and dividers",
        IconName::Person,
    ),
    (
        "Dialogs",
        "Modal dialog with scrim, hero icon and actions",
        IconName::Delete,
    ),
];

struct Demo {
    dark: bool,
    page: usize,
    checkbox_a: bool,
    checkbox_b: bool,
    radio: usize,
    switch_a: bool,
    switch_b: bool,
    filter_chips: [bool; 3],
    input_chips: Vec<SharedString>,
    selected_tab: usize,
    dialog_open: bool,
    fav_selected: bool,
    slider: Entity<Slider>,
}

impl Demo {
    fn new(cx: &mut Context<Self>) -> Self {
        let slider = cx.new(|_| Slider::new(0., 100., 40.).step(1.));
        // slider 变化时刷新父视图
        cx.observe(&slider, |_, _, cx| cx.notify()).detach();
        Self {
            dark: false,
            page: 0,
            checkbox_a: true,
            checkbox_b: false,
            radio: 0,
            switch_a: true,
            switch_b: false,
            filter_chips: [true, false, false],
            input_chips: vec!["Rust".into(), "GPUI".into(), "MD3".into()],
            selected_tab: 0,
            dialog_open: false,
            fav_selected: false,
            slider,
        }
    }
}

/// 内容区小节标题
fn subsection(cx: &App, title: &'static str, content: impl IntoElement) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            theme
                .typography
                .title_medium
                .apply(div())
                .text_color(theme.colors.primary)
                .child(title),
        )
        .child(content)
}

fn demo_card(cx: &App, card: Card, title: &'static str) -> impl IntoElement {
    let theme = cx.theme();
    card.w(px(220.))
        .p(px(16.))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(theme.typography.title_medium.apply(div()).child(title))
        .child(
            theme
                .typography
                .body_medium
                .apply(div())
                .text_color(theme.colors.on_surface_variant)
                .child("Cards contain content and actions about a single subject."),
        )
}

impl Render for Demo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();
        let theme = cx.theme();
        let colors = theme.colors.clone();
        let typography = theme.typography.clone();
        let font_family = theme.font_family.clone();
        let slider_value = self.slider.read(cx).value();
        let page = self.page.min(PAGES.len() - 1);
        let (page_title, page_subtitle, _) = PAGES[page];

        // 左侧导航栏
        let sidebar = div()
            .w(px(280.))
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .p(px(12.))
            .gap(px(4.))
            .bg(colors.surface)
            .child(
                // 标题
                div().h(px(56.)).px(px(16.)).flex().items_center().child(
                    typography
                        .title_large
                        .apply(div())
                        .text_color(colors.on_surface)
                        .child("md3-gpui"),
                ),
            )
            .children(PAGES.iter().enumerate().map(|(ix, (title, _, _))| {
                let selected = ix == page;
                let (item_bg, item_fg) = if selected {
                    (
                        Some(colors.secondary_container),
                        colors.on_secondary_container,
                    )
                } else {
                    (None, colors.on_surface_variant)
                };
                let layer = colors.on_surface;
                let item = div()
                    .id(("nav", ix))
                    .h(px(48.))
                    .px(px(16.))
                    .flex()
                    .flex_none()
                    .items_center()
                    .rounded_full()
                    .cursor_pointer()
                    .text_color(item_fg)
                    .when_some(item_bg, |el, bg| el.bg(bg))
                    .when(!selected, |el| el.hover(move |s| s.bg(layer.opacity(0.08))))
                    .on_click({
                        let this = this.clone();
                        move |_, _w, cx| {
                            this.update(cx, |d, cx| {
                                d.page = ix;
                                cx.notify();
                            })
                        }
                    })
                    .child(*title);
                typography.label_large.apply(item)
            }));

        // 右侧内容

        // 内容主体
        let body: AnyElement =
            match page {
                // Overview
                0 => div()
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .child(
                        typography
                            .title_small
                            .apply(div())
                            .text_color(colors.on_surface)
                            .child("Material Components"),
                    )
                    .child(Card::new().outlined().overflow_hidden().child(
                        List::new().children(PAGES.iter().enumerate().skip(1).map(
                            |(ix, (title, subtitle, icon))| {
                                ListItem::new(("overview", ix), *title)
                                    .supporting_text(*subtitle)
                                    .leading_icon(*icon)
                                    .trailing(
                                        Icon::new(IconName::ChevronRight)
                                            .size(px(24.))
                                            .color(colors.on_surface_variant),
                                    )
                                    .on_click({
                                        let this = this.clone();
                                        move |_, _w, cx| {
                                            this.update(cx, |d, cx| {
                                                d.page = ix;
                                                cx.notify();
                                            })
                                        }
                                    })
                            },
                        )),
                    ))
                    .into_any_element(),

                // Buttons
                1 => subsection(
                    cx,
                    "Common buttons",
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(12.))
                        .child(Button::new("b-filled", "Filled").filled())
                        .child(Button::new("b-tonal", "Tonal").tonal())
                        .child(Button::new("b-elevated", "Elevated").elevated())
                        .child(Button::new("b-outlined", "Outlined").outlined())
                        .child(Button::new("b-text", "Text").text())
                        .child(
                            Button::new("b-icon", "With icon")
                                .filled()
                                .leading_icon(IconName::Add),
                        )
                        .child(
                            Button::new("b-disabled", "Disabled")
                                .filled()
                                .disabled(true),
                        ),
                )
                .into_any_element(),

                // Icon buttons & FAB
                2 => subsection(
                    cx,
                    "Icon buttons & FAB",
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(12.))
                        .child(
                            IconButton::new("ib-standard", IconName::Favorite)
                                .selected(self.fav_selected)
                                .on_click({
                                    let this = this.clone();
                                    move |_, _w, cx| {
                                        this.update(cx, |d, cx| {
                                            d.fav_selected = !d.fav_selected;
                                            cx.notify();
                                        })
                                    }
                                }),
                        )
                        .child(IconButton::new("ib-filled", IconName::Edit).filled())
                        .child(IconButton::new("ib-tonal", IconName::Settings).tonal())
                        .child(IconButton::new("ib-outlined", IconName::MoreVert).outlined())
                        .child(Fab::new("fab-small", IconName::Edit).size(FabSize::Small))
                        .child(Fab::new("fab-std", IconName::Add))
                        .child(
                            Fab::new("fab-ext", IconName::Add)
                                .color(FabColor::Tertiary)
                                .label("Compose"),
                        ),
                )
                .into_any_element(),

                // Selection controls
                3 => subsection(
                    cx,
                    "Checkbox / Radio / Switch",
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(16.))
                        .child(Checkbox::new("cb-a").checked(self.checkbox_a).on_change({
                            let this = this.clone();
                            move |checked, _w, cx| {
                                this.update(cx, |d, cx| {
                                    d.checkbox_a = checked;
                                    cx.notify();
                                })
                            }
                        }))
                        .child(Checkbox::new("cb-b").checked(self.checkbox_b).on_change({
                            let this = this.clone();
                            move |checked, _w, cx| {
                                this.update(cx, |d, cx| {
                                    d.checkbox_b = checked;
                                    cx.notify();
                                })
                            }
                        }))
                        .child(Checkbox::new("cb-dis").checked(true).disabled(true))
                        .child(div().w(px(8.)))
                        .children((0..3).map(|ix| {
                            RadioButton::new(("radio", ix))
                                .selected(self.radio == ix)
                                .on_select({
                                    let this = this.clone();
                                    move |_w, cx| {
                                        this.update(cx, |d, cx| {
                                            d.radio = ix;
                                            cx.notify();
                                        })
                                    }
                                })
                        }))
                        .child(div().w(px(8.)))
                        .child(Switch::new("sw-a").checked(self.switch_a).on_change({
                            let this = this.clone();
                            move |checked, _w, cx| {
                                this.update(cx, |d, cx| {
                                    d.switch_a = checked;
                                    cx.notify();
                                })
                            }
                        }))
                        .child(
                            Switch::new("sw-b")
                                .checked(self.switch_b)
                                .show_icon(true)
                                .on_change({
                                    let this = this.clone();
                                    move |checked, _w, cx| {
                                        this.update(cx, |d, cx| {
                                            d.switch_b = checked;
                                            cx.notify();
                                        })
                                    }
                                }),
                        ),
                )
                .into_any_element(),

                // Chips
                4 => subsection(
                    cx,
                    "Chips",
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(8.))
                        .child(
                            Chip::new("chip-assist", "Assist")
                                .assist()
                                .leading_icon(IconName::Info),
                        )
                        .children(["Alpha", "Beta", "Gamma"].into_iter().enumerate().map(
                            |(ix, label)| {
                                Chip::new(("chip-filter", ix), label)
                                    .filter()
                                    .selected(self.filter_chips[ix])
                                    .on_click({
                                        let this = this.clone();
                                        move |_, _w, cx| {
                                            this.update(cx, |d, cx| {
                                                d.filter_chips[ix] = !d.filter_chips[ix];
                                                cx.notify();
                                            })
                                        }
                                    })
                            },
                        ))
                        .children(self.input_chips.iter().cloned().enumerate().map(
                            |(ix, label)| {
                                Chip::new(("chip-input", ix), label).input().on_remove({
                                    let this = this.clone();
                                    move |_, _w, cx| {
                                        this.update(cx, |d, cx| {
                                            if ix < d.input_chips.len() {
                                                d.input_chips.remove(ix);
                                                cx.notify();
                                            }
                                        })
                                    }
                                })
                            },
                        ))
                        .child(
                            Chip::new("chip-suggestion", "Suggestion")
                                .suggestion()
                                .elevated(true),
                        ),
                )
                .into_any_element(),

                // Slider & Progress
                5 => {
                    subsection(
                        cx,
                        "Slider & Progress",
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.))
                            .child(self.slider.clone())
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(16.))
                                    .child(div().flex_1().child(
                                        LinearProgress::new("lp").value(slider_value / 100.),
                                    ))
                                    .child(
                                        typography
                                            .label_medium
                                            .apply(div())
                                            .text_color(colors.on_surface_variant)
                                            .child(format!("{:.0}%", slider_value)),
                                    ),
                            )
                            .child(LinearProgress::new("lp-ind").indeterminate())
                            .child(CircularProgress::new().size(px(40.))),
                    )
                    .into_any_element()
                }

                // Tabs
                6 => subsection(
                    cx,
                    "Primary tabs",
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            TabBar::new("tabs")
                                .tab(Tab::new("Home").icon(IconName::Home))
                                .tab(Tab::new("Search").icon(IconName::Search))
                                .tab(Tab::new("Profile").icon(IconName::Person))
                                .selected(self.selected_tab)
                                .on_change({
                                    let this = this.clone();
                                    move |ix, _w, cx| {
                                        this.update(cx, |d, cx| {
                                            d.selected_tab = ix;
                                            cx.notify();
                                        })
                                    }
                                }),
                        )
                        .child(
                            div().p(px(16.)).child(
                                typography
                                    .body_medium
                                    .apply(div())
                                    .text_color(colors.on_surface_variant)
                                    .child(format!("Tab {} selected", self.selected_tab + 1)),
                            ),
                        ),
                )
                .into_any_element(),

                // Cards
                7 => subsection(
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
                .into_any_element(),

                // Lists
                8 => subsection(
                    cx,
                    "Lists",
                    Card::new().outlined().overflow_hidden().child(
                        List::new()
                            .child(
                                ListItem::new("li-1", "Photos")
                                    .supporting_text("Jan 9, 2026")
                                    .leading_icon(IconName::Star)
                                    .trailing_icon(IconName::MoreVert)
                                    .on_click(|_, _, _| {}),
                            )
                            .child(Divider::horizontal().inset())
                            .child(
                                ListItem::new("li-2", "Recipes")
                                    .supporting_text("Updated yesterday")
                                    .leading_icon(IconName::Favorite)
                                    .trailing_text("2")
                                    .on_click(|_, _, _| {}),
                            )
                            .child(Divider::horizontal().inset())
                            .child(
                                ListItem::new("li-3", "Settings")
                                    .leading_icon(IconName::Settings)
                                    .on_click(|_, _, _| {}),
                            ),
                    ),
                )
                .into_any_element(),

                // Dialogs
                _ => subsection(
                    cx,
                    "Dialogs",
                    div().child(
                        Button::new("open-dialog", "Open dialog")
                            .outlined()
                            .on_click({
                                let this = this.clone();
                                move |_, _w, cx| {
                                    this.update(cx, |d, cx| {
                                        d.dialog_open = true;
                                        cx.notify();
                                    })
                                }
                            }),
                    ),
                )
                .into_any_element(),
            };

        // 页头
        let content_header = div()
            .flex()
            .items_start()
            .justify_between()
            .gap(px(16.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(
                        typography
                            .headline_large
                            .apply(div())
                            .text_color(colors.on_surface)
                            .child(page_title),
                    )
                    .child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(page_subtitle),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap(px(12.))
                    .child(
                        typography
                            .label_large
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(if self.dark { "Dark" } else { "Light" }),
                    )
                    .child(
                        Switch::new("theme-switch")
                            .checked(self.dark)
                            .show_icon(true)
                            .on_change({
                                let this = this.clone();
                                move |checked, _window, cx| {
                                    this.update(cx, |d, cx| {
                                        d.dark = checked;
                                        Theme::set(
                                            cx,
                                            if checked {
                                                Theme::dark()
                                            } else {
                                                Theme::light()
                                            },
                                        );
                                        // 刷新 slider 缓存
                                        d.slider.update(cx, |_, cx| cx.notify());
                                        cx.notify();
                                    })
                                }
                            }),
                    ),
            );

        let content = div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .bg(colors.surface_container_lowest)
            .border_l_1()
            .border_color(colors.outline_variant)
            .child(
                div()
                    .px(px(32.))
                    .pt(px(32.))
                    .pb(px(16.))
                    .child(content_header),
            )
            .child(
                div()
                    .id("demo-content")
                    .flex_1()
                    .overflow_y_scroll()
                    .px(px(32.))
                    .pb(px(32.))
                    .child(body),
            );

        div()
            .size_full()
            .flex()
            .bg(colors.surface)
            .font_family(font_family)
            .text_color(colors.on_surface)
            .child(sidebar)
            .child(content)
            .when(self.dialog_open, |el| {
                let close = |this: &Entity<Demo>| {
                    let this = this.clone();
                    move |cx: &mut App| {
                        this.update(cx, |d, cx| {
                            d.dialog_open = false;
                            cx.notify();
                        })
                    }
                };
                let close_a = close(&this);
                let close_b = close(&this);
                let close_c = close(&this);
                el.child(
                    Dialog::new("demo-dialog")
                        .icon(IconName::Delete)
                        .title("Permanently delete?")
                        .child(
                            "Deleting the selected items will also remove them from all synced \
                             devices. This action cannot be undone.",
                        )
                        .action(
                            Button::new("dlg-cancel", "Cancel")
                                .text()
                                .on_click(move |_, _w, cx| close_a(cx)),
                        )
                        .action(
                            Button::new("dlg-ok", "Delete")
                                .text()
                                .on_click(move |_, _w, cx| close_b(cx)),
                        )
                        .on_dismiss(move |_w, cx| close_c(cx)),
                )
            })
    }
}

fn main() {
    application().with_assets(Md3Assets).run(|cx: &mut App| {
        md3_gpui::init(cx);
        let bounds = Bounds::centered(None, size(px(1200.), px(860.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("md3-gpui".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(Demo::new),
        )
        .unwrap();
        cx.activate(true);
    });
}
