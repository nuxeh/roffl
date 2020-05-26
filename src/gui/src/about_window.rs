use super::AppData;

use druid::{
    Widget,
};
use druid::widget::{
    Flex, Svg,
};

pub struct AboutWindow;

impl AboutWindow {
    pub fn make() -> impl Widget<AppData> {
        Svg::new(crate::LOGO.parse().unwrap())
    }
}
