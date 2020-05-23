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

use druid::kurbo::{Point, Rect, Size, RoundedRect};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    UpdateCtx, Widget, WidgetPod, Color, RenderContext, Cursor,
};

/// A widget that overlays two child views.
pub struct Overlay<T> {
    bottom: WidgetPod<T, Box<dyn Widget<T>>>,
    top: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T> Overlay<T> {
    /// Create a new widget that overlays two views.
    pub fn new(
        bottom: impl Widget<T> + 'static,
        top: impl Widget<T> + 'static
    ) -> Overlay<T> {
        Overlay {
            bottom: WidgetPod::new(bottom).boxed(),
            top: WidgetPod::new(top).boxed(),
        }
    }
}

impl<T: Data> Widget<T> for Overlay<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.top.event(ctx, event, data, env);
        if ctx.is_handled() {
            return;
        }
        self.bottom.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.top.lifecycle(ctx, event, data, env);
        self.bottom.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        let size_bottom = self.bottom.layout(ctx, bc, data, env);
        let _ = self.top.layout(ctx, bc, data, env);

        let rect = Rect::from_origin_size(Point::ORIGIN, size_bottom);
        self.bottom.set_layout_rect(ctx, data, env, rect);
        self.top.set_layout_rect(ctx, data, env, rect);

        size_bottom
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.bottom.paint(ctx, data, env);
        self.top.paint(ctx, data, env);
    }
}
