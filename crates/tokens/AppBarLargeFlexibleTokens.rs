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
use super::{Dp, TokenEntry, TokenValue, TypographyKeyTokens, TypographyToken};

#[derive(Clone, Copy, Debug, Default)]
pub struct AppBarLargeFlexibleTokens;
impl AppBarLargeFlexibleTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(120.0);
    pub const SUBTITLE_FONT: TypographyToken = TypographyKeyTokens::TITLE_MEDIUM;
    pub const TITLE_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_SMALL;
    pub const LARGE_CONTAINER_HEIGHT: Dp = Dp(152.0);
}
impl AppBarLargeFlexibleTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "AppBarLargeFlexibleTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(AppBarLargeFlexibleTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "AppBarLargeFlexibleTokens",
            name: "SubtitleFont",
            value: TokenValue::TypographyRole(AppBarLargeFlexibleTokens::SUBTITLE_FONT),
        },
        TokenEntry {
            group: "AppBarLargeFlexibleTokens",
            name: "TitleFont",
            value: TokenValue::TypographyRole(AppBarLargeFlexibleTokens::TITLE_FONT),
        },
        TokenEntry {
            group: "AppBarLargeFlexibleTokens",
            name: "LargeContainerHeight",
            value: TokenValue::Dp(AppBarLargeFlexibleTokens::LARGE_CONTAINER_HEIGHT),
        },
    ];
}
