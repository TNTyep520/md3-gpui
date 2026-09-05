//! MD3 令牌 Profile。
//!
//! 移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `org.glavo.m3fx.tokens.M3Profile`（Apache-2.0，© 2026 Glavo）。
//!
//! Profile 是一份"预设身份"，决定动态色的规格版本、色彩风格，
//! 以及 typography / shape / motion / 组件令牌的默认家族。

use mcu_dynamiccolor::{SpecVersion, Variant};

/// 令牌 Profile 预设。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Profile {
    /// 2021 基线（Material Design 3 baseline）：SPEC_2021 + TONAL_SPOT。
    #[default]
    Baseline2021,
    /// 2025 Expressive：SPEC_2025 + EXPRESSIVE 色彩风格 + Expressive 令牌家族。
    Expressive2025,
}

impl Profile {
    /// 该 Profile 使用的动态色规格版本。
    pub fn color_spec_version(self) -> SpecVersion {
        match self {
            Profile::Baseline2021 => SpecVersion::Spec2021,
            Profile::Expressive2025 => SpecVersion::Spec2025,
        }
    }

    /// 该 Profile 使用的色彩风格（动态色 Variant）。
    pub fn color_style(self) -> Variant {
        match self {
            Profile::Baseline2021 => Variant::TonalSpot,
            Profile::Expressive2025 => Variant::Expressive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profiles_map_to_specs() {
        assert_eq!(
            Profile::Baseline2021.color_spec_version(),
            SpecVersion::Spec2021
        );
        assert_eq!(
            Profile::Expressive2025.color_spec_version(),
            SpecVersion::Spec2025
        );
        assert_eq!(Profile::Baseline2021.color_style(), Variant::TonalSpot);
        assert_eq!(Profile::Expressive2025.color_style(), Variant::Expressive);
    }
}
