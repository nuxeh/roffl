use super::AppData;

use druid::{
    Widget,
};
use druid::widget::{
    Flex, Label, Button
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
                SvgButton::new(crate::LOGO.parse().unwrap()),
                1.0
            )
            .with_child(
                Label::new("Hi")
            )
    }
}
