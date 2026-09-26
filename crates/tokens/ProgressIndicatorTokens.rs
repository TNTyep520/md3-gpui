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
pub struct ProgressIndicatorTokens;
impl ProgressIndicatorTokens {
    pub const ACTIVE_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const STOP_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const STOP_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const TRACK_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
}
impl ProgressIndicatorTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ProgressIndicatorTokens",
            name: "ActiveIndicatorColor",
            value: TokenValue::ColorRole(ProgressIndicatorTokens::ACTIVE_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "ProgressIndicatorTokens",
            name: "ActiveShape",
            value: TokenValue::ShapeRole(ProgressIndicatorTokens::ACTIVE_SHAPE),
        },
        TokenEntry {
            group: "ProgressIndicatorTokens",
            name: "StopColor",
            value: TokenValue::ColorRole(ProgressIndicatorTokens::STOP_COLOR),
        },
        TokenEntry {
            group: "ProgressIndicatorTokens",
            name: "StopShape",
            value: TokenValue::ShapeRole(ProgressIndicatorTokens::STOP_SHAPE),
        },
        TokenEntry {
            group: "ProgressIndicatorTokens",
            name: "TrackColor",
            value: TokenValue::ColorRole(ProgressIndicatorTokens::TRACK_COLOR),
        },
        TokenEntry {
            group: "ProgressIndicatorTokens",
            name: "TrackShape",
            value: TokenValue::ShapeRole(ProgressIndicatorTokens::TRACK_SHAPE),
        },
    ];
}
