use super::AppData;

use druid::{
    Widget,
};
use druid::widget::{
    Flex, Label, Button
};
use druid::{
    UnitPoint, WidgetExt
};
// TODO clean up imports
use crate::widgets::{
    svg_button::SvgButton
};

pub struct AboutWindow;

impl AboutWindow {
    pub fn make() -> impl Widget<AppData> {
        Flex::column()
            .with_flex_child(
                SvgButton::new(crate::LOGO.parse().unwrap())
                    .align_horizontal(UnitPoint::TOP),
                0.75
            )
            .with_child(
                Label::new("Hi")
            )
            .debug_paint_layout()
    }
}
