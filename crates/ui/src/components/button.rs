use crate::{Selectable, Theme, primitives::h_flex_center};
use gpui::{prelude::FluentBuilder, *};

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Ghost,
}

#[derive(Clone, Copy)]
pub enum ButtonSize {
    Small,
    Medium,
}

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Button {
    base: Div,
    id: ElementId,
    selected: bool,
    children: Vec<AnyElement>,
    variant: ButtonVariant,
    size: ButtonSize,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    stop_propagation: bool,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: h_flex_center(),
            id: id.into(),
            selected: false,
            children: Vec::new(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            on_click: None,
            stop_propagation: true,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn stop_propagation(mut self, val: bool) -> Self {
        self.stop_propagation = val;
        self
    }
}

impl Selectable for Button {
    fn element_id(&self) -> &ElementId {
        &self.id
    }

    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(elements)
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let mut element = match self.size {
            ButtonSize::Small => self
                .base
                .h(rems(2.))
                .text_size(theme.text.size.xs)
                .px(rems(2.)),
            ButtonSize::Medium => self
                .base
                .h(rems(2.5))
                .text_size(theme.text.size.md)
                .px(rems(2.5)),
        };

        element = match self.variant {
            ButtonVariant::Primary => element
                .bg(theme.colors.primary)
                .text_color(theme.colors.primary_foreground),
            ButtonVariant::Ghost => element.border_color(theme.colors.border).px(rems(0.)),
        };

        element
            .id(self.id)
            .cursor_pointer()
            .overflow_hidden()
            .when_some(self.on_click, |this, on_click| {
                let stop_propagation = self.stop_propagation;
                this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    window.prevent_default();
                    if stop_propagation {
                        cx.stop_propagation();
                    }
                })
                .on_click(move |event, window, cx| {
                    (on_click)(event, window, cx);
                })
            })
            .children(self.children)
    }
}
