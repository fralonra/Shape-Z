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

    widgets                         : Vec<Box<dyn Widget>>,

    pub tile_editor_width           : usize,
    tile_editor_drag_rect           : Rect,
    pub tile_editor_drag_start      : Option<(usize, usize)>,
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
            widgets,

            tile_editor_width       : 300,
            tile_editor_drag_rect   : Rect::empty(),
            tile_editor_drag_start  : None,

        }
    }

    pub fn draw(&mut self, pixels: &mut [u8], context: &mut Context, world: &World, ctx: &TheContext) {

        //self.widgets[TileEditorIndex as usize].set_visible(context.curr_key.is_some());

        /*
        self.widgets[3].set_visible(context.selected_pos.is_some() && context.selected_id.is_some());

        self.widgets[0].set_rect(Rect::new(40, 100, 50, 240));
        self.widgets[1].set_rect(Rect::new(0, (context.height - BROWSER_HEIGHT) as u32, context.width as u32, BROWSER_HEIGHT as u32));
        self.widgets[2].set_rect(Rect::new((context.width - 200) as u32, 20, 150, 40));
        self.widgets[3].set_rect(Rect::new((context.width - 260) as u32, 100, 250, 400));
        */

        // Tile editor rects

        let mut tile_editor_rect = Rect::new((context.width - self.tile_editor_width), 0, self.tile_editor_width, context.height);

        self.widgets[TileEditorIndex as usize].set_rect(tile_editor_rect.clone());
        tile_editor_rect.x -= 20;
        tile_editor_rect.width = 15;
        tile_editor_rect.height = 100;
        tile_editor_rect.y = (context.height - 50) / 2;
        self.tile_editor_drag_rect = tile_editor_rect;

        let drag_color = if self.tile_editor_drag_start.is_some() { [255, 255, 255, 255] } else { [0, 0, 0, 255] };

        ctx.draw.rounded_rect(pixels, &self.tile_editor_drag_rect.to_usize(), ctx.width, &drag_color, &(8.0, 8.0, 8.0, 8.0));

        // ---

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

    pub fn touch_down(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        if self.tile_editor_drag_rect.is_inside((x as usize, y as usize)) {
            let offset = x as usize - (context.width - self.tile_editor_width as usize);
            self.tile_editor_drag_start = Some((offset, self.tile_editor_width as usize));
            return true;
        }

        for w in &mut self.widgets {
            if w.touch_down(x, y, context) {
                return true;
            }
        }

        false
    }

    pub fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        if let Some(tile_editor_drag) = &self.tile_editor_drag_start {

            if context.width > x as usize {
                let mut new_width = (context.width - x as usize) +  tile_editor_drag.0;

                if new_width < 300 { new_width = 300; }
                else
                if new_width > context.height - 20 {
                    new_width = context.height - 20;
                } else
                if new_width > context.width - 200 {
                    new_width = context.width - 200;
                }

                self.tile_editor_width = new_width + tile_editor_drag.0;
                return true;
            }
        }

        for w in &mut self.widgets {
            if w.touch_dragged(x, y, context) {
                return true;
            }
        }

        false
    }

    pub fn touch_up(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        let mut consumed = false;

        if self.tile_editor_drag_start.is_some() {
            self.tile_editor_drag_start = None;
            consumed = true;
        }

        for w in &mut self.widgets {
            if w.touch_up(x, y, context) {
                consumed = true;
            }
        }

        consumed
    }

    pub fn update(&mut self, context: &mut Context) {
        for w in &mut self.widgets {
            w.update(context);
        }
    }

}