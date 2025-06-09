use gpui::*;

#[derive(Default)]
pub struct Theme {
    pub colors: Colors,
    pub icons: Icons,
}

pub struct Colors {
    pub background: Rgba,
    pub sidebar: Rgba,
    pub border: Rgba,
    pub primary: Rgba,
    pub secondary: Rgba,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            background: rgb(0x000000),
            sidebar: rgb(0x000000),
            border: rgb(0x373737),
            primary: rgb(0xbab6be),
            secondary: rgb(0x726f76),
        }
    }
}

pub struct Icons {
    pub logo: SharedString,
    pub library: SharedString,
    pub downloads: SharedString,
}

impl Default for Icons {
    fn default() -> Self {
        Self {
            logo: "icons/box.svg".into(),
            library: "icons/library.svg".into(),
            downloads: "icons/download.svg".into(),
        }
    }
}

impl Global for Theme {}
