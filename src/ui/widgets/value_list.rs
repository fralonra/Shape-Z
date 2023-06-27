
use crate::prelude::*;

pub struct ValueList {
    rect                : Rect,

    values              : Vec<Value>,
    widgets             : Vec<Box<dyn Widget>>,

    drag_widget_index   : Option<usize>,
}

impl Widget for ValueList {

    fn new() -> Self {

        Self {
            rect                : Rect::empty(),

            values              : vec![],
            widgets             : vec![],
            drag_widget_index   : None,
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn set_value_list(&mut self, values: Vec<Value>) {
        let mut widgets = vec![];
        for value in &values {
            let w = value.create_widget();
            widgets.push(w);
        }
        self.values = values;
        self.widgets = widgets;
    }


    fn draw(&mut self, pixels: &mut [u8], stride: usize, context: &mut Context, world: &World, ctx: &TheContext) {
        let mut rect = self.rect.clone();
        rect.height = 24;
        for w in &mut self.widgets {
            w.set_rect(rect);
            w.draw(pixels, stride, context, world, ctx);
            rect.y += rect.height + 8;
        }
    }

    fn contains(&mut self, x: f32, y: f32) -> bool {
        if self.rect.is_inside((x as usize, y as usize)) {
            true
        } else {
            false
        }
    }

    fn touch_down(&mut self, x: f32, y: f32, context: &mut Context, world: &World) -> bool {

        self.drag_widget_index = None;
        if self.rect.is_inside((x as usize, y as usize)) {
            for (index, w) in self.widgets.iter_mut().enumerate() {
                if w.touch_down(x, y, context, world) {
                    self.drag_widget_index = Some(index);
                    return true;
                }
            }
        }

        false
    }

    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        if let Some(drag_widget_index) = self.drag_widget_index {
            if self.widgets[drag_widget_index].touch_dragged(x, y, context) {
                return true;
            }
        }

        true
    }

    fn touch_up(&mut self, _x: f32, _y: f32, _context: &mut Context) -> bool {
        self.drag_widget_index = None;
        false
    }

    fn generate_value_list(&self) -> Vec<Value> {
        let mut list = vec![];
        for w in &self.widgets {
            list.push(w.generate_value());
        }
        list
    }
}