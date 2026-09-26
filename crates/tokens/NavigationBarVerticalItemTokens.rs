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
use super::{Dp, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct NavigationBarVerticalItemTokens;
impl NavigationBarVerticalItemTokens {
    pub const ACTIVE_INDICATOR_HEIGHT: Dp = Dp(32.0);
    pub const ACTIVE_INDICATOR_WIDTH: Dp = Dp(56.0);
    pub const CONTAINER_BETWEEN_SPACE: Dp = Dp(6.0);
    pub const ICON_SIZE: Dp = Dp(24.0);
}
impl NavigationBarVerticalItemTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "NavigationBarVerticalItemTokens",
            name: "ActiveIndicatorHeight",
            value: TokenValue::Dp(NavigationBarVerticalItemTokens::ACTIVE_INDICATOR_HEIGHT),
        },
        TokenEntry {
            group: "NavigationBarVerticalItemTokens",
            name: "ActiveIndicatorWidth",
            value: TokenValue::Dp(NavigationBarVerticalItemTokens::ACTIVE_INDICATOR_WIDTH),
        },
        TokenEntry {
            group: "NavigationBarVerticalItemTokens",
            name: "ContainerBetweenSpace",
            value: TokenValue::Dp(NavigationBarVerticalItemTokens::CONTAINER_BETWEEN_SPACE),
        },
        TokenEntry {
            group: "NavigationBarVerticalItemTokens",
            name: "IconSize",
            value: TokenValue::Dp(NavigationBarVerticalItemTokens::ICON_SIZE),
        },
    ];
}
