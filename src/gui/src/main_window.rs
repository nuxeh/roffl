use std::sync::Arc;

use druid::lens::{self, LensExt};
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, List, Scroll,
    TextBox, Container, Split, SizedBox
};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, UnitPoint, Widget,
    WidgetExt, WindowDesc, MenuDesc, MenuItem, Selector,
};
use super::AppData;
use crate::widgets::panel2::Panel;

pub fn make() -> impl Widget<AppData> {
    let mut root = Flex::row();
    let mut left_panel = Flex::column();
    let mut message_area = Flex::column();
    let mut right_panel = Flex::column();

    let channel_list = Scroll::new(
        List::new(|| {
            Flex::row()
                .with_spacer(10.0)
                .with_flex_child(
                    Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
                        .with_text_size(10.0)
                        .align_vertical(UnitPoint::LEFT)
                        .padding(2.0)
                        .expand()
                        .height(20.0),
                    1.0
                )
        }))
        .vertical()
        .lens(AppData::channels);

    left_panel.add_flex_child(channel_list, 1.0);

    root.add_child(
        SizedBox::new(left_panel)
            .fix_width(200.0)
            .background(Color::rgb(0.25, 0.25, 0.25))
    );

    let messages = Scroll::new(
       List::new(|| {
            Flex::row()
                .with_child(
                    Label::new(|(_, item): &(Arc<Vec<u32>>, u32), _env: &_| {
                        format!("List item #{}", item)
                    })
                    .with_text_size(10.0)
                    .align_vertical(UnitPoint::LEFT),
                )
                .with_flex_spacer(1.0)
                .padding(2.0)
                .fix_height(20.0)
        }))
        .vertical()
        .expand()
        .align_vertical(UnitPoint::BOTTOM)
        .lens(
            lens::Id.map(
                |d: &AppData| (d.messages.clone(), d.messages.clone()),
                |d: &mut AppData, x: (Arc<Vec<u32>>, Arc<Vec<u32>>)| {
                    d.messages = x.0
                },
            )
        );

    message_area.add_flex_child(messages, 1.0);

    let input_box = TextBox::new()
        .padding(0.0)
        .expand_width()
        .align_vertical(UnitPoint::BOTTOM)
        .align_horizontal(UnitPoint::CENTER)
        .lens(AppData::message_text);

    message_area.add_child(input_box);
    message_area.add_spacer(1.0);

    root.add_flex_child(
        SizedBox::new(message_area)
            .expand()
            .background(Color::rgb(0.2, 0.2, 0.2)),
        1.0
    );

    let nick_list = Scroll::new(
        List::new(|| {
            Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
                .with_text_size(10.0)
                .align_vertical(UnitPoint::LEFT)
                .padding(2.0)
                .expand()
                .height(20.0)
                .background(Color::rgb(0.4, 0.4, 0.4))
        }))
        .vertical()
        .lens(AppData::nicks);

    right_panel.add_flex_child(nick_list, 1.0);

    root.add_child(
        SizedBox::new(right_panel)
            .fix_width(200.0)
            .background(Color::rgb(0.25, 0.25, 0.25))
    );

    //root.debug_paint_layout()
    root
}
