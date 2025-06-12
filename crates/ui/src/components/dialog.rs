use std::rc::Rc;

use gpui::{prelude::FluentBuilder, *};

use crate::{ContextProvider, DialogContext, Theme, primitives::v_flex_center};

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Dialog {
    title: Option<AnyElement>,
    on_close: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    overlay: bool,
    overlay_closable: bool,
    pub(crate) focus_handle: FocusHandle,
    pub(crate) layer_ix: usize,
    pub(crate) overlay_visible: bool,
}

impl Dialog {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        Self {
            title: None,
            on_close: Rc::new(|_, _, _| {}),
            overlay: true,
            overlay_closable: true,
            focus_handle: cx.focus_handle(),
            layer_ix: 0,
            overlay_visible: false,
        }
    }

    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }

    pub fn overlay_closable(mut self, overlay_closable: bool) -> Self {
        self.overlay_closable = overlay_closable;
        self
    }

    pub fn on_close(
        mut self,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Rc::new(on_close);
        self
    }

    pub(crate) fn has_overlay(&self) -> bool {
        self.overlay
    }
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let on_close = self.on_close.clone();

        let viewport = window.viewport_size();
        let theme = cx.global::<Theme>();

        anchored().child(
            v_flex_center()
                .h(viewport.height)
                .w(viewport.width)
                .when(self.overlay_visible, |this| {
                    this.occlude().bg(rgba(0x00000F))
                })
                .when(self.overlay_closable, |this| {
                    if (self.layer_ix + 1) != ContextProvider::read(window, cx).active_modals.len()
                    {
                        return this;
                    }

                    this.on_mouse_down(MouseButton::Left, {
                        let on_close = on_close.clone();
                        move |_, window, cx| {
                            on_close(&ClickEvent::default(), window, cx);
                            window.close_dialog(cx);
                        }
                    })
                })
                .child(
                    div()
                        .border_1()
                        .border_color(theme.colors.border)
                        .bg(theme.colors.background)
                        .rounded(theme.rounded.lg)
                        .p(rems(2.))
                        .w(rems(50.))
                        .id("modal1")
                        .track_focus(&self.focus_handle)
                        .absolute()
                        .occlude()
                        .relative()
                        .when_some(self.title, |this, title| {
                            this.child(div().child(title).text_color(theme.colors.primary))
                        }),
                ),
        )
    }
}
