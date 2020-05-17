use std::sync::Arc;

use druid::lens::{self, LensExt};
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, List, Scroll,
    TextBox, Container, Split,
};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, UnitPoint, Widget,
    WidgetExt, WindowDesc, MenuDesc, MenuItem, Selector,
};
use super::AppData;
use crate::widgets::panel2::Panel;

pub fn make() -> impl Widget<AppData> {
    let mut root = Flex::column();

    // Build a button to add children to both lists
    root.add_child(
        Label::new("Channel name")
            .padding(4.0)
            .expand_width(),
    );

    let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    // Build the channel list
    lists.add_flex_child(
        Scroll::new(List::new(|| {
            Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
                .with_text_size(10.0)
                .align_vertical(UnitPoint::LEFT)
                .padding(2.0)
                .expand()
                .height(20.0)
                .background(Color::rgb(0.5, 0.5, 0.5))
        }))
        .vertical()
        .lens(AppData::channels),
        1.0,
    );


    // Build the nick list
    lists.add_flex_child(
        Scroll::new(List::new(|| {
            Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
                .with_text_size(10.0)
                .align_vertical(UnitPoint::LEFT)
                .padding(2.0)
                .expand()
                .height(20.0)
                .background(Color::rgb(0.5, 0.5, 0.5))
        }))
        .vertical()
        .lens(AppData::nicks),
        1.0,
    );

    // Construct the footer
    let mut footer = Flex::row();
    footer.add_flex_child(
        TextBox::new()
            .padding(2.0)
            .expand()
            .align_vertical(UnitPoint::BOTTOM)
            .lens(AppData::message_text),
            1.0
    );
    footer.add_child(
        Button::new("Send")
            .padding(2.0)
            .align_vertical(UnitPoint::BOTTOM)
    );

    // Add panel layout
    let mut left_panel_flex = Flex::column();

    let channels_list = Scroll::new(List::new(|| {
        Flex::row()
            .with_spacer(10.0)
            .with_flex_child(
                Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
                    .with_text_size(10.0)
                    .align_vertical(UnitPoint::LEFT)
                    .padding(2.0)
                    .expand()
                    .height(20.0),
                    //.background(Color::rgb(0.5, 0.5, 0.5)),
                    1.0
            )
    }))
    .vertical()
    .lens(AppData::channels);

    /*
    left_panel_flex.add_flex_child(
        Label::new("Channels").padding(10.0),
        0.0
    );
    */
    left_panel_flex.add_flex_child(channels_list, 1.0);
    left_panel_flex.add_spacer(10.0);

    let left_panel = Panel::left(left_panel_flex, 150.0)
        .expand_height();

    let right_panel = Panel::right(lists, 250.0)
        .expand_height();

    // Build mid section
    let mut midsection = Flex::row();
    midsection.add_child(left_panel);

    // Add message list
    // Build the message list with shared data
    midsection.add_flex_child(
        Scroll::new(List::new(|| {
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
                //.background(Color::rgb(0.5, 0.0, 0.5))
                .fix_height(20.0)
        }))
        .vertical()
        .align_vertical(UnitPoint::BOTTOM)
        .expand_width()
        .lens(lens::Id.map(
            // Expose shared data with children data
            |d: &AppData| (d.messages.clone(), d.messages.clone()),
            |d: &mut AppData, x: (Arc<Vec<u32>>, Arc<Vec<u32>>)| {
                // If shared data was changed reflect the changes in our AppData
                d.messages = x.0
            },
        )),
        1.0,
    );

    midsection.add_child(right_panel);

    root.add_flex_child(midsection, 1.0);
    root.add_spacer(2.0);

    root.add_child(
        footer
            .fix_height(30.0)
            .padding(2.0)
            .expand_width(),
    );

    //root.debug_paint_layout()
    root
}
