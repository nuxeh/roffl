use std::sync::Arc;

use druid::lens::{self, LensExt};
use druid::widget::{
    Flex, Label, List, Scroll, SizedBox, TextBox, Svg, SvgData, CrossAxisAlignment
};
use druid::{
    Color, UnitPoint, Widget, WidgetExt
};
use super::AppData;
use crate::widgets::{
    borderless_textbox::BorderlessText,
    overlay::Overlay,
    svg_button::SvgButton,
};

pub fn make() -> impl Widget<AppData> {
    let mut root = Flex::row();
    let mut left_panel_base = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start);
    let mut left_panel_overlay = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start);
    let mut message_area = Flex::column();
    let mut right_panel_base = Flex::column();
    let mut right_panel_overlay = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End);

    // Assets
    let logo = include_str!("../../../rc.svg").parse::<SvgData>().unwrap();

    let send_base = include_str!("../assets/send.svg");
    let send = send_base.parse::<SvgData>().unwrap();
    let send_active = send_base
        .replace("#fff", "#f08c14")
        .parse::<SvgData>().unwrap();

    let search_base = include_str!("../assets/search.svg");
    let search = search_base.parse::<SvgData>().unwrap();
    let search_active = search_base
        .replace("#fff", "#f08c14")
        .parse::<SvgData>().unwrap();

    let list = include_str!("../assets/list.svg")
        .replace("#fff", "#191919")
        .parse::<SvgData>().unwrap();

    let plus_base = include_str!("../assets/plus.svg");
    let plus = plus_base.parse::<SvgData>().unwrap();
    let plus_active = plus_base
        .replace("#fff", "#f08c14")
        .parse::<SvgData>().unwrap();

    // Logo
    left_panel_base.add_child(
        SizedBox::new(
            Svg::new(logo.clone())
                .align_horizontal(UnitPoint::LEFT)
        )
        .align_horizontal(UnitPoint::LEFT)
        .padding(0.0)
        .fix_height(50.0)
        .background(Color::rgb(0.078, 0.212, 0.259))
    );

    left_panel_base.add_child(
        SizedBox::empty()
            .fix_height(1.0)
    );

    let add_button = SvgButton::new(plus)
        .with_active_image(plus_active)
        .padding(4.0)
        .fix_width(20.0)
        .fix_height(20.0);

    left_panel_base.add_child(
        SizedBox::new(
            Flex::row()
                .with_child(add_button)
        )
    );

    // Channel list
    let channel_list = Scroll::new(
        List::new(|| {
            Flex::row()
                .with_flex_child(
                    Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
                        .with_text_size(10.0)
                        .align_vertical(UnitPoint::LEFT)
                        .padding(2.0)
                        .expand_width()
                        .height(20.0)
                        .background(Color::rgb(0.4, 0.4, 0.4)),
                    1.0
                )
                .with_child(
                    SizedBox::new(
                        Label::new("1000")
                            .with_text_size(10.0)
                            .with_text_color(Color::rgb(0.25, 0.25, 0.25))
                            .center()
                            .background(Color::rgb(0.965, 0.682, 0.176))
                            //.rounded(4.0)
                            //.padding(0.0)
                    )
                    .height(20.0)
                    .background(Color::rgb(0.4, 0.4, 0.4))
                )
        }))
        .vertical()
        .expand_height()
        .lens(AppData::channels);

    left_panel_base.add_flex_child(channel_list, 1.0);

    left_panel_overlay
        .add_flex_child(
            SizedBox::empty().expand_height(),
            1.0
        );

    let list_button = Svg::new(list.clone())
        .padding(2.0)
        .fix_width(16.0)
        .fix_height(16.0)
        .background(Color::rgba(0.25, 0.25, 0.25, 0.5));

    left_panel_overlay.add_child(
        SizedBox::new(
            Flex::row()
                .with_child(list_button)
        )
    );

    let left_panel = Overlay::new(left_panel_base, left_panel_overlay);

    root.add_child(
        SizedBox::new(left_panel)
            .fix_width(200.0)
            .background(Color::rgb(0.25, 0.25, 0.25))
    );

    // Message area and input box
    let messages = Scroll::new(
        Flex::row()
            .with_child(
                List::new(|| {
                    Label::new("09:45")
                    .with_text_size(10.0)
                    .align_vertical(UnitPoint::LEFT)
                    .padding(2.0)
                    .fix_height(20.0)
                    .background(Color::rgb(0.1, 0.1, 0.1))
                })
            )
            .with_child(
                List::new(|| {
                    Label::new(|(_, item): &(Arc<Vec<u32>>, u32), _env: &_| {
                        if item == &2 {
                            format!("verylongusername{}", item)
                       } else {
                            format!("user{}", item)
                       }
                    })
                    .with_text_size(10.0)
                    .padding(2.0)
                    .fix_height(20.0)
                    .align_vertical(UnitPoint::CENTER)
                    .background(Color::rgb(0.2, 0.2, 0.2))
                })
            )
            .with_flex_child(
                List::new(|| {
                    Label::new(|(_, item): &(Arc<Vec<u32>>, u32), _env: &_| {
                        if item != &4 {
                            format!("hi, this is a message #{}", item)
                       } else {
                            format!("hi, this is a message #{}, split over multiple lines, split over multiple lines, split over multiple lines, hopefully", item)
                       }
                    })
                    .with_text_size(10.0)
                    .padding(2.0)
                    .align_vertical(UnitPoint::CENTER)
                    .align_horizontal(UnitPoint::LEFT)
                    .expand_width()
                    .height(20.0)
                    .background(Color::rgb(0.15, 0.15, 0.15))
                }),
                1.1
            )
        )
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
        .with_border(false)
        .with_background(false)
        .padding(1.0)
        .expand_width()
        .align_vertical(UnitPoint::BOTTOM)
        .align_horizontal(UnitPoint::CENTER)
        .env_scope(|env, _| {
            env.set(druid::theme::SELECTION_COLOR, Color::rgb(0.4, 0.4, 0.4));
            env.set(druid::theme::BACKGROUND_LIGHT, Color::rgba8(0, 0, 0, 0));
        })
        .lens(AppData::message_text);

    let send_button = SvgButton::new(send)
        .with_active_image(send_active)
        .padding(4.0)
        .fix_width(20.0)
        .fix_height(20.0);
    let search_button = SvgButton::new(search)
        .with_active_image(search_active)
        .padding(4.0)
        .fix_width(20.0)
        .fix_height(20.0);

    message_area.add_child(
        SizedBox::new(
            Flex::row()
                .with_flex_child(input_box, 1.0)
                .with_child(send_button)
                .with_child(search_button)
            .background(Color::rgb(0.11, 0.11, 0.11))
        )
    );

    root.add_flex_child(
        SizedBox::new(message_area)
            .expand()
            .background(Color::rgb(0.2, 0.2, 0.2)),
        1.0
    );

    // Nick list and search results panel
    let nick_list = Scroll::new(
        List::new(|| {
            Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
                .with_text_size(10.0)
                .align_vertical(UnitPoint::LEFT)
                .padding(2.0)
                .expand_width()
                .height(20.0)
                .background(Color::rgb(0.4, 0.4, 0.4))
        }))
        .vertical()
        .expand_height() // makes a sized box?!
        .lens(AppData::nicks);

    right_panel_base.add_flex_child(nick_list, 1.0);

    let list_button = Svg::new(list.clone())
        .padding(2.0)
        .fix_width(16.0)
        .fix_height(16.0)
        .background(Color::rgba(0.25, 0.25, 0.25, 0.5));

    right_panel_overlay
        .add_flex_child(
            SizedBox::empty().expand_height(),
            1.0
        );

    right_panel_overlay.add_child(
        SizedBox::new(
            Flex::row()
                .with_child(list_button)
        )
    );

    let right_panel = Overlay::new(right_panel_base, right_panel_overlay);

    root.add_child(
        SizedBox::new(right_panel)
            .fix_width(200.0)
            .background(Color::rgb(0.25, 0.25, 0.25))
    );

    //root.debug_paint_layout()
    root
}
