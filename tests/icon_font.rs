//! 图标字体子集的健康检查。
//!
//! 验证 `assets/fonts/MaterialSymbolsSubset.woff2`：
//! 1. wuff 可解码为 sfnt；
//! 2. 保留 GSUB 表（ligature 替换所必需）；
//! 3. cmap 覆盖 ligature 组件字母（连字触发的前提）。

const SUBSET_WOFF2: &[u8] = include_bytes!("../src/fonts/MaterialSymbolsSubset.woff2");

#[test]
fn subset_font_supports_ligatures() {
    let ttf = wuff::decompress_woff2(SUBSET_WOFF2).expect("woff2 decode failed");
    let face = ttf_parser::Face::parse(&ttf, 0).expect("sfnt parse failed");

    // 1. GSUB（连字替换表）必须存在且有脚本记录
    let gsub = face
        .tables()
        .gsub
        .expect("subset font is missing the GSUB table");
    assert!(
        !gsub.scripts.is_empty(),
        "GSUB table has no scripts; ligatures cannot trigger"
    );

    // 2. cmap 必须覆盖全部 ligature 组件字母（IconName 全部 ligature 名的字符）

    let ligatures: &[&str] = &[
        "add",
        "arrow_back",
        "check",
        "chevron_right",
        "close",
        "delete",
        "edit",
        "favorite",
        "home",
        "info",
        "menu",
        "more_vert",
        "person",
        "progress_activity",
        "search",
        "settings",
        "star",
    ];
    for name in ligatures {
        for ch in name.chars() {
            assert!(
                face.glyph_index(ch).is_some(),
                "cmap is missing component glyph {ch:?} (ligature {name:?})"
            );
        }
    }
}

/// 库内嵌字体体积守卫：woff2 内嵌总体积保持在百 KB 量级。
#[test]
fn embedded_fonts_stay_small() {
    let roboto: usize = [
        include_bytes!("../src/fonts/Roboto-400.woff2").len(),
        include_bytes!("../src/fonts/Roboto-500.woff2").len(),
        include_bytes!("../src/fonts/Roboto-700.woff2").len(),
    ]
    .into_iter()
    .sum();
    let symbols = include_bytes!("../src/fonts/MaterialSymbolsSubset.woff2").len();
    assert!(roboto < 200_000, "Roboto woff2 grew to {roboto} bytes");
    assert!(symbols < 20_000, "icon subset grew to {symbols} bytes");
}
