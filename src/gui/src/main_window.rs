use std::sync::Arc;

use druid::lens::{self, LensExt};
use druid::widget::{
    Flex, Label, List, Scroll, SizedBox, TextBox, Svg, SvgData, CrossAxisAlignment
};
use druid::{
    Color, UnitPoint, Widget, WidgetExt, Target, commands
};
use super::AppData;
use crate::widgets::{
    borderless_textbox::BorderlessText,
    overlay::Overlay,
    svg_button::SvgButton,
};
use crate::assets::{
    ICON_ADD, ICON_SEND, ICON_LIST, ICON_COG, ICON_LEFT_PANEL, ICON_RIGHT_PANEL, ICON_SEARCH
};

pub struct MainWindow;

impl MainWindow {
    pub fn make() -> impl Widget<AppData> {
        let mut root = Flex::row();
        let mut left_panel_base = Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start);
        let mut message_area = Flex::column();
        let mut right_panel_base = Flex::column();

        // Include, and generate derived, assets
        let send = ICON_SEND.parse::<SvgData>().unwrap();
        let send_active = ICON_SEND
            .replace("#fff", "#f6ae2d")
            .parse::<SvgData>().unwrap();

        let search = ICON_SEARCH.parse::<SvgData>().unwrap();
        let search_active = ICON_SEARCH
            .replace("#fff", "#f6ae2d")
            .parse::<SvgData>().unwrap();

        let list = ICON_LIST
            .replace("#fff", "#191919")
            .parse::<SvgData>().unwrap();

        let plus = ICON_ADD.parse::<SvgData>().unwrap();
        let plus_active = ICON_ADD
            .replace("#fff", "#f6ae2d")
            .parse::<SvgData>().unwrap();

        let cog = ICON_COG.parse::<SvgData>().unwrap();
        let cog_active = ICON_COG
            .replace("#fff", "#f6ae2d")
            .parse::<SvgData>().unwrap();

        let leftpanel = ICON_LEFT_PANEL.parse::<SvgData>().unwrap();
        let leftpanel_active = ICON_LEFT_PANEL
            .replace("#fff", "#f6ae2d")
            .parse::<SvgData>().unwrap();

        let rightpanel = ICON_RIGHT_PANEL.parse::<SvgData>().unwrap();
        let rightpanel_active = ICON_RIGHT_PANEL
            .replace("#fff", "#f6ae2d")
            .parse::<SvgData>().unwrap();

        // Logo
        left_panel_base.add_child(
            SizedBox::new(
                SvgButton::new(crate::LOGO.parse().unwrap())
                    .on_click(|ctx, _data , _env| {
                        println!("hi");
                        ctx.submit_command(commands::SHOW_ABOUT, Target::Global);
                    })
                    .align_horizontal(UnitPoint::LEFT)
            )
            .align_horizontal(UnitPoint::LEFT)
            .padding(0.0)
            .fix_height(50.0)
            .background(Color::rgb(0.078, 0.212, 0.259))
        );

        let add_button = SvgButton::new(plus)
            .with_active_image(plus_active)
            .padding(4.0)
            .fix_width(20.0)
            .fix_height(20.0);

        let settings_button = SvgButton::new(cog)
            .with_active_image(cog_active)
            .padding(4.0)
            .fix_width(20.0)
            .fix_height(20.0);

        let leftpanel_button = SvgButton::new(leftpanel)
            .with_active_image(leftpanel_active)
            .padding(4.0)
            .fix_width(20.0)
            .fix_height(20.0);

        let rightpanel_button = SvgButton::new(rightpanel)
            .with_active_image(rightpanel_active)
            .padding(4.0)
            .fix_width(20.0)
            .fix_height(20.0);

        left_panel_base.add_child(
            SizedBox::new(
                Flex::row()
                    .with_child(add_button)
                    .with_child(settings_button)
                    .with_flex_spacer(1.0)
                    .with_child(leftpanel_button)
                    .with_child(rightpanel_button)
            )
        );

        // Channel list
        let channel_list = Scroll::new(
            List::new(|| {
                SizedBox::new(
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
                            Label::new("10")
                                .with_text_size(10.0)
                                .with_text_color(Color::rgb(0.3, 0.3, 0.3))
                                .background(Color::rgb(0.4, 0.4, 0.4)),
                        )
                        .with_child(
                            SizedBox::empty()
                                .fix_height(20.0)
                                .fix_width(20.0)
                                .background(Color::rgb(0.965, 0.682, 0.176))
                        )
                        .with_child(
                            SizedBox::empty()
                                .fix_height(20.0)
                                .fix_width(20.0)
                                .background(Color::rgb(0.129, 0.514, 0.502)) // #218380
                        )
                )
                .background(Color::rgb(0.4, 0.4, 0.4))
            }))
            .vertical()
            .expand_height()
            .lens(AppData::channels);

        left_panel_base.add_flex_child(channel_list, 1.0);

        root.add_child(
            SizedBox::new(left_panel_base)
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

        root.add_child(
            SizedBox::new(right_panel_base)
                .fix_width(200.0)
                .background(Color::rgb(0.25, 0.25, 0.25))
        );

        //root.debug_paint_layout()
        root
    }
}
