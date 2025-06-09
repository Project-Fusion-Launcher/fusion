use gpui::*;

#[derive(IntoElement)]
pub struct Button {
    pub base: Div,
    id: ElementId,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().flex_shrink_0(),
            id: id.into(),
        }
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.id(self.id)
    }
}
