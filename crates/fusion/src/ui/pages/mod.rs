use gpui::SharedString;
use std::fmt;
use ui::Theme;

mod library;

#[derive(Clone, Copy, PartialEq)]
#[repr(u64)]
pub enum Page {
    Library = 0,
    Downloads = 1,
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Page::Library => write!(f, "Library"),
            Page::Downloads => write!(f, "Downloads"),
        }
    }
}

impl Page {
    pub fn icon(&self, theme: &Theme) -> SharedString {
        match self {
            Page::Library => theme.icons.library.clone(),
            Page::Downloads => theme.icons.downloads.clone(),
        }
    }
}
