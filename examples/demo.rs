//! md3-gpui 组件演示
//!
//! 左侧导航选择组件类别，右侧展示对应组件。
//! 头部可切换亮/暗色与 Baseline/Expressive 主题（m3fx 令牌 profile），
//! 文本框页面输入 hex 种子色可实时应用动态色。
//!
//! 注意：Entity 化组件只在首帧创建一次（Gallery / Demo 字段持有），
//! 每帧重建会使涟漪与状态层动画失效。
//!
//! 运行：`cargo run --example demo`

use gpui::{
    AnyElement, App, Bounds, Context, Entity, IntoElement, Render, TitlebarOptions, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use gpui_platform::application;
use md3_gpui::overlay::{
    MenuItem, MenuState, Snackbar, close_tooltip, host, show_menu, show_snackbar, show_tooltip,
};
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
        "Common button variants with ripple and state layer springs (m3fx motion)",
        IconName::Add,
    ),
    (
        "Icon buttons & FAB",
        "Icon button variants and floating action button sizes",
        IconName::Favorite,
    ),
    (
        "Selection controls",
        "Checkbox, radio button and switch with spring-driven animations",
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
        "Primary tabs; the selection indicator slides with a fastSpatial spring",
        IconName::Menu,
    ),
    (
        "Text fields",
        "Outlined text field with floating label and focus morph",
        IconName::Edit,
    ),
    (
        "Overlays",
        "Snackbar, menu and tooltip rendered through the window overlay host",
        IconName::MoreVert,
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

const DEFAULT_SEED: u32 = 0x6750A4;

/// 一次创建、长期持有的页面组件集合。
///
/// Entity 化组件必须持久持有：每帧重建会让按压/涟漪状态丢失。
struct Gallery {
    dialog_open: bool,
    wired: bool,
    slider: Entity<SliderState>,
    tabbar: Entity<TabBarState>,
    seed_field: Entity<TextFieldState>,
    // Buttons 页
    b_filled: Entity<ButtonState>,
    b_tonal: Entity<ButtonState>,
    b_elevated: Entity<ButtonState>,
    b_outlined: Entity<ButtonState>,
    b_text: Entity<ButtonState>,
    b_icon: Entity<ButtonState>,
    b_disabled: Entity<ButtonState>,
    // Icon buttons & FAB 页
    ib_standard: Entity<IconButtonState>,
    ib_filled: Entity<IconButtonState>,
    ib_tonal: Entity<IconButtonState>,
    ib_outlined: Entity<IconButtonState>,
    fab_small: Entity<FabState>,
    fab_std: Entity<FabState>,
    fab_ext: Entity<FabState>,
    // Selection controls 页
    cb_a: Entity<CheckboxState>,
    cb_b: Entity<CheckboxState>,
    cb_disabled: Entity<CheckboxState>,
    radios: Vec<Entity<RadioState>>,
    // Chips 页
    chip_assist: Entity<ChipState>,
    chip_filters: Vec<Entity<ChipState>>,
    chip_input: Entity<ChipState>,
    chip_suggestion: Entity<ChipState>,
    // Text fields 页
    tf_name: Entity<TextFieldState>,
    tf_error: Entity<TextFieldState>,
    tf_disabled: Entity<TextFieldState>,
    // Overlays 页
    b_snack: Entity<ButtonState>,
    b_menu: Entity<ButtonState>,
    tooltip_trigger: Entity<ButtonState>,
    // Dialogs 页
    b_dialog: Entity<ButtonState>,
    dlg_cancel: Entity<ButtonState>,
    dlg_ok: Entity<ButtonState>,
}

impl Gallery {
    fn new(cx: &mut Context<Self>) -> Self {
        let slider = Slider::new(0., 100., 40.).step(1.).build(cx);

        let tabbar = TabBar::new("tabs")
            .tab(Tab::new("Home").icon(IconName::Home))
            .tab(Tab::new("Search").icon(IconName::Search))
            .tab(Tab::new("Profile").icon(IconName::Person))
            .selected(0)
            .build(cx);

        let seed_field = TextField::new("seed-field", "Seed color (hex)")
            .value("6750A4")
            .helper("Type a hex color (like 6750A4); the theme applies live")
            .build(cx);

        let _menu = MenuState::new()
            .item(
                MenuItem::new("Refresh")
                    .icon(IconName::Settings)
                    .on_click(|window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Refreshed"), None);
                    }),
            )
            .item(
                MenuItem::new("Send feedback")
                    .icon(IconName::Info)
                    .on_click(|window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Thanks for the feedback!"), None);
                    }),
            )
            .item(
                MenuItem::new("Settings")
                    .icon(IconName::Edit)
                    .on_click(|_, _| {}),
            )
            .build(cx);

        let b_filled = Button::new("b-filled", "Filled")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(window, cx, Snackbar::new("Filled button clicked"), None);
            })
            .build(cx);
        let b_tonal = Button::new("b-tonal", "Tonal").tonal().build(cx);
        let b_elevated = Button::new("b-elevated", "Elevated").elevated().build(cx);
        let b_outlined = Button::new("b-outlined", "Outlined").outlined().build(cx);
        let b_text = Button::new("b-text", "Text").text().build(cx);
        let b_icon = Button::new("b-icon", "With icon")
            .filled()
            .leading_icon(IconName::Add)
            .build(cx);
        let b_disabled = Button::new("b-disabled", "Disabled")
            .filled()
            .disabled(true)
            .build(cx);

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

        let cb_a = Checkbox::new("cb-a").checked(true).build(cx);
        let cb_b = Checkbox::new("cb-b").build(cx);
        let cb_disabled = Checkbox::new("cb-dis")
            .checked(true)
            .disabled(true)
            .build(cx);
        let radios = (0usize..3)
            .map(|ix| RadioButton::new(("radio", ix)).selected(ix == 0).build(cx))
            .collect();

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
                show_snackbar(window, cx, Snackbar::new("Chip removed"), None);
            })
            .build(cx);
        let chip_suggestion = Chip::new("chip-suggestion", "Suggestion")
            .suggestion()
            .elevated(true)
            .build(cx);

        let tf_name = TextField::new("tf-name", "Name").build(cx);
        let tf_error = TextField::new("tf-error", "Email")
            .error("Please enter a valid email address")
            .build(cx);
        let tf_disabled = TextField::new("tf-disabled", "Disabled")
            .disabled(true)
            .build(cx);

        let b_snack = Button::new("b-snack", "Show snackbar")
            .filled()
            .on_click(|_, window, cx| {
                show_snackbar(
                    window,
                    cx,
                    Snackbar::new("File archived")
                        .action("UNDO")
                        .on_action(|window, cx| {
                            show_snackbar(window, cx, Snackbar::new("Restored"), None);
                        }),
                    None,
                );
            })
            .build(cx);

        let menu_for_button = MenuState::new()
            .item(
                MenuItem::new("Refresh")
                    .icon(IconName::Settings)
                    .on_click(|window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Refreshed"), None);
                    }),
            )
            .item(
                MenuItem::new("Send feedback")
                    .icon(IconName::Info)
                    .on_click(|window, cx| {
                        show_snackbar(window, cx, Snackbar::new("Thanks for the feedback!"), None);
                    }),
            )
            .build(cx);
        let b_menu = Button::new("b-menu", "Show menu")
            .outlined()
            .on_click(move |event, window, cx| {
                let anchor = Bounds {
                    origin: event.position(),
                    size: size(px(0.), px(0.)),
                };
                show_menu(window, cx, menu_for_button.clone(), anchor);
            })
            .build(cx);
        let tooltip_trigger = Button::new("tooltip-trigger", "Hover me")
            .outlined()
            .build(cx);

        let b_dialog = Button::new("b-dialog", "Open dialog").outlined().build(cx);
        let dlg_cancel = Button::new("dlg-cancel", "Cancel").text().build(cx);
        let dlg_ok = Button::new("dlg-ok", "Delete").text().build(cx);

        Self {
            dialog_open: false,
            wired: false,
            slider,
            tabbar,
            seed_field,
            b_filled,
            b_tonal,
            b_elevated,
            b_outlined,
            b_text,
            b_icon,
            b_disabled,
            ib_standard,
            ib_filled,
            ib_tonal,
            ib_outlined,
            fab_small,
            fab_std,
            fab_ext,
            cb_a,
            cb_b,
            cb_disabled,
            radios,
            chip_assist,
            chip_filters,
            chip_input,
            chip_suggestion,
            tf_name,
            tf_error,
            tf_disabled,
            b_snack,
            b_menu,
            tooltip_trigger,
            b_dialog,
            dlg_cancel,
            dlg_ok,
        }
    }

    /// 首帧接线：需要 Gallery 自身句柄的回调
    fn wire(&mut self, cx: &mut Context<Self>) {
        if self.wired {
            return;
        }
        self.wired = true;
        let gallery = cx.entity();
        self.b_dialog.update(cx, |button, _| {
            let gallery = gallery.clone();
            button.set_on_click(move |_, _window, cx| {
                gallery.update(cx, |g, cx| {
                    g.dialog_open = true;
                    cx.notify();
                });
            });
        });
        let gallery = cx.entity();
        self.dlg_cancel.update(cx, |button, _| {
            let gallery = gallery.clone();
            button.set_on_click(move |_, _window, cx| {
                gallery.update(cx, |g, cx| {
                    g.dialog_open = false;
                    cx.notify();
                });
            });
        });
        let gallery = cx.entity();
        self.dlg_ok.update(cx, |button, _| {
            let gallery = gallery.clone();
            button.set_on_click(move |_, _window, cx| {
                gallery.update(cx, |g, cx| {
                    g.dialog_open = false;
                    cx.notify();
                });
            });
        });
    }
}

struct Demo {
    dark: bool,
    expressive: bool,
    seed: u32,
    page: usize,
    gallery: Entity<Gallery>,
    dark_switch: Entity<SwitchState>,
    profile_switch: Entity<SwitchState>,
    wired: bool,
}

impl Demo {
    fn new(cx: &mut Context<Self>) -> Self {
        let gallery = cx.new(Gallery::new);

        // slider / tabbar 变化时刷新父视图
        let slider = gallery.read(cx).slider.clone();
        cx.observe(&slider, |_, _, cx| cx.notify()).detach();
        let tabbar = gallery.read(cx).tabbar.clone();
        cx.observe(&tabbar, |_, _, cx| cx.notify()).detach();

        // 种子色输入实时应用动态色
        let seed_field = gallery.read(cx).seed_field.clone();
        cx.observe(&seed_field, |this, field, cx| {
            let value = field
                .read(cx)
                .value()
                .trim()
                .trim_start_matches('#')
                .to_string();
            if let Ok(seed) = u32::from_str_radix(&value, 16)
                && seed != this.seed
                && seed != 0
            {
                this.seed = seed;
                this.apply_theme(cx);
            }
        })
        .detach();

        let dark_switch = Switch::new("theme-switch").show_icon(true).build(cx);
        let profile_switch = Switch::new("profile-switch").build(cx);

        Self {
            dark: false,
            expressive: false,
            seed: DEFAULT_SEED,
            page: 0,
            gallery,
            dark_switch,
            profile_switch,
            wired: false,
        }
    }

    /// 以当前模式/Profile/种子色重建主题并刷新窗口
    fn apply_theme(&self, cx: &mut App) {
        let mode = if self.dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        let profile = if self.expressive {
            Profile::Expressive2025
        } else {
            Profile::Baseline2021
        };
        Theme::set(cx, Theme::from_seed(self.seed, mode, profile));
        cx.refresh_windows();
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
                .typography()
                .title_medium
                .apply(div())
                .text_color(theme.colors().primary)
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
        .child(theme.typography().title_medium.apply(div()).child(title))
        .child(
            theme
                .typography()
                .body_medium
                .apply(div())
                .text_color(theme.colors().on_surface_variant)
                .child("Cards contain content and actions about a single subject."),
        )
}

impl Render for Demo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 首帧接线
        if !self.wired {
            self.wired = true;
            let this = cx.entity();
            self.dark_switch.update(cx, |switch, _| {
                switch.set_on_change(move |checked, _window, cx| {
                    this.update(cx, |d, cx| {
                        d.dark = checked;
                        d.apply_theme(cx);
                    })
                });
            });
            let this = cx.entity();
            self.profile_switch.update(cx, |switch, _| {
                switch.set_on_change(move |checked, _window, cx| {
                    this.update(cx, |d, cx| {
                        d.expressive = checked;
                        d.apply_theme(cx);
                    })
                });
            });
        }
        self.gallery.update(cx, Gallery::wire);

        let this = cx.entity();
        let colors = cx.theme().colors().clone();
        let typography = *cx.theme().typography();
        let font_family = cx.theme().font_family().clone();
        let gallery_entity = self.gallery.clone();
        let gallery = gallery_entity.read(cx);
        let slider_value = gallery.slider.read(cx).value();
        let selected_tab = gallery.tabbar.read(cx).selected();
        let page = self.page.min(PAGES.len() - 1);
        let (page_title, page_subtitle, _) = PAGES[page];
        let dialog_open = gallery.dialog_open;
        let dlg_cancel = gallery.dlg_cancel.clone();
        let dlg_ok = gallery.dlg_ok.clone();

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
        let body: AnyElement = match page {
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
                    .child(gallery.b_filled.clone())
                    .child(gallery.b_tonal.clone())
                    .child(gallery.b_elevated.clone())
                    .child(gallery.b_outlined.clone())
                    .child(gallery.b_text.clone())
                    .child(gallery.b_icon.clone())
                    .child(gallery.b_disabled.clone()),
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
                    .child(gallery.ib_standard.clone())
                    .child(gallery.ib_filled.clone())
                    .child(gallery.ib_tonal.clone())
                    .child(gallery.ib_outlined.clone())
                    .child(gallery.fab_small.clone())
                    .child(gallery.fab_std.clone())
                    .child(gallery.fab_ext.clone()),
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
                    .child(gallery.cb_a.clone())
                    .child(gallery.cb_b.clone())
                    .child(gallery.cb_disabled.clone())
                    .child(div().w(px(8.)))
                    .children(gallery.radios.clone())
                    .child(div().w(px(8.)))
                    .child(self.dark_switch.clone())
                    .child(self.profile_switch.clone()),
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
                    .child(gallery.chip_assist.clone())
                    .children(gallery.chip_filters.clone())
                    .child(gallery.chip_input.clone())
                    .child(gallery.chip_suggestion.clone()),
            )
            .into_any_element(),

            // Slider & Progress
            5 => subsection(
                cx,
                "Slider & Progress",
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .child(gallery.slider.clone())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(16.))
                            .child(
                                div()
                                    .flex_1()
                                    .child(LinearProgress::new("lp").value(slider_value / 100.)),
                            )
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
            .into_any_element(),

            // Tabs
            6 => subsection(
                cx,
                "Primary tabs",
                div().flex().flex_col().child(gallery.tabbar.clone()).child(
                    div().p(px(16.)).child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(format!("Tab {} selected", selected_tab + 1)),
                    ),
                ),
            )
            .into_any_element(),

            // Text fields
            7 => subsection(
                cx,
                "Text fields",
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .max_w(px(480.))
                    .child(gallery.seed_field.clone())
                    .child(gallery.tf_name.clone())
                    .child(gallery.tf_error.clone())
                    .child(gallery.tf_disabled.clone()),
            )
            .into_any_element(),

            // Overlays
            8 => {
                let tooltip_trigger = gallery.tooltip_trigger.clone();
                subsection(
                    cx,
                    "Snackbar / Menu / Tooltip",
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(12.))
                        .child(gallery.b_snack.clone())
                        .child(gallery.b_menu.clone())
                        .child(
                            div()
                                .id("tooltip-wrap")
                                .cursor_pointer()
                                .on_hover(move |hovered, window, cx| {
                                    if *hovered {
                                        let bounds = tooltip_trigger.read(cx).bounds();
                                        show_tooltip(
                                            window,
                                            cx,
                                            "Tooltip via overlay host",
                                            bounds,
                                        );
                                    } else {
                                        close_tooltip(window, cx);
                                    }
                                })
                                .child(gallery.tooltip_trigger.clone()),
                        ),
                )
                .into_any_element()
            }

            // Cards
            9 => subsection(
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
            10 => subsection(
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
            _ => {
                subsection(cx, "Dialogs", div().child(gallery.b_dialog.clone())).into_any_element()
            }
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
                            .child(if self.expressive {
                                "Expressive"
                            } else {
                                "Baseline"
                            }),
                    )
                    .child(self.profile_switch.clone())
                    .child(
                        typography
                            .label_large
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(if self.dark { "Dark" } else { "Light" }),
                    )
                    .child(self.dark_switch.clone()),
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
            .id("demo-root")
            .relative()
            .size_full()
            .flex()
            .bg(colors.surface)
            .font_family(font_family)
            .text_color(colors.on_surface)
            .child(sidebar)
            .child(content)
            // 窗口级弹层宿主：Snackbar / Menu / Tooltip
            .child(host(window, cx))
            .when(dialog_open, |el| {
                let close = move |cx: &mut App| {
                    gallery_entity.update(cx, |g, cx| {
                        g.dialog_open = false;
                        cx.notify();
                    })
                };
                el.child(
                    Dialog::new("demo-dialog")
                        .icon(IconName::Delete)
                        .title("Permanently delete?")
                        .child(
                            "Deleting the selected items will also remove them from all synced \
                             devices. This action cannot be undone.",
                        )
                        .action(dlg_cancel.clone())
                        .action(dlg_ok.clone())
                        .on_dismiss(move |_w, cx| close(cx)),
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
