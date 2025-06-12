use crate::{Selectable, components::Tab, primitives::h_flex};
use gpui::{prelude::FluentBuilder, *};
use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectTab(usize);

impl_internal_actions!(tab_bar, [SelectTab]);

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct TabBar {
    base: Stateful<Div>,
    children: SmallVec<[Tab; 3]>,
    selected_ix: Option<usize>,
    on_click: Option<Arc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: h_flex().id(id),
            children: SmallVec::new(),
            selected_ix: None,
            on_click: None,
        }
    }

    pub fn selected_index(mut self, ix: usize) -> Self {
        self.selected_ix = Some(ix);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<Tab>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child(mut self, child: impl Into<Tab>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn on_click(mut self, on_click: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Arc::new(on_click));
        self
    }
}

impl Styled for TabBar {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for TabBar {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base
            .group("tab-bar")
            .on_action({
                let on_click = self.on_click.clone();
                move |action: &SelectTab, window: &mut Window, cx: &mut App| {
                    if let Some(on_click) = on_click.clone() {
                        on_click(&action.0, window, cx);
                    }
                }
            })
            .relative()
            .items_center()
            .gap(rems(2.5))
            .children(self.children.into_iter().enumerate().map(|(ix, tab)| {
                tab.id(ix)
                    .when_some(self.selected_ix, |this, selected_ix| {
                        this.selected(selected_ix == ix)
                    })
                    .when_some(self.on_click.clone(), move |this, on_click| {
                        this.on_click(move |_, window, cx| on_click(&ix, window, cx))
                    })
            }))
    }
}
