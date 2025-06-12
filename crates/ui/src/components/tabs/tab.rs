use crate::{Selectable, Theme, primitives::h_flex};
use gpui::{prelude::FluentBuilder, *};
use std::sync::Arc;

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Tab {
    id: ElementId,
    base: Div,
    label: SharedString,
    children: Vec<AnyElement>,
    pub(super) selected: bool,
    on_click: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Tab {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            id: ElementId::Integer(0),
            base: h_flex(),
            label: label.into(),
            children: Vec::new(),
            selected: false,
            on_click: None,
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Arc::new(on_click));
        self
    }
}

impl ParentElement for Tab {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Selectable for Tab {
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

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Tab {}

impl Styled for Tab {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Tab {
    fn render(self, _window: &mut Window, app: &mut App) -> impl IntoElement {
        let theme = app.global::<Theme>();

        self.base
            .id(self.id)
            .gap(rems(0.5))
            .cursor_pointer()
            .items_center()
            .flex_shrink_0()
            .child(self.label)
            .when_else(
                self.selected,
                |div| div.text_color(theme.colors.primary),
                |div| div.text_color(theme.colors.secondary),
            )
            .when_some(self.on_click.clone(), |this, on_click| {
                this.on_click(move |event, window, cx| on_click(event, window, cx))
            })
            .children(self.children)
    }
}
