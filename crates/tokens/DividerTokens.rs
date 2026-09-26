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
use super::{ColorSchemeKeyTokens, ColorToken, Dp, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct DividerTokens;
impl DividerTokens {
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const THICKNESS: Dp = Dp(1.0);
}
impl DividerTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "DividerTokens",
            name: "Color",
            value: TokenValue::ColorRole(DividerTokens::COLOR),
        },
        TokenEntry {
            group: "DividerTokens",
            name: "Thickness",
            value: TokenValue::Dp(DividerTokens::THICKNESS),
        },
    ];
}
