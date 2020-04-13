//! `roffl` main window

use std::sync::Arc;

use druid::lens::{self, LensExt};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct AppData {
    left: Arc<Vec<u32>>,
    right: Arc<Vec<u32>>,
}

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("roffl-window-title").with_placeholder("roffl"));
    // Set our initial data
    let data = AppData {
        left: Arc::new(vec![1, 2]),
        right: Arc::new(vec![1, 2, 3]),
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
        Button::new("Add")
            .on_click(|_, data: &mut AppData, _| {
                // Add child to left list
                let value = data.left.len() + 1;
                Arc::make_mut(&mut data.left).push(value as u32);

                // Add child to right list
                let value = data.right.len() + 1;
                Arc::make_mut(&mut data.right).push(value as u32);
            })
            .fix_height(30.0)
            .expand_width(),
    );

    let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    // Build a simple list
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
        .lens(AppData::left),
        1.0,
    );

    // Build a list with shared data
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
            |d: &AppData| (d.right.clone(), d.right.clone()),
            |d: &mut AppData, x: (Arc<Vec<u32>>, Arc<Vec<u32>>)| {
                // If shared data was changed reflect the changes in our AppData
                d.right = x.0
            },
        )),
        1.0,
    );

    root.add_flex_child(lists, 1.0);
    root
}
