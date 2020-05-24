// Copyright 2018 The xi-editor Authors.
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

//! A button widget.
use druid::theme;
use druid::widget::prelude::*;
use druid::widget::{
    Click, ControllerHost, Label, LabelText, Svg
};

use druid::{
    Affine, Data, Insets, LinearGradient, Point, Rect, RenderContext, UnitPoint, Widget
};

/// A button with a text label.
pub struct SvgButton<T> {
    data: Option<T>,
    image: Svg,
    active_image: Option<Svg>,
    is_active: bool,
}

impl<T: Data> SvgButton<T> {
    /// Create a new button with an SVG image.
    ///
    /// Use the `.on_click` method to provide a closure to be called when the
    /// button is clicked.
    ///
    /// # Examples
    ///
    /// ```
    /// use druid::widget::Button;
    ///
    /// let button = Button::new("Increment").on_click(|_ctx, data: &mut u32, _env| {
    ///     *data += 1;
    /// });
    /// ```
    pub fn new(image: Svg) -> Self {
        SvgButton {
            data: None,
            image,
            active_image: None,
            is_active: false,
        }
    }

    /// Add an image to render when the button is active
    pub fn with_active_image(mut self, image: Svg) -> Self {
        self.active_image = Some(image);
        self
    }

    /// Provide a closure to be called when this button is clicked.
    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl<T: Data> Widget<T> for SvgButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        //self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        //self.label.update(ctx, old_data, data, env)
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        bc.debug_check("SvgButton");
        let size = self.image.layout(layout_ctx, &bc, data, env);
        if let Some(i) = &mut self.active_image {
            i.layout(layout_ctx, &bc, data, env);
        }
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        /*
        let is_active = ctx.is_active();
        let is_hot = ctx.is_hot();
        let size = ctx.size();

        let rounded_rect = Rect::from_origin_size(Point::ORIGIN, size)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        let bg_gradient = if is_active {
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (env.get(theme::BUTTON_DARK), env.get(theme::BUTTON_LIGHT)),
            )
        } else {
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (env.get(theme::BUTTON_LIGHT), env.get(theme::BUTTON_DARK)),
            )
        };

        let border_color = if is_hot {
            env.get(theme::BORDER_LIGHT)
        } else {
            env.get(theme::BORDER_DARK)
        };

        ctx.stroke(
            rounded_rect,
            &border_color,
            env.get(theme::BUTTON_BORDER_WIDTH),
        );

        ctx.fill(rounded_rect, &bg_gradient);

        let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });
        */
    }
}
