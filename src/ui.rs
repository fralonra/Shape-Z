pub mod widgets;
pub mod rect;
pub mod context;
pub mod navigator;

pub mod prelude {
    pub use crate::ui::widgets::*;
    pub use crate::ui::rect::Rect;
    pub use crate::ui::context::*;
    pub use crate::ui::navigator::*;

    pub use crate::ui::widgets::text_button::*;
    pub use crate::ui::widgets::settings::*;
    pub use crate::ui::widgets::browser::*;
    pub use crate::ui::widgets::palettebar::*;
    pub use crate::ui::widgets::switch_button::*;

    pub use crate::ui::widgets::tool_extrusion::*;
}

#[repr(usize)]
enum WidgetIndices {
    SettingsIndex,
    BrowserIndex,
    ModeBarIndex,
}

#[repr(usize)]
enum ToolBarIndices {
    EditSwitchIndex,
    IsoButtonIndex,
}

use WidgetIndices::*;
use ToolBarIndices::*;

pub use crate::prelude::*;

pub struct UI {

    widgets                         : Vec<Box<dyn Widget>>,
    toolbar_widgets                 : Vec<Box<dyn Widget>>,

    toolbar_rect                    : Rect,
    toolbar_buffer                  : Vec<u8>,
    toolbar_dirty                   : bool,

    pub toolbar_height              : usize,
    pub palettebar_width            : usize,
    pub settings_width              : usize,
    pub browser_height              : usize,
}

impl UI {

    pub fn new() -> Self {

        // Widgets

        let mut widgets : Vec<Box<dyn Widget>> = vec![];

        let settings = Box::new(Settings::new());
        widgets.push(settings);

        let browser = Box::new(Browser::new());
        widgets.push(browser);

        let palette: Box<PaletteBar> = Box::new(PaletteBar::new());
        widgets.push(palette);

        // Toolbar Widgets

        let mut toolbar_widgets : Vec<Box<dyn Widget>> = vec![];

        let mut edit_switch = Box::new(SwitchButton::new());
        edit_switch.set_text_list(vec!["PIXEL".to_string(), "PBR".to_string()]);
        edit_switch.set_cmd(Command::EditStateSwitched);
        toolbar_widgets.push(edit_switch);

        let mut iso_button = Box::new(TextButton::new());
        iso_button.set_text("ORTHO".to_string());
        iso_button.set_cmd(Command::IsoStateSwitched);
        iso_button.set_has_state(false);
        toolbar_widgets.push(iso_button);

        Self {
            widgets,
            toolbar_widgets,

            toolbar_rect            : Rect::empty(),
            toolbar_buffer          : vec![],
            toolbar_dirty           : true,

            toolbar_height          : 90,
            palettebar_width        : 162,
            settings_width          : 300,
            browser_height          : 180,
        }
    }

    pub fn draw(&mut self, pixels: &mut [u8], context: &mut Context, world: &World, ctx: &TheContext) {

        // Toolbar

        self.toolbar_rect = Rect::new(0, 0, ctx.width, self.toolbar_height);

        // Draw toolbar in an offscreen buffer on change and blit it

        if self.toolbar_buffer.len() != ctx.width * self.toolbar_height * 4 {
            self.toolbar_buffer = vec![0;ctx.width * self.toolbar_height * 4];
            self.toolbar_dirty = true;
        }

        if self.toolbar_dirty {
            let frame = &mut self.toolbar_buffer[..];

            //println!("drawing toolbar");

            ctx.draw.rect(frame, &(0, 0, self.toolbar_rect.width as usize, self.toolbar_rect.height as usize), ctx.width, &context.color_toolbar);
            ctx.draw.rect(frame, &(0, 45, ctx.width, 1), ctx.width, &[21, 21, 21, 255]);

            // --- Icon

            if let Some(logo) = context.icons.get(&"logo_toolbar".to_string()) {
                ctx.draw.blend_slice(frame, &logo.0, &(4, 2, logo.1 as usize, logo.2 as usize), context.width);
            }

            self.toolbar_widgets[EditSwitchIndex as usize].set_rect(Rect::new(10, 52, 200, 34));
            self.toolbar_widgets[IsoButtonIndex as usize].set_rect(Rect::new(ctx.width - 125, 52, 120, 32));

            for w in &mut self.toolbar_widgets {
                w.draw(frame, context, world, ctx);
            }

            self.toolbar_dirty = false;
        }

        ctx.draw.copy_slice(pixels, &self.toolbar_buffer, &self.toolbar_rect.to_usize(), ctx.width);

        // Settings rect

        let settings_rect = Rect::new(context.width - self.settings_width, self.toolbar_height, self.settings_width, context.height - self.toolbar_height);

        self.widgets[SettingsIndex as usize].set_rect(settings_rect.clone());

        // --- ModeBar

        let modebar_rect: Rect = Rect::new(0, self.toolbar_height, self.palettebar_width, ctx.height - self.toolbar_height);

        self.widgets[ModeBarIndex as usize].set_rect(modebar_rect.clone());

        // --- Browser

        let browser_rect: Rect = Rect::new(self.palettebar_width, context.height - self.browser_height, context.width - self.settings_width -  self.palettebar_width, self.browser_height);

        self.widgets[BrowserIndex as usize].set_rect(browser_rect.clone());

        // ---

        for w in &mut self.widgets {
            w.draw(pixels, context, world, ctx);
        }
    }

    pub fn contains(&mut self, x: f32, y: f32) -> bool {

        if self.toolbar_rect.is_inside((x as usize, y as usize)) {
            return true;
        }

        for w in &mut self.widgets {
            if w.contains(x, y) {
                return true;
            }
        }
        false
    }

    pub fn touch_down(&mut self, x: f32, y: f32, context: &mut Context, world: &World) -> bool {

        for w in &mut self.toolbar_widgets {
            if w.touch_down(x, y, context, world) {
                self.toolbar_dirty = true;
                return true;
            }
        }

        for w in &mut self.widgets {
            if w.touch_down(x, y, context, world) {
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
        let mut consumed = false;

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

    /// Returns the edit state
    pub fn get_edit_state(&self) -> bool {
        !self.toolbar_widgets[EditSwitchIndex as usize].get_state()
    }

    /// Returns the iso state
    pub fn get_iso_state(&self) -> bool {
        !self.toolbar_widgets[IsoButtonIndex as usize].get_state()
    }

}