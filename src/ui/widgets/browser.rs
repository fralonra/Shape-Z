
use crate::{prelude::*, tool::ToolRole};

#[derive(PartialEq, Debug, Clone)]
enum Mode {
    Tools,
    Navigator
}

pub struct Browser {
    mode                : Mode,

    rect                : Rect,

    navigator           : Navigator,

    header_height       : usize,

    mode_widgets        : Vec<Box<dyn Widget>>,
    tool_widgets        : Vec<Box<dyn Widget>>,
    navi_widgets        : Vec<Box<dyn Widget>>,

    // Tools
    item_size           : (usize, usize),
    content_rects       : Vec<Rect>,
    ids                 : Vec<Uuid>,
}

impl Widget for Browser {

    fn new() -> Self {

        let mut mode_widgets : Vec<Box<dyn Widget>> = vec![];

        let mut tools_button = Box::new(TextButton::new());
        tools_button.set_text("TOOLS".to_string());
        tools_button.set_has_state(true);
        mode_widgets.push(tools_button);

        let mut navigator_button = Box::new(TextButton::new());
        navigator_button.set_text("NAVI".to_string());
        navigator_button.set_has_state(false);
        mode_widgets.push(navigator_button);

        let tool_widgets : Vec<Box<dyn Widget>> = vec![];

        let mut navi_widgets : Vec<Box<dyn Widget>> = vec![];

        let mut create_tile_button = Box::new(TextButton::new());
        create_tile_button.set_text("CREATE".to_string());
        create_tile_button.set_has_state(false);
        navi_widgets.push(create_tile_button);

        let mut delete_tile_button = Box::new(TextButton::new());
        delete_tile_button.set_text("DELETE".to_string());
        delete_tile_button.set_has_state(false);
        navi_widgets.push(delete_tile_button);

        let mut focus_tile_button = Box::new(TextButton::new());
        focus_tile_button.set_text("FOCUS".to_string());
        focus_tile_button.set_has_state(false);
        navi_widgets.push(focus_tile_button);

        let mut apply_tool_button = Box::new(TextButton::new());
        apply_tool_button.set_text("APPLY TOOL".to_string());
        navi_widgets.push(apply_tool_button);

        Self {
            mode            : Mode::Tools,

            rect            : Rect::empty(),

            navigator       : Navigator::new(),

            mode_widgets,
            tool_widgets,
            navi_widgets,

            header_height   : 30,

            item_size       : (120, 25),
            content_rects   : vec![],
            ids             : vec![],
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, pixels: &mut [u8], context: &mut Context, world: &World, ctx: &TheContext) {

        let mut r = self.rect.to_usize();
        ctx.draw.rect(pixels, &r, context.width, &context.color_widget);
        ctx.draw.rect(pixels, &(r.0 + r.2, r.1, 1, r.3), ctx.width, &context.color_black);

        r.3 = self.header_height;

        ctx.draw.rect(pixels, &r, ctx.width, &context.color_toolbar);
        ctx.draw.rect(pixels, &(r.0, r.1 + r.3, r.2, 1), ctx.width, &context.color_black);

        // --- Mode Widgets

        self.mode_widgets[0].set_rect(Rect::new(self.rect.x + 5, self.rect.y + 2, 100, self.header_height - 6));
        self.mode_widgets[1].set_rect(Rect::new(self.rect.x + 110, self.rect.y + 2, 100, self.header_height - 6));

        for w in &mut self.mode_widgets {
            w.draw(pixels, context, world, ctx);
        }

        ctx.draw.rect(pixels, &(r.0 + 220, r.1 + 4, 1, self.header_height - 8), ctx.width, &context.color_selected);

        let start_x = r.0 + 230;

        // --- TOOLS / NAVI Widgets

        if self.mode == Mode::Navigator {
            self.navi_widgets[0].set_rect(Rect::new(start_x, self.rect.y + 2, 100, self.header_height - 6));
            self.navi_widgets[1].set_rect(Rect::new(start_x+ 110, self.rect.y + 2, 100, self.header_height - 6));
            self.navi_widgets[2].set_rect(Rect::new(start_x+ 220, self.rect.y + 2, 100, self.header_height - 6));
            self.navi_widgets[3].set_rect(Rect::new(self.rect.x + self.rect.width - 10 - 130, self.rect.y + 2, 130, self.header_height - 6));

            for w in &mut self.navi_widgets {
                w.draw(pixels, context, world, ctx);
            }
        }

        // ---

        self.navigator.set_rect(Rect::new(self.rect.x, self.rect.y + self.header_height, self.rect.width, self.rect.height - self.header_height));

        if self.mode == Mode::Navigator {
            self.navigator.draw(pixels, context, world, ctx);
        } else {

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
            }
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

        if self.contains(x, y) {

            for (index, widget) in self.mode_widgets.iter_mut().enumerate() {
                if widget.touch_down(x, y, context, world) {
                    if index == 0 {
                        self.mode = Mode::Tools;
                        self.mode_widgets[0].set_has_state(true);
                        self.mode_widgets[1].set_has_state(false);
                        return true;
                    } else
                    if index == 1 {
                        self.mode = Mode::Navigator;
                        self.mode_widgets[0].set_has_state(false);
                        self.mode_widgets[1].set_has_state(true);
                        return true;
                    }
                }
            }

            if self.mode == Mode::Navigator {
                for (index, w) in self.navi_widgets.iter_mut().enumerate() {
                    if w.touch_down(x, y, context, world) {
                        if index == 0 {
                            if w.get_state() {
                                self.navigator.set_mode(NavigatorMode::Create);
                                self.navi_widgets[1].set_has_state(false);
                                self.navi_widgets[2].set_has_state(false);
                            } else {
                                self.navigator.set_mode(NavigatorMode::None)
                            }
                        } else
                        if index == 1 {
                            if w.get_state() {
                                self.navigator.set_mode(NavigatorMode::Delete);
                                self.navi_widgets[0].set_has_state(false);
                                self.navi_widgets[2].set_has_state(false);
                            } else {
                                self.navigator.set_mode(NavigatorMode::None)
                            }
                        } else
                        if index == 2 {
                            if w.get_state() {
                                self.navigator.set_mode(NavigatorMode::Focus);
                                self.navi_widgets[0].set_has_state(false);
                                self.navi_widgets[1].set_has_state(false);
                            } else {
                                self.navigator.set_mode(NavigatorMode::None)
                            }
                        } else
                        if index == 3 {
                            context.cmd = Some(Command::ApplyTool);
                        }

                        return true;
                    }
                }
            }

            if self.mode == Mode::Tools {
                for (index, r) in self.content_rects.iter().enumerate() {
                    if r.is_inside((x as usize, y as usize)) {
                        if let Some(tool) = context.tools.get(&context.curr_tools[index]) {
                            context.curr_tool = tool.clone();
                            context.curr_tool_role = tool.role();
                            return true;
                        }
                    }
                }
            } else
            if self.mode == Mode::Navigator {
                if y as usize > self.rect.y + self.header_height {
                    return self.navigator.touch_down(x, y, context);
                }
            }

            false
        } else {
            false
        }
    }

    fn touch_dragged(&mut self, x: f32, y: f32, _context: &mut Context) -> bool {

        if self.contains(x, y) {
            if self.mode == Mode::Navigator {
                return self.navigator.touch_dragged(x, y);
            }
        }

        false
    }

    fn touch_up(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        if self.mode == Mode::Navigator {
            for (_index, w) in self.navi_widgets.iter_mut().enumerate() {
                if w.touch_up(x, y, context) {
                    return true;
                }
            }
        }

        false
    }
}