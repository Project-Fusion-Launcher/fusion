use gpui::*;

#[derive(Default)]
pub struct Theme {
    pub colors: Colors,
    pub icons: Icons,
    pub text: Text,
    pub rounded: Rounded,
}

pub struct Colors {
    pub transparent: Rgba,

    pub background: Rgba,
    pub sidebar: Rgba,
    pub border: Rgba,
    pub accent: Rgba,
    pub primary: Rgba,
    pub primary_foreground: Rgba,
    pub secondary: Rgba,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            transparent: rgba(0x00000000),
            background: rgb(0x000000),
            sidebar: rgb(0x000000),
            border: rgb(0x373737),
            accent: rgb(0x874295),
            primary: rgb(0xbab6be),
            primary_foreground: rgb(0x000000),
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

#[derive(Default)]
pub struct Text {
    pub size: TextSize,
}

pub struct TextSize {
    pub xs: AbsoluteLength,
    pub sm: AbsoluteLength,
    pub md: AbsoluteLength,
    pub xl: AbsoluteLength,
}

impl Default for TextSize {
    fn default() -> Self {
        Self {
            xs: rems(0.75).into(),
            sm: rems(0.875).into(),
            md: rems(1.).into(),
            xl: rems(2.).into(),
        }
    }
}

pub struct Rounded {
    pub sm: AbsoluteLength,
    pub md: AbsoluteLength,
}

impl Default for Rounded {
    fn default() -> Self {
        Self {
            sm: rems(0.25).into(),
            md: rems(0.375).into(),
        }
    }
}

impl Global for Theme {}
