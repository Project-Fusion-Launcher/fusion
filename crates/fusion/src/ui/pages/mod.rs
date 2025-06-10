use gpui::*;
use ui::Theme;

mod library;

pub use library::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u64)]
pub enum Page {
    Library = 0,
    Downloads = 1,
}

impl From<Page> for SharedString {
    fn from(page: Page) -> Self {
        match page {
            Page::Library => "Library".into(),
            Page::Downloads => "Downloads".into(),
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

impl Global for Page {}
