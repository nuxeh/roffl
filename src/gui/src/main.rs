//! `rcchat` main window

mod widgets;
//use widgets::panel2::Panel;

mod main_window;
use main_window::make;

use std::sync::Arc;

use druid::{
    Data, Lens, WindowDesc, LocalizedString, AppLauncher, Selector, MenuDesc, MenuItem
};

#[derive(Clone, Data, Lens)]
struct AppData {
    channels: Arc<Vec<u32>>,
    messages: Arc<Vec<u32>>,
    nicks: Arc<Vec<u32>>,
    message_text: String,
}

#[derive(Clone, Data, Lens)]
struct AppState {
}

fn main() {
    // Make the window
    let main_window = WindowDesc::new(make)
        .title(LocalizedString::new("rc").with_placeholder("rc"));
        //.menu(make_menu());

    // Set our initial data
    let data = AppData {
        channels: Arc::new(vec![1, 2, 3, 1,1,2,3,2,1,2,2,3,1,2,2,3,1,2,2,3,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8]),
        messages: Arc::new(vec![1, 2, 3]),
        nicks: Arc::new(vec![1, 2]),
        message_text: String::from(""),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

const MENU_COLOURS_ACTION: Selector = Selector::new("menu-colours-action");
const MENU_MESSAGING_ACTION: Selector = Selector::new("menu-messaging-action");
const MENU_CONNECT_ACTION: Selector = Selector::new("menu-server-action");
const MENU_VIEW_RBAR_ACTION: Selector = Selector::new("menu-right-bar-action");
const MENU_SEARCH_ACTION: Selector = Selector::new("menu-search-action");

fn make_menu<T: Data>() -> MenuDesc<T> {
    let edit_menu = MenuDesc::new(LocalizedString::new("common-menu-edit-menu"))
        .append(druid::platform_menus::common::cut())
        .append(druid::platform_menus::common::copy())
        .append(druid::platform_menus::common::paste());

    let settings_menu = MenuDesc::new(LocalizedString::new("Settings"))
        .append(MenuItem::new(
            LocalizedString::new("Colours..."),
            MENU_COLOURS_ACTION
        ))
        .append(MenuItem::new(
            LocalizedString::new("Messaging..."),
            MENU_MESSAGING_ACTION
        ))
        .append(MenuItem::new(
            LocalizedString::new("About..."),
            MENU_MESSAGING_ACTION
        ));

    let view_menu = MenuDesc::new(LocalizedString::new("View"))
        .append(MenuItem::new(
            LocalizedString::new("Toggle right panel"),
            MENU_VIEW_RBAR_ACTION
        ))
        .append(MenuItem::new(
            LocalizedString::new("Toggle left panel"),
            MENU_VIEW_RBAR_ACTION
        ));

    let search_menu = MenuDesc::new(LocalizedString::new("Search"))
        .append(MenuItem::new(
            LocalizedString::new("Search..."),
            MENU_SEARCH_ACTION
        ))
        .append(MenuItem::new(
            LocalizedString::new("Build database"),
            MENU_SEARCH_ACTION
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
        .append(view_menu)
        .append(search_menu)
        .append(settings_menu)
}
