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
    UpdateCtx, Widget, WidgetPod, Color, RenderContext
};

/// A widget that switches between two possible child views.
pub struct Panel<T> {
    child: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T> Panel<T> {
    /// Create a new widget that switches between two views.
    ///
    /// The given closure is evaluated on data change. If its value is `true`, then
    /// the `true_branch` widget is shown, otherwise `false_branch`.
    pub fn new(
        child: impl Widget<T> + 'static,
    ) -> Panel<T> {
        Panel {
            child: WidgetPod::new(child).boxed(),
        }
    }
}

impl<T: Data> Widget<T> for Panel<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
            self.child.event(ctx, event, data, env)
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
        let size = self.child.layout(layout_ctx, bc, data, env);
        self.child
            .set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
        layout_ctx.set_paint_insets(self.child.paint_insets());
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();
        let layout_rect = RoundedRect::from_origin_size(Point::ORIGIN, size, 10.0);

        let background_color = Color::rgb(0.5, 0.0, 0.5);

        ctx.fill(layout_rect, &background_color);
        self.child.paint(ctx, data, env);
    }
}
