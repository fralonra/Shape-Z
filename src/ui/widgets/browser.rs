use crate::prelude::*;


pub struct Browser {
    rect                : Rect,

    header_height       : usize,

    widgets             : Vec<Box<dyn Widget>>,
}

impl Widget for Browser {

    fn new() -> Self {

        let mut widgets : Vec<Box<dyn Widget>> = vec![];

        let mut create_tile_button = Box::new(TextButton::new());
        create_tile_button.set_text("BUILDER".to_string());
        create_tile_button.set_has_state(false);
        widgets.push(create_tile_button);

        let mut delete_tile_button = Box::new(TextButton::new());
        delete_tile_button.set_text("DELETE".to_string());
        delete_tile_button.set_has_state(false);
        widgets.push(delete_tile_button);

        let mut focus_tile_button = Box::new(TextButton::new());
        focus_tile_button.set_text("FOCUS".to_string());
        focus_tile_button.set_has_state(false);
        widgets.push(focus_tile_button);

        let mut apply_tool_button = Box::new(TextButton::new());
        apply_tool_button.set_text("APPLY TOOL".to_string());
        widgets.push(apply_tool_button);

        Self {
            rect            : Rect::empty(),

            widgets,

            header_height   : 30,
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, pixels: &mut [u8], stride: usize, context: &mut Context, world: &World, ctx: &TheContext) {

        let mut r = self.rect.to_usize();
        ctx.draw.rect(pixels, &r, context.width, &context.color_widget);
        ctx.draw.rect(pixels, &(r.0 + r.2 - 1, r.1, 1, r.3), ctx.width, &context.color_black);
        ctx.draw.rect(pixels, &(r.0, r.1, r.2, 1), ctx.width, &context.color_black);

        r.3 = self.header_height;
/* *
        ctx.draw.rect(pixels, &r, ctx.width, &context.color_toolbar);
        ctx.draw.rect(pixels, &(r.0, r.1 + r.3, r.2, 1), ctx.width, &context.color_black);

        // --- Mode Widgets

        self.mode_widgets[0].set_rect(Rect::new(self.rect.x + 5, self.rect.y + 2, 100, self.header_height - 6));
        self.mode_widgets[1].set_rect(Rect::new(self.rect.x + 110, self.rect.y + 2, 100, self.header_height - 6));

        for w in &mut self.mode_widgets {
            w.draw(pixels, context, world, ctx);
        }

        ctx.draw.rect(pixels, &(r.0 + 220, r.1 + 4, 1, self.header_height - 8), ctx.width, &context.color_selected);
*/
        let start_x = r.0 + 10;

        // --- Widgets

        self.widgets[0].set_rect(Rect::new(start_x, self.rect.y + 2, 100, self.header_height - 6));
        self.widgets[1].set_rect(Rect::new(start_x+ 110, self.rect.y + 2, 100, self.header_height - 6));
        self.widgets[2].set_rect(Rect::new(start_x+ 220, self.rect.y + 2, 100, self.header_height - 6));
        self.widgets[3].set_rect(Rect::new(self.rect.x + self.rect.width - 10 - 130, self.rect.y + 2, 130, self.header_height - 6));

        for w in &mut self.widgets {
            w.draw(pixels, stride, context, world, ctx);
        }

        // ---

        let crect = Rect::new(self.rect.x, self.rect.y + self.header_height, self.rect.width, self.rect.height - self.header_height);

        let obj_left = 80;
        let obj_height = 40;

        if context.curr_object.tools.is_empty() == false {
            for t in &context.curr_object.tools {

            }
        } else {
            let mut rect = crect.clone();
            rect.x += obj_left;
            rect.width -= 1 + obj_left;
            rect.height = obj_height;
            ctx.draw.rect(pixels, &rect.to_usize(), context.width, &context.color_toolbar);

            let mut left_rect = crect.clone();
            left_rect.width = obj_left;
            left_rect.height = obj_height;

            if let Some(font) = &context.font {
                ctx.draw.blend_text_rect(pixels, &left_rect.to_usize(), context.width, &font, 20.0, &"TOOL".to_string(), &context.color_text, theframework::thedraw2d::TheTextAlignment::Center);
            }
        }

        /*
        if self.mode == Mode::CodeEditor {
            context.code_editor.draw(pixels, content_rect.to_usize(), ctx.width);
        } else {*/
            /*
            self.content_rects = vec![];
            self.ids = vec![];

            r.1 += r.3;
            r.2 = self.item_size.0;
            r.3 = self.item_size.1;

            let curr_name = context.curr_tool.name();

            self.content_rects = vec![];
            for tool_name in &context.curr_tools {

                let mut color = &context.color_green;
                let mut border_color = &context.color_green;
                let ro = 0.0;

                if let Some(tool) = context.tools.get(tool_name) {
                    if tool.role() == ToolRole::Tile {
                        color = &context.color_blue;
                        border_color = &context.color_blue;
                    }
                }

                if curr_name == *tool_name {
                    border_color = &context.color_white;
                }

                ctx.draw.rounded_rect_with_border(pixels, &r, ctx.width, &color, &(ro,ro, ro, ro), border_color, 1.5);

                ctx.draw.text_rect(pixels, &r, ctx.width, &context.font.as_ref().unwrap(), 17.0, tool_name, &context.color_text, &color, theframework::thedraw2d::TheTextAlignment::Center);

                self.content_rects.push(Rect::from(r));

                if r.1 + r.3 > ctx.height {
                    r.0 += r.2;
                    r.1 = self.rect.y + self.header_height;
                } else {
                    r.1 += r.3;
                }
            }*/
        //}
    }

    fn contains(&mut self, x: f32, y: f32) -> bool {
        if self.rect.is_inside((x as usize, y as usize)) {
            true
        } else {
            false
        }
    }

    fn touch_down(&mut self, x: f32, y: f32, context: &mut Context, world: &World) -> bool {

        if self.contains(x, y) {

            for (index, w) in self.widgets.iter_mut().enumerate() {
                if w.touch_down(x, y, context, world) {
                    if index == 0 {

                    } else
                    if index == 1 {

                    } else
                    if index == 2 {
                    } else
                    if index == 3 {
                        context.cmd = Some(Command::ApplyTool);
                    }

                    return true;
                }
            }

            false
        } else {
            false
        }
    }

    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        if self.contains(x, y) {

        }

        false
    }

    fn touch_up(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        false
    }

}