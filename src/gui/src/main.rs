//! `roffl` main window

use std::sync::Arc;

use druid::lens::{self, LensExt};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll, TextBox, Padding, SizedBox};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc, MenuDesc, MenuItem, Selector,
};

#[derive(Clone, Data, Lens)]
struct AppData {
    channels: Arc<Vec<u32>>,
    messages: Arc<Vec<u32>>,
    nicks: Arc<Vec<u32>>,
}

fn main() {
    // Make the window
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("roffl-window-title").with_placeholder("roffl"))
        .menu(make_menu());

    // Set our initial data
    let data = AppData {
        channels: Arc::new(vec![1, 2]),
        messages: Arc::new(vec![1, 2, 3]),
        nicks: Arc::new(vec![1, 2]),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<AppData> {
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

    // Build the message list with shared data
    lists.add_flex_child(
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
                .background(Color::rgb(0.5, 0.0, 0.5))
                .fix_height(20.0)
        }))
        .vertical()
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

    // Add lists
    root.add_flex_child(lists, 0.0);

    let textbox_msg = TextBox::new();

    let send_bar = Flex::row()
        .with_child(Padding::new(5.0, textbox_msg))
            .fix_height(30.0)
            .expand_width();

    //root.add_child(send_bar);

    // Spacer to keep footer at the bottom
    root.add_flex_child(SizedBox::empty().expand(), 1.0);

    root.add_child(
        Button::new("Send")
            .on_click(|_, data: &mut AppData, _| {
            })
            .fix_height(30.0)
            .expand_width(),
    );

    root
}

const MENU_COLOURS_ACTION: Selector = Selector::new("menu-colours-action");
const MENU_MESSAGING_ACTION: Selector = Selector::new("menu-messaging-action");
const MENU_CONNECT_ACTION: Selector = Selector::new("menu-server-action");

fn make_menu<T: Data>() -> MenuDesc<T> {
    let edit_menu = MenuDesc::new(LocalizedString::new("common-menu-edit-menu"))
        .append(druid::platform_menus::common::cut())
        .append(druid::platform_menus::common::copy())
        .append(druid::platform_menus::common::paste());

    let settings_menu = MenuDesc::new(LocalizedString::new("Settings"))
        .append(MenuItem::new(
            LocalizedString::new("Colours"),
            MENU_COLOURS_ACTION
        ))
        .append(MenuItem::new(
            LocalizedString::new("Messaging"),
            MENU_MESSAGING_ACTION
        ));

    let server_menu = MenuDesc::new(LocalizedString::new("Server"))
        .append(MenuItem::new(
            LocalizedString::new("Connect..."),
            MENU_CONNECT_ACTION
        ));

    MenuDesc::platform_default()
        .unwrap_or(MenuDesc::empty())
        .append(server_menu)
        .append(edit_menu)
        .append(settings_menu)
}
