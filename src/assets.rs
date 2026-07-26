//! 内嵌图标资源（Material Symbols，Apache-2.0）
//!
//! gpui 的 `svg()` 元素通过 `AssetSource` 按路径加载资源，
//! 因此库把所需图标以字节形式内嵌，并提供 [`Md3Assets`] 作为资源源。
//!
//! 应用启动时安装：
//! ```ignore
//! gpui_platform::application().with_assets(Md3Assets).run(|cx| { ... })
//! ```
//! 如果你已有自己的 `AssetSource`，可用 [`Md3Assets::with_fallback`] 组合。

use anyhow::Result;
use gpui::{AssetSource, SharedString};
use std::borrow::Cow;

macro_rules! icons {
    ($($name:literal),* $(,)?) => {
        &[
            $((
                concat!("md3-icons/", $name, ".svg"),
                include_bytes!(concat!("../assets/icons/", $name, ".svg")).as_slice(),
            )),*
        ]
    };
}

/// (asset_path, bytes)
static ICONS: &[(&str, &[u8])] = icons![
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
    "progress_arc",
    "search",
    "settings",
    "star",
];

/// md3-gpui 的内嵌资源源
pub struct Md3Assets;

impl Md3Assets {
    /// 与另一个 AssetSource 组合：md3 图标优先，其余路径回退到 `fallback`。
    pub fn with_fallback(fallback: impl AssetSource) -> CombinedAssets {
        CombinedAssets {
            fallback: Box::new(fallback),
        }
    }

    fn find(path: &str) -> Option<&'static [u8]> {
        ICONS
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, bytes)| *bytes)
    }
}

impl AssetSource for Md3Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Md3Assets::find(path).map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(ICONS
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}

/// [`Md3Assets`] 与用户资源源的组合体
pub struct CombinedAssets {
    fallback: Box<dyn AssetSource>,
}

impl AssetSource for CombinedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = Md3Assets::find(path) {
            return Ok(Some(Cow::Borrowed(bytes)));
        }
        self.fallback.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut out: Vec<SharedString> = ICONS
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect();
        out.extend(self.fallback.list(path)?);
        Ok(out)
    }
}
