use druid::widget::TextBox;
use druid::{
    Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx
};
//use druid::theme;
use druid::kurbo::Size;

pub struct BorderlessText(TextBox);

impl BorderlessText {
    pub fn new() -> BorderlessText {
        BorderlessText(TextBox::new())
    }
}

impl Widget<String> for BorderlessText {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        self.0.event(ctx, event, data, env)
    }
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &String, _env: &Env) {
        self.0.lifecycle(ctx, event, _data, _env)
    }
    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {
        self.0.update(ctx, _old_data, _data, _env)
    }
    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &String, env: &Env) -> Size {
        self.0.layout(_layout_ctx, bc, _data, env)
    }
    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        self.0.paint(ctx, data, env)
    }
}

