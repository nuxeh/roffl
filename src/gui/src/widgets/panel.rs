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

//! A widget with draggable resizing on one edge.

use druid::kurbo::{Line, Point, Rect, Size};
use druid::{
    theme, BoxConstraints, Color, Cursor, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, RenderContext, UpdateCtx, Widget, WidgetPod,
};

/// A container containing two other widgets, splitting the area either horizontally or vertically.
pub struct Stretch<T> {
    width_chosen: f64,
    min_size: f64,     // Integers only
    bar_size: f64,     // Integers only
    min_bar_area: f64, // Integers only
    solid: bool,
    draggable: bool,
    child: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T> Stretch<T> {
    /// Create a new split panel, with the specified axis being split in two.
    ///
    /// Horizontal split axis means that the children are left and right.
    /// Vertical split axis means that the children are up and down.
    pub fn new(
        child: impl Widget<T> + 'static,
    ) -> Self {
        Stretch {
            width_chosen: 0.5,
            min_size: 0.0,
            bar_size: 6.0,
            min_bar_area: 6.0,
            solid: false,
            draggable: false,
            child: WidgetPod::new(child).boxed(),
        }
    }

    /// Builder-style method to set the minimum size for both sides of the split axis.
    ///
    /// The value must be greater than or equal to `0.0`.
    /// The value will be rounded up to the nearest integer.
    pub fn min_size(mut self, min_size: f64) -> Self {
        assert!(min_size >= 0.0);
        self.min_size = min_size.ceil();
        self
    }

    /// Builder-style method to set the size of the splitter bar.
    ///
    /// The value must be positive or zero.
    /// The value will be rounded up to the nearest integer.
    /// The default splitter bar size is `6.0`.
    pub fn bar_size(mut self, bar_size: f64) -> Self {
        assert!(bar_size >= 0.0, "bar_size must be 0.0 or greater!");
        self.bar_size = bar_size.ceil();
        self
    }

    /// Builder-style method to set the minimum size of the splitter bar area.
    ///
    /// The minimum splitter bar area defines the minimum size of the area
    /// where mouse hit detection is done for the splitter bar.
    /// The final area is either this or the splitter bar size, whichever is greater.
    ///
    /// This can be useful when you want to use a very narrow visual splitter bar,
    /// but don't want to sacrifice user experience by making it hard to click on.
    ///
    /// The value must be positive or zero.
    /// The value will be rounded up to the nearest integer.
    /// The default minimum splitter bar area is `6.0`.
    pub fn min_bar_area(mut self, min_bar_area: f64) -> Self {
        assert!(min_bar_area >= 0.0, "min_bar_area must be 0.0 or greater!");
        self.min_bar_area = min_bar_area.ceil();
        self
    }

    /// Builder-style method to set whether the split point can be changed by dragging.
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }

    /// Builder-style method to set whether the splitter bar is drawn as a solid rectangle.
    ///
    /// If this is `false` (the default), the bar will be drawn as two parallel lines.
    pub fn solid_bar(mut self, solid: bool) -> Self {
        self.solid = solid;
        self
    }

    /// Returns the size of the splitter bar area.
    #[inline]
    fn bar_area(&self) -> f64 {
        self.bar_size.max(self.min_bar_area)
    }

    /// Returns the padding size added to each side of the splitter bar.
    #[inline]
    fn bar_padding(&self) -> f64 {
        (self.bar_area() - self.bar_size) / 2.0
    }

    /// Returns the location of the edges of the splitter bar area,
    /// given the specified total size.
    fn bar_edges(&self, size: Size) -> (f64, f64) {
        let bar_area = self.bar_area();
        let reduced_width = size.width - bar_area;
        let edge1 = (reduced_width * 0.5).floor();
        let edge2 = edge1 + bar_area;
        (edge1, edge2)
    }

    /// Returns true if the provided mouse position is inside the splitter bar area.
    fn bar_hit_test(&self, size: Size, mouse_pos: Point) -> bool {
        let (edge1, edge2) = self.bar_edges(size);
        mouse_pos.x >= edge1 && mouse_pos.x <= edge2
    }

    /// Set a new chosen split point.
    fn update_size(&mut self, size: Size, mouse_pos: Point) {
        size.width
    }

    /// Returns the color of the splitter bar.
    fn bar_color(&self, env: &Env) -> Color {
        if self.draggable {
            env.get(theme::BORDER_LIGHT)
        } else {
            env.get(theme::BORDER_DARK)
        }
    }

    fn paint_solid_bar(&mut self, ctx: &mut PaintCtx, env: &Env) {
        let size = ctx.size();
        let (edge1, edge2) = self.bar_edges(size);
        let padding = self.bar_padding();
        let rect = {
            Rect::from_points(
                Point::new(edge1 + padding.ceil(), 0.0),
                Point::new(edge2 - padding.floor(), size.height),
            )
        };
        let splitter_color = self.bar_color(env);
        ctx.fill(rect, &splitter_color);
    }

    fn paint_stroked_bar(&mut self, ctx: &mut PaintCtx, env: &Env) {
        let size = ctx.size();
        // Set the line width to a third of the splitter bar size,
        // because we'll paint two equal lines at the edges.
        let line_width = (self.bar_size / 3.0).floor();
        let line_midpoint = line_width / 2.0;
        let (edge1, edge2) = self.bar_edges(size);
        let padding = self.bar_padding();
        let (line1, line2) = {
            (
                Line::new(
                    Point::new(edge1 + line_midpoint + padding.ceil(), 0.0),
                    Point::new(edge1 + line_midpoint + padding.ceil(), size.height),
                ),
                Line::new(
                    Point::new(edge2 - line_midpoint - padding.floor(), 0.0),
                    Point::new(edge2 - line_midpoint - padding.floor(), size.height),
                ),
            )
        };
        let splitter_color = self.bar_color(env);
        ctx.stroke(line1, &splitter_color, line_width);
        ctx.stroke(line2, &splitter_color, line_width);
    }
}

impl<T: Data> Widget<T> for Stretch<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if self.child.is_active() {
            self.child.event(ctx, event, data, env);
            if ctx.is_handled() {
                return;
            }
        }
        match event {
            Event::MouseDown(mouse) => {
                if mouse.button.is_left() && self.bar_hit_test(ctx.size(), mouse.pos) {
                    ctx.set_active(true);
                    ctx.set_handled();
                }
            }
            Event::MouseUp(mouse) => {
                if mouse.button.is_left() && ctx.is_active() {
                    ctx.set_active(false);
                    self.update_size(ctx.size(), mouse.pos);
                    ctx.request_paint();
                }
            }
            Event::MouseMove(mouse) => {
                if ctx.is_active() {
                    self.update_size(ctx.size(), mouse.pos);
                    ctx.request_layout();
                }

                if ctx.is_hot() && self.bar_hit_test(ctx.size(), mouse.pos) || ctx.is_active() {
                    ctx.set_cursor(&Cursor::ResizeLeftRight)
                }
            }
            _ => {}
        }
        if !self.child.is_active() {
            self.child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.child.update(ctx, &data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Stretch");

        let mut my_size = bc.max();
        let bar_area = self.bar_area();
        let reduced_size = Size::new(
            (my_size.width - bar_area).max(0.),
            (my_size.height - bar_area).max(0.),
        );

        let child_size = self.child.layout(ctx, &child_bc, &data, env);

        let child_rect = {
            my_size.height = child_size.height;
            Rect::from_origin_size(
                Point::new(child_size.width + bar_area, 0.0),
                child_size,
            )
        };
        self.child.set_layout_rect(child_rect);

        let insets = self.paint_rect() - Rect::ZERO.with_size(my_size);
        ctx.set_paint_insets(insets);

        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if self.solid {
            self.paint_solid_bar(ctx, env);
        } else {
            self.paint_stroked_bar(ctx, env);
        }
        self.child.paint_with_offset(ctx, &data, env);
    }
}

// Move to std lib clamp as soon as https://github.com/rust-lang/rust/issues/44095 lands
fn clamp(mut x: f64, min: f64, max: f64) -> f64 {
    assert!(min <= max);
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}
