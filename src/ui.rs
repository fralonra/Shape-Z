pub mod widgets;
pub mod rect;
pub mod context;

pub mod prelude {
    pub use crate::ui::widgets::*;
    pub use crate::ui::rect::Rect;
    pub use crate::ui::context::*;

    pub use crate::ui::widgets::text_button::*;
    pub use crate::ui::widgets::tile_editor::*;
}

#[repr(usize)]
enum WidgetIndices {
    TileEditorIndex,
}

use WidgetIndices::*;

pub use crate::prelude::*;

const BROWSER_HEIGHT : usize = 120;

pub struct UI {
    widgets                 : Vec<Box<dyn Widget>>,
}

impl UI {

    pub fn new() -> Self {

        let mut widgets : Vec<Box<dyn Widget>> = vec![];

        let tile_editor = Box::new(TileEditor::new());
        widgets.push(tile_editor);

        // let modebar = Box::new(ModeBar::new());
        // let browser = Box::new(Browser::new());
        // let perspective = Box::new(PerspectiveBar::new());
        // let property = Box::new(PropertyWidget::new());

        // widgets.push(modebar);
        // widgets.push(browser);
        // widgets.push(perspective);
        // widgets.push(property);

        Self {
            widgets
        }
    }

    pub fn draw(&mut self, pixels: &mut [u8], context: &mut Context, world: &World, ctx: &TheContext) {

        self.widgets[TileEditorIndex as usize].set_visible(context.curr_key.is_some());

        /*
        self.widgets[3].set_visible(context.selected_pos.is_some() && context.selected_id.is_some());

        self.widgets[0].set_rect(Rect::new(40, 100, 50, 240));
        self.widgets[1].set_rect(Rect::new(0, (context.height - BROWSER_HEIGHT) as u32, context.width as u32, BROWSER_HEIGHT as u32));
        self.widgets[2].set_rect(Rect::new((context.width - 200) as u32, 20, 150, 40));
        self.widgets[3].set_rect(Rect::new((context.width - 260) as u32, 100, 250, 400));
        */

        self.widgets[TileEditorIndex as usize].set_rect(Rect::new((context.width - 260) as u32, 100, 250, 400));

        for w in &mut self.widgets {
            w.draw(pixels, context, world, ctx);
        }
    }

    pub fn contains(&mut self, x: f32, y: f32) -> bool {
        for w in &mut self.widgets {
            if w.contains(x, y) {
                return true;
            }
        }
        false
    }

    pub fn touch_down(&mut self, button: i32, x: f32, y: f32, context: &mut Context) -> bool {

        for w in &mut self.widgets {
            if w.touch_down(button, x, y, context) {
                return true;
            }
        }

        false
    }

    pub fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        for w in &mut self.widgets {
            if w.touch_dragged(x, y, context) {
                return true;
            }
        }

        false
    }

    pub fn touch_up(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        for w in &mut self.widgets {
            if w.touch_up(x, y, context) {
                return true;
            }
        }

        false
    }

    pub fn update(&mut self, context: &mut Context) {
        for w in &mut self.widgets {
            w.update(context);
        }
    }

}