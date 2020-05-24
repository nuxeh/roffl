use super::AppData;
use druid::{
    Widget,
};
use druid::widget::{
    SizedBox,
};

pub struct AboutWindow;

impl AboutWindow {
    pub fn make() -> impl Widget<AppData> {
        SizedBox::empty()
    }
}
