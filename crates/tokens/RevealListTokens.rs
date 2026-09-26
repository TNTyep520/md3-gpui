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
use super::{ColorSchemeKeyTokens, ColorToken, ShapeKeyTokens, ShapeToken, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct RevealListTokens;
impl RevealListTokens {
    pub const ITEM_ACTION_BUTTON_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const ITEM_ACTION_ICON_BUTTON_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ITEM_BUTTON_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const ITEM_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_ICON_BUTTON_ACTION_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_ICON_BUTTON_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ITEM_ICON_BUTTON_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ITEM_SEGMENTED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
}
impl RevealListTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "RevealListTokens",
            name: "ItemActionButtonIconIconColor",
            value: TokenValue::ColorRole(RevealListTokens::ITEM_ACTION_BUTTON_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "RevealListTokens",
            name: "ItemActionIconButtonContainerColor",
            value: TokenValue::ColorRole(RevealListTokens::ITEM_ACTION_ICON_BUTTON_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "RevealListTokens",
            name: "ItemButtonIconIconColor",
            value: TokenValue::ColorRole(RevealListTokens::ITEM_BUTTON_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "RevealListTokens",
            name: "ItemContainerColor",
            value: TokenValue::ColorRole(RevealListTokens::ITEM_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "RevealListTokens",
            name: "ItemContainerShape",
            value: TokenValue::ShapeRole(RevealListTokens::ITEM_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "RevealListTokens",
            name: "ItemIconButtonActionContainerShape",
            value: TokenValue::ShapeRole(RevealListTokens::ITEM_ICON_BUTTON_ACTION_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "RevealListTokens",
            name: "ItemIconButtonContainerColor",
            value: TokenValue::ColorRole(RevealListTokens::ITEM_ICON_BUTTON_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "RevealListTokens",
            name: "ItemIconButtonContainerShape",
            value: TokenValue::ShapeRole(RevealListTokens::ITEM_ICON_BUTTON_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "RevealListTokens",
            name: "ItemSegmentedContainerShape",
            value: TokenValue::ShapeRole(RevealListTokens::ITEM_SEGMENTED_CONTAINER_SHAPE),
        },
    ];
}
