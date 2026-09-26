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
    ColorSchemeKeyTokens, ColorToken, ShapeKeyTokens, ShapeToken, TokenEntry, TokenValue,
    TypographyKeyTokens, TypographyToken,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct PlainTooltipTokens;
impl PlainTooltipTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_SURFACE;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
}
impl PlainTooltipTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "PlainTooltipTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(PlainTooltipTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "PlainTooltipTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(PlainTooltipTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "PlainTooltipTokens",
            name: "SupportingTextColor",
            value: TokenValue::ColorRole(PlainTooltipTokens::SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "PlainTooltipTokens",
            name: "SupportingTextFont",
            value: TokenValue::TypographyRole(PlainTooltipTokens::SUPPORTING_TEXT_FONT),
        },
    ];
}
