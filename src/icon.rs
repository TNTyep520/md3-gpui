//! Icon 元素：渲染内嵌的 Material Symbols 图标
//!
//! ```ignore
//! Icon::new(IconName::Favorite).size(px(24.)).color(theme.colors.primary)
//! ```
//! 不设置 `color` 时继承父元素文字颜色。

use gpui::{prelude::*, px, svg, App, Hsla, IntoElement, Pixels, RenderOnce, Styled, Window};

/// 内嵌图标名（对应 assets/icons/*.svg）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconName {
    Add,
    ArrowBack,
    Check,
    ChevronRight,
    Close,
    Delete,
    Edit,
    Favorite,
    Home,
    Info,
    Menu,
    MoreVert,
    Person,
    /// 内部使用：环形进度条的 270° 圆弧
    ProgressArc,
    Search,
    Settings,
    Star,
}

impl IconName {
    /// 资源路径（由 [`crate::assets::Md3Assets`] 提供）
    pub fn asset_path(&self) -> &'static str {
        match self {
            IconName::Add => "md3-icons/add.svg",
            IconName::ArrowBack => "md3-icons/arrow_back.svg",
            IconName::Check => "md3-icons/check.svg",
            IconName::ChevronRight => "md3-icons/chevron_right.svg",
            IconName::Close => "md3-icons/close.svg",
            IconName::Delete => "md3-icons/delete.svg",
            IconName::Edit => "md3-icons/edit.svg",
            IconName::Favorite => "md3-icons/favorite.svg",
            IconName::Home => "md3-icons/home.svg",
            IconName::Info => "md3-icons/info.svg",
            IconName::Menu => "md3-icons/menu.svg",
            IconName::MoreVert => "md3-icons/more_vert.svg",
            IconName::Person => "md3-icons/person.svg",
            IconName::ProgressArc => "md3-icons/progress_arc.svg",
            IconName::Search => "md3-icons/search.svg",
            IconName::Settings => "md3-icons/settings.svg",
            IconName::Star => "md3-icons/star.svg",
        }
    }
}

/// 图标元素
#[derive(IntoElement)]
pub struct Icon {
    name: IconName,
    size: Pixels,
    color: Option<Hsla>,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: px(24.),
            color: None,
        }
    }

    /// 图标尺寸（默认 24dp）
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    /// 图标颜色；不设置则继承文字颜色
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        svg()
            .path(self.name.asset_path())
            .size(self.size)
            .flex_none()
            .when_some(self.color, |el, color| el.text_color(color))
    }
}
