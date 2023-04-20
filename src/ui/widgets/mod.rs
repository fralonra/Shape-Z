use crate::prelude::*;

pub mod text_button;
pub mod tile_editor;

#[allow(unused)]
pub trait Widget : Sync + Send {

    fn new() -> Self where Self: Sized;

    fn set_rect(&mut self, rect: Rect);
    fn set_text(&mut self, text: String) {}
    fn set_cmd(&mut self, cmd: Command) {}

    fn get_state(&mut self) -> bool { false }
    fn set_has_state(&mut self, state: bool) {}

    fn is_visible(&self) -> bool { return true; }
    fn set_visible(&mut self, visible: bool) { }

    fn update(&mut self, context: &mut Context) {}

    fn draw(&mut self, pixels: &mut [u8], context: &mut Context, world: &World, ctx: &TheContext);

    fn contains(&mut self, x: f32, y: f32) -> bool {
        false
    }

    fn touch_down(&mut self, button: i32, x: f32, y: f32, context: &mut Context) -> bool {
        false
    }

    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {
        false
    }

    fn touch_up(&mut self, _x: f32, _y: f32, context: &mut Context) -> bool {
        false
    }
}