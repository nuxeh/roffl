// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A panel widget

use druid::kurbo::{Point, Rect, Size, RoundedRect};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    UpdateCtx, Widget, WidgetPod, Color, RenderContext, Cursor,
};

/// A widget that switches between two possible child views.
pub struct Panel<T> {
    child: WidgetPod<T, Box<dyn Widget<T>>>,
    left: bool,
    width: f64,
    mouse_down: Point,
    mouse_down_size: f64,
}

impl<T> Panel<T> {
    /// Create a new widget that switches between two views.
    ///
    /// The given closure is evaluated on data change. If its value is `true`, then
    /// the `true_branch` widget is shown, otherwise `false_branch`.
    pub fn new(
        child: impl Widget<T> + 'static,
        left: bool,
        initial_width: f64,
    ) -> Panel<T> {
        Panel {
            child: WidgetPod::new(child).boxed(),
            left,
            width: initial_width,
            mouse_down: Point::ORIGIN,
            mouse_down_size: 0.0,
        }
    }

    /// Create a panel for the left side
    pub fn left(
        child: impl Widget<T> + 'static,
        initial_width: f64,
    ) -> Panel<T> {
        Self::new(child, true, initial_width)
    }

    /// Create a panel for the right side
    pub fn right(
        child: impl Widget<T> + 'static,
        initial_width: f64,
    ) -> Panel<T> {
        Self::new(child, false, initial_width)
    }

    fn resize_hit_test(&self, size: Size, mouse_pos: Point) -> bool {
        if self.left {
            mouse_pos.x < size.width && mouse_pos.x > size.width - 4.0
        } else {
            mouse_pos.x < 4.0
        }
    }

    fn update_width(&mut self, mouse_pos: &Point) {
        let delta = mouse_pos.x - self.mouse_down.x;
        self.width = if self.left {
            self.mouse_down_size + delta
        } else {
            self.mouse_down_size - delta
        };
        println!("w: {}", self.width);
    }
}

impl<T: Data> Widget<T> for Panel<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        // Dispatch event to child
        if self.child.is_active() {
            self.child.event(ctx, event, data, env);
            if ctx.is_handled() {
                return;
            }
        }

        match event {
            Event::MouseDown(mouse) => {
                if mouse.button.is_left() && self.resize_hit_test(ctx.size(), mouse.pos) {
                    self.mouse_down = mouse.pos;
                    self.mouse_down_size = self.width;
                    ctx.set_active(true);
                    ctx.set_handled();
                }
            }
            Event::MouseUp(mouse) => {
                if mouse.button.is_left() && ctx.is_active() {
                    ctx.set_active(false);
                    self.update_width(&mouse.pos);
                    ctx.request_paint();
                }
            }
            Event::MouseMoved(mouse) => {
                if ctx.is_active() {
                    self.update_width(&mouse.pos);
                    ctx.request_layout();
                }

                if ctx.is_hot() && self.resize_hit_test(ctx.size(), mouse.pos) || ctx.is_active() {
                    ctx.set_cursor(&Cursor::ResizeLeftRight)
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        // Restrict child box constraints
        let mut min = bc.min();
        let mut max = bc.max();
        min.width = self.width;
        max.width = self.width;
        let clamped = BoxConstraints::new(min, max);

        let size = self.child.layout(layout_ctx, &clamped, data, env);
        self.child
            .set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
        layout_ctx.set_paint_insets(self.child.paint_insets());
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();
        let mask_size = Size::new(10.0, size.height);

        let layout_rect = RoundedRect::from_origin_size(Point::ORIGIN, size, 10.0);
        let corner_mask = if self.left {
            Rect::from_origin_size(Point::ORIGIN, mask_size)
        } else {
            Rect::from_origin_size(
                Point {
                    x: size.width - 10.0,
                    y: 0.0
                },
                mask_size
            )
        };

        let background_color = Color::rgb(0.5, 0.0, 0.5);

        ctx.fill(layout_rect, &background_color);
        ctx.fill(corner_mask, &background_color);

        self.child.paint(ctx, data, env);
    }
}
