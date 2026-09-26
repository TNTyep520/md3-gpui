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
pub struct LinearProgressIndicatorTokens;
impl LinearProgressIndicatorTokens {
    pub const ACTIVE_THICKNESS: Dp = Dp(4.0);
    pub const ACTIVE_WAVE_AMPLITUDE: Dp = Dp(3.0);
    pub const ACTIVE_WAVE_WAVELENGTH: Dp = Dp(40.0);
    pub const HEIGHT: Dp = Dp(4.0);
    pub const INDETERMINATE_ACTIVE_WAVE_WAVELENGTH: Dp = Dp(20.0);
    pub const STOP_SIZE: Dp = Dp(4.0);
    pub const STOP_TRAILING_SPACE: Dp = Dp(0.0);
    pub const TRACK_ACTIVE_SPACE: Dp = Dp(4.0);
    pub const TRACK_THICKNESS: Dp = Dp(4.0);
    pub const WAVE_HEIGHT: Dp = Dp(10.0);
}
impl LinearProgressIndicatorTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "ActiveThickness",
            value: TokenValue::Dp(LinearProgressIndicatorTokens::ACTIVE_THICKNESS),
        },
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "ActiveWaveAmplitude",
            value: TokenValue::Dp(LinearProgressIndicatorTokens::ACTIVE_WAVE_AMPLITUDE),
        },
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "ActiveWaveWavelength",
            value: TokenValue::Dp(LinearProgressIndicatorTokens::ACTIVE_WAVE_WAVELENGTH),
        },
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "Height",
            value: TokenValue::Dp(LinearProgressIndicatorTokens::HEIGHT),
        },
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "IndeterminateActiveWaveWavelength",
            value: TokenValue::Dp(
                LinearProgressIndicatorTokens::INDETERMINATE_ACTIVE_WAVE_WAVELENGTH,
            ),
        },
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "StopSize",
            value: TokenValue::Dp(LinearProgressIndicatorTokens::STOP_SIZE),
        },
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "StopTrailingSpace",
            value: TokenValue::Dp(LinearProgressIndicatorTokens::STOP_TRAILING_SPACE),
        },
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "TrackActiveSpace",
            value: TokenValue::Dp(LinearProgressIndicatorTokens::TRACK_ACTIVE_SPACE),
        },
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "TrackThickness",
            value: TokenValue::Dp(LinearProgressIndicatorTokens::TRACK_THICKNESS),
        },
        TokenEntry {
            group: "LinearProgressIndicatorTokens",
            name: "WaveHeight",
            value: TokenValue::Dp(LinearProgressIndicatorTokens::WAVE_HEIGHT),
        },
    ];
}
