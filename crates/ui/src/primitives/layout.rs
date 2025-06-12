use gpui::*;

pub fn h_flex() -> Div {
    div().flex().flex_row()
}

pub fn v_flex() -> Div {
    div().flex().flex_col()
}

pub fn h_flex_center() -> Div {
    h_flex().justify_center().items_center()
}

pub fn v_flex_center() -> Div {
    v_flex().justify_center().items_center()
}
