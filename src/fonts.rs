//! 内嵌字体注册（woff2 内嵌 + 运行时解码）。
//!
//! - **Roboto**（Apache-2.0）：MD3 默认正文字体，Regular / Medium / Bold
//!   三个字重（Google Fonts latin 切片 woff2，各约 22KB）。
//! - **Material Symbols Outlined 子集**（Apache-2.0）：只含
//!   [`crate::icon::IconName`] 用到的图标字形（woff2 约 3KB），
//!   [`crate::icon::Icon`] 以字体字形渲染图标（对齐 m3fx `M3Icon` 的实现方式）。
//!
//! 字体以 woff2 内嵌以压缩体积；gpui 的字体栈只接受 sfnt（ttf），
//! 注册前经纯 Rust 解码器 [`wuff`](https://crates.io/crates/wuff)
//! 运行时解回 ttf（毫秒级）。经 [`install`] 注册，[`crate::init`] 默认调用。
//!
//! 符号子集按官方 Material Symbols 自托管指引生成（Google Fonts css2：
//! `icon_names=` 按字母排序的连体名列表 + 轴实例化
//! `opsz,wght,FILL,GRAD@24,400,0,0`，woff2）。如需
//! [`crate::icon::IconName::Custom`] 任意图标，请自行下载完整
//! Material Symbols 字体并额外注册。

use std::borrow::Cow;

use gpui::App;

/// Material Symbols Outlined 图标字体族名（ligature 渲染时引用）。
pub const ICON_FONT_FAMILY: &str = "Material Symbols Outlined";

/// Roboto 正文字体族名（主题 `font_family` 默认值）。
pub const TEXT_FONT_FAMILY: &str = "Roboto";

const ROBOTO_REGULAR_WOFF2: &[u8] = include_bytes!("fonts/Roboto-400.woff2");
const ROBOTO_MEDIUM_WOFF2: &[u8] = include_bytes!("fonts/Roboto-500.woff2");
const ROBOTO_BOLD_WOFF2: &[u8] = include_bytes!("fonts/Roboto-700.woff2");
const MATERIAL_SYMBOLS_SUBSET_WOFF2: &[u8] = include_bytes!("fonts/MaterialSymbolsSubset.woff2");

/// 注册内嵌字体（Roboto 三字重 + Material Symbols 子集）。
///
/// woff2 在注册前解码为 ttf（毫秒级）。幂等：重复调用只会重复注册
/// （平台层通常去重）。单个字体解码失败则跳过并计入错误，不 panic。
pub fn install(cx: &mut App) -> anyhow::Result<()> {
    const SOURCES: [(&str, &[u8]); 4] = [
        ("Roboto Regular", ROBOTO_REGULAR_WOFF2),
        ("Roboto Medium", ROBOTO_MEDIUM_WOFF2),
        ("Roboto Bold", ROBOTO_BOLD_WOFF2),
        (
            "Material Symbols Outlined (subset)",
            MATERIAL_SYMBOLS_SUBSET_WOFF2,
        ),
    ];

    let decoded: anyhow::Result<Vec<Vec<u8>>> = SOURCES
        .into_iter()
        .map(|(label, woff2)| {
            wuff::decompress_woff2(woff2)
                .map_err(|err| anyhow::anyhow!("{label}: woff2 decode failed: {err:?}"))
        })
        .collect();
    let fonts: Vec<Cow<'static, [u8]>> = decoded?.into_iter().map(Cow::Owned).collect();

    cx.text_system().add_fonts(fonts)?;
    Ok(())
}
