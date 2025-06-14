use crate::{
    PortalContext, PortalContextProvider, Theme,
    components::{Button, ButtonVariant},
    primitives::{v_flex, v_flex_center},
};
use gpui::{prelude::FluentBuilder, *};
use std::rc::Rc;

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Modal {
    title: Option<AnyElement>,
    description: Option<AnyElement>,
    content: Div,
    on_close: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    overlay: bool,
    overlay_closable: bool,
    show_close: bool,
    pub(crate) focus_handle: FocusHandle,
    pub(crate) layer_ix: usize,
    pub(crate) overlay_visible: bool,
}

impl Modal {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        Self {
            title: None,
            description: None,
            content: v_flex(),
            on_close: Rc::new(|_, _, _| {}),
            overlay: true,
            overlay_closable: true,
            show_close: true,
            focus_handle: cx.focus_handle(),
            layer_ix: 0,
            overlay_visible: false,
        }
    }

    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }

    pub fn description(mut self, subtitle: impl IntoElement) -> Self {
        self.description = Some(subtitle.into_any_element());
        self
    }

    pub fn overlay_closable(mut self, overlay_closable: bool) -> Self {
        self.overlay_closable = overlay_closable;
        self
    }

    pub fn show_close(mut self, show_close: bool) -> Self {
        self.show_close = show_close;
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

impl ParentElement for Modal {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.content.extend(elements);
    }
}

impl RenderOnce for Modal {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let on_close = self.on_close.clone();

        let viewport = window.viewport_size();
        let theme = cx.global::<Theme>();

        anchored().child(
            v_flex_center()
                .h(viewport.height)
                .w(viewport.width)
                .when(self.overlay_visible, |this| {
                    this.occlude().bg(theme.colors.overlay)
                })
                .when(self.overlay_closable, |this| {
                    if (self.layer_ix + 1)
                        != PortalContextProvider::read(window, cx).active_modals.len()
                    {
                        return this;
                    }

                    this.on_mouse_down(MouseButton::Left, {
                        let on_close = on_close.clone();
                        move |_, window, cx| {
                            on_close(&ClickEvent::default(), window, cx);
                            window.close_modal(cx);
                        }
                    })
                })
                .child(
                    div()
                        .border_1()
                        .border_color(theme.colors.border)
                        .bg(theme.colors.popover)
                        .rounded(theme.rounded.lg)
                        .p(rems(2.))
                        .w(rems(50.))
                        .id(("modal", self.layer_ix))
                        .track_focus(&self.focus_handle)
                        .absolute()
                        .occlude()
                        .relative()
                        .when_some(self.title, |this, title| {
                            this.child(
                                div()
                                    .text_color(theme.colors.popover_primary)
                                    .text_size(theme.text.size.lg)
                                    .font_weight(FontWeight::BOLD)
                                    .child(title),
                            )
                        })
                        .when_some(self.description, |this, description| {
                            this.child(
                                div()
                                    .text_color(theme.colors.popover_secondary)
                                    .text_size(theme.text.size.md)
                                    .child(description),
                            )
                        })
                        .when(self.show_close, |this| {
                            this.child(
                                Button::new(("close-modal", self.layer_ix))
                                    .variant(ButtonVariant::Ghost)
                                    .child(
                                        svg()
                                            .size(rems(1.5))
                                            .path("icons/x.svg")
                                            .text_color(theme.colors.popover_secondary),
                                    )
                                    .on_click(move |event, window, cx| {
                                        on_close(event, window, cx);
                                        window.close_modal(cx);
                                    })
                                    .absolute()
                                    .top(rems(1.5))
                                    .right(rems(2.)),
                            )
                        })
                        .child(self.content.mt(rems(2.))),
                ),
        )
    }
}
