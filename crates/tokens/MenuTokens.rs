/*
 * Copyright 2021 The Android Open Source Project
 * Copyright 2022 The Android Open Source Project
 * Copyright 2023 The Android Open Source Project
 * Copyright 2024 The Android Open Source Project
 * Copyright 2025 The Android Open Source Project
 * Copyright 2026 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use super::{
    ColorSchemeKeyTokens, ColorToken, Dp, ElevationTokens, ShapeKeyTokens, ShapeToken, TokenEntry,
    TokenValue,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct MenuTokens;
impl MenuTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const LIST_ITEM_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const LIST_ITEM_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const LIST_ITEM_SELECTED_LEADING_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const MENU_LIST_ITEM_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
impl MenuTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "MenuTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(MenuTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "MenuTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(MenuTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "MenuTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(MenuTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "MenuTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(MenuTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "MenuTokens",
            name: "ListItemSelectedContainerColor",
            value: TokenValue::ColorRole(MenuTokens::LIST_ITEM_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "MenuTokens",
            name: "ListItemSelectedLabelTextColor",
            value: TokenValue::ColorRole(MenuTokens::LIST_ITEM_SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "MenuTokens",
            name: "ListItemSelectedLeadingTrailingIconColor",
            value: TokenValue::ColorRole(
                MenuTokens::LIST_ITEM_SELECTED_LEADING_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "MenuTokens",
            name: "MenuListItemLeadingIconColor",
            value: TokenValue::ColorRole(MenuTokens::MENU_LIST_ITEM_LEADING_ICON_COLOR),
        },
    ];
}
