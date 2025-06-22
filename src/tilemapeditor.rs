use crate::editor::RUSTERIX;
use crate::editor::UNDOMANAGER;
use crate::prelude::*;
use shared::tilemap;

#[derive(PartialEq)]
enum AddMode {
    Single,
    Anim,
    Multi,
}

use AddMode::*;

pub struct TilemapEditor {
    curr_tilemap_id: Uuid,
    add_mode: AddMode,
}

#[allow(clippy::new_without_default)]
impl TilemapEditor {
    pub fn new() -> Self {
        Self {
            curr_tilemap_id: Uuid::new_v4(),
            add_mode: Single,
        }
    }

    pub fn build(&self) -> TheCanvas {
        let mut canvas = TheCanvas::new();

        let rgba_layout = TheRGBALayout::new(TheId::named("Tilemap Editor"));
        canvas.set_layout(rgba_layout);

        let mut toolbar_canvas = TheCanvas::new();
        let traybar_widget = TheTraybar::new(TheId::empty());
        toolbar_canvas.set_widget(traybar_widget);

        let mut clear_button = TheTraybarButton::new(TheId::named("Tilemap Editor Clear"));
        clear_button.set_text("Clear".to_string());
        clear_button.set_status_text("Clear the current selection.");

        //let icon_view = TheIconView::new(TheId::named("Tilemap Editor Icon View"));

        let mut toolbar_hlayout = TheHLayout::new(TheId::empty());
        toolbar_hlayout.set_background_color(None);
        toolbar_hlayout.set_margin(Vec4::new(10, 4, 5, 4));

        /*
        let mut tile_name_text = TheText::new(TheId::empty());
        tile_name_text.set_text("Tags".to_string());
        toolbar_hlayout.add_widget(Box::new(tile_name_text));

        let mut tile_name_edit = TheTextLineEdit::new(TheId::named("Tilemap Editor Name Edit"));
        tile_name_edit.limiter_mut().set_max_width(80);
        toolbar_hlayout.add_widget(Box::new(tile_name_edit));

        let mut block_name_text = TheText::new(TheId::empty());
        block_name_text.set_text("Blocking".to_string());
        toolbar_hlayout.add_widget(Box::new(block_name_text));

        let block_check_button: TheCheckButton =
            TheCheckButton::new(TheId::named("Tilemap Editor Block"));
        toolbar_hlayout.add_widget(Box::new(block_check_button));

        let mut hdivider = TheHDivider::new(TheId::empty());
        hdivider.limiter_mut().set_max_width(15);
        toolbar_hlayout.add_widget(Box::new(hdivider));
        */

        let mut drop_down = TheDropdownMenu::new(TheId::named("Tilemap Editor Role"));

        for dir in TileRole::iterator() {
            drop_down.add_option(dir.to_string().to_string());
        }
        toolbar_hlayout.add_widget(Box::new(drop_down));

        let mut hdivider = TheHDivider::new(TheId::empty());
        hdivider.limiter_mut().set_max_width(15);
        toolbar_hlayout.add_widget(Box::new(hdivider));

        let mut add_switch = TheGroupButton::new(TheId::named("Tilemap Editor Switch"));
        add_switch.add_text_status("Single".to_string(), "Show tile picker.".to_string());
        add_switch.add_text_status(
            "Anim".to_string(),
            "Apply procedural materials.".to_string(),
        );
        add_switch.add_text_status("Multi".to_string(), "Apply a color.".to_string());

        add_switch.set_item_width(70);
        add_switch.set_index(0);
        toolbar_hlayout.add_widget(Box::new(add_switch));

        let mut hdivider = TheHDivider::new(TheId::empty());
        hdivider.limiter_mut().set_max_width(15);
        toolbar_hlayout.add_widget(Box::new(hdivider));

        let mut add_button = TheTraybarButton::new(TheId::named("Tilemap Editor Add"));
        add_button.set_text("Add Tiles".to_string());
        add_button.set_status_text(
            "Adds the current selection as animation, every tile is one animation frame.",
        );

        toolbar_hlayout.add_widget(Box::new(add_button));

        // let mut hdivider = TheHDivider::new(TheId::empty());
        // hdivider.limiter_mut().set_max_width(15);
        // toolbar_hlayout.add_widget(Box::new(hdivider));

        let mut zoom = TheSlider::new(TheId::named("Tilemap Editor Zoom"));
        zoom.set_value(TheValue::Float(2.0));
        zoom.set_range(TheValue::RangeF32(0.5..=5.0));
        zoom.set_continuous(true);
        zoom.limiter_mut().set_max_width(120);
        toolbar_hlayout.add_widget(Box::new(zoom));
        toolbar_hlayout.add_widget(Box::new(clear_button));
        toolbar_hlayout.set_reverse_index(Some(2));

        // Details
        let mut details_canvas = TheCanvas::new();

        let mut vlayout = TheVLayout::new(TheId::named(" Tile Details Layout"));
        vlayout.set_margin(Vec4::new(5, 20, 5, 10));
        vlayout.set_alignment(TheHorizontalAlign::Center);
        vlayout.limiter_mut().set_max_width(120);

        // let mut switch_button = TheTraybarButton::new(TheId::named("Tilemap Selection Switch"));
        // switch_button.set_text("Anim".to_string());
        // switch_button
        //     .set_status_text("Switches between an anim based preview and multi tiles preview.");

        let mut icon_preview = TheIconView::new(TheId::named("Tilemap Selection Preview"));
        icon_preview.set_alpha_mode(false);
        icon_preview.limiter_mut().set_max_size(Vec2::new(100, 100));
        icon_preview.set_border_color(Some([100, 100, 100, 255]));

        // vlayout.add_widget(Box::new(switch_button));
        vlayout.add_widget(Box::new(icon_preview));

        details_canvas.set_layout(vlayout);

        toolbar_canvas.set_layout(toolbar_hlayout);
        canvas.set_top(toolbar_canvas);
        canvas.set_right(details_canvas);

        canvas
    }

    /// Set the current tilemap
    pub fn set_tilemap(
        &mut self,
        tilemap: &tilemap::Tilemap,
        ui: &mut TheUI,
        ctx: &mut TheContext,
    ) {
        self.curr_tilemap_id = tilemap.id;

        ui.set_widget_value("Tilemap Editor Zoom", ctx, TheValue::Float(tilemap.zoom));

        if let Some(rgba_layout) = ui.get_rgba_layout("Tilemap Editor") {
            rgba_layout.set_buffer(tilemap.buffer.clone());
            rgba_layout.set_scroll_offset(tilemap.scroll_offset);
            if let Some(rgba_view) = rgba_layout.rgba_view_mut().as_rgba_view() {
                rgba_view.set_grid(Some(tilemap.grid_size));
                rgba_view.set_mode(TheRGBAViewMode::TileSelection);
                rgba_view.set_background([116, 116, 116, 255]);
                let mut c = WHITE;
                c[3] = 128;
                rgba_view.set_hover_color(Some(c));
                rgba_view.set_rectangular_selection(true);
                rgba_view.set_dont_show_grid(true);
                rgba_view.set_zoom(tilemap.zoom);

                let mut used = FxHashSet::default();

                // Compute used
                for tile in &tilemap.tiles {
                    for region in &tile.sequence.regions {
                        used.insert((
                            region.x as i32 / tilemap.grid_size,
                            region.y as i32 / tilemap.grid_size,
                        ));
                    }
                }
                rgba_view.set_used(used);
            }
        }
    }

    /// Clears the selection
    pub fn clear(&mut self, ui: &mut TheUI) {
        if let Some(editor) = ui
            .canvas
            .get_layout(Some(&"Tilemap Editor".to_string()), None)
        {
            if let Some(editor) = editor.as_rgba_layout() {
                editor
                    .rgba_view_mut()
                    .as_rgba_view()
                    .unwrap()
                    .set_selection(FxHashSet::default());
            }
        }
        self.set_tilemap_preview(TheRGBATile::default(), ui);
    }

    /// Set the selection preview
    pub fn set_tilemap_preview(&self, tile: TheRGBATile, ui: &mut TheUI) {
        if let Some(icon_view) = ui.get_icon_view("Tilemap Selection Preview") {
            icon_view.set_rgba_tile(tile);
        }
    }

    /// Compute the selection preview
    pub fn compute_preview(&mut self, project: &mut Project, ui: &mut TheUI) {
        if let Some(rgba_layout) = ui.get_rgba_layout("Tilemap Editor") {
            if let Some(rgba_view) = rgba_layout.rgba_view_mut().as_rgba_view() {
                if self.add_mode == Single {
                    let mut tile = rgba_view.selection_as_tile();
                    if let Some(last) = tile.buffer.last() {
                        tile.buffer = vec![last.clone()];
                    }
                    self.set_tilemap_preview(tile, ui);
                } else if self.add_mode == Anim {
                    let selection = rgba_view.selection_as_sequence();
                    let mut tile = TheRGBATile::default();
                    if let Some(tilemap) = project.get_tilemap(self.curr_tilemap_id) {
                        tile.buffer = tilemap.buffer.extract_sequence(&selection);
                    }
                    self.set_tilemap_preview(tile, ui);
                } else {
                    let mut tile = TheRGBATile::default();
                    let dim = rgba_view.selection_as_dim();

                    if let Some(tilemap) = project.get_tilemap(self.curr_tilemap_id) {
                        let region = TheRGBARegion::new(
                            dim.x as usize * tilemap.grid_size as usize,
                            dim.y as usize * tilemap.grid_size as usize,
                            dim.width as usize * tilemap.grid_size as usize,
                            dim.height as usize * tilemap.grid_size as usize,
                        );
                        tile.buffer.push(tilemap.buffer.extract_region(&region));
                    }
                    self.set_tilemap_preview(tile, ui);
                }
            }
        }
    }

    pub fn handle_event(
        &mut self,
        event: &TheEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        project: &mut Project,
        _server_ctx: &mut ServerContext,
    ) -> bool {
        let mut redraw = false;

        match event {
            TheEvent::IndexChanged(id, index) => {
                if id.name == "Tilemap Editor Switch" {
                    if *index == 0 {
                        self.add_mode = Single;
                    } else if *index == 1 {
                        self.add_mode = Anim;
                    } else {
                        self.add_mode = Multi;
                    }
                    self.compute_preview(project, ui);
                    redraw = true;
                }
            }
            TheEvent::DialogValueOnClose(role, name, uuid, value) => {
                if name == "Rename Tileset" && *role == TheDialogButtonRole::Accept {
                    if let Some(tilemap) = project.get_tilemap(self.curr_tilemap_id) {
                        tilemap.name = value.describe();
                        ctx.ui.send(TheEvent::SetValue(*uuid, value.clone()));
                    }
                }
            }
            TheEvent::ContextMenuSelected(_widget_id, item_id) => {
                if item_id.name == "Rename Tileset" {
                    if let Some(tilemap) = project.get_tilemap(self.curr_tilemap_id) {
                        open_text_dialog(
                            "Rename Tileset",
                            "Tilset Name",
                            tilemap.name.as_str(),
                            self.curr_tilemap_id,
                            ui,
                            ctx,
                        );
                    }
                } else if item_id.name == "Add Tileset Colors" {
                    let prev = project.palette.clone();
                    if let Some(tilemap) = project.get_tilemap(self.curr_tilemap_id).cloned() {
                        let width = tilemap.buffer.dim().width;
                        let height = tilemap.buffer.dim().height;
                        for y in 0..height {
                            for x in 0..width {
                                if let Some(c) = tilemap.buffer.get_pixel(x, y) {
                                    let color = TheColor::from(c);
                                    if color.a == 1.0 {
                                        project.palette.add_unique_color(color);
                                    }
                                }
                            }
                        }
                    }
                    if let Some(palette_picker) = ui.get_palette_picker("Palette Picker") {
                        let index = palette_picker.index();

                        palette_picker.set_palette(project.palette.clone());
                        if let Some(widget) = ui.get_widget("Palette Color Picker") {
                            if let Some(color) = &project.palette[index] {
                                widget.set_value(TheValue::ColorObject(color.clone()));
                            }
                        }
                        if let Some(widget) = ui.get_widget("Palette Hex Edit") {
                            if let Some(color) = &project.palette[index] {
                                widget.set_value(TheValue::Text(color.to_hex()));
                            }
                        }
                    }
                    redraw = true;

                    let undo = PaletteUndoAtom::Edit(prev, project.palette.clone());
                    UNDOMANAGER.write().unwrap().add_palette_undo(undo, ctx);
                }
            }
            TheEvent::TileSelectionChanged(id) => {
                if id.name == "Tilemap Editor View" {
                    self.compute_preview(project, ui);
                }
            }
            TheEvent::StateChanged(id, state) => {
                if id.name == "Tilemap Editor Clear" && *state == TheWidgetState::Clicked {
                    self.clear(ui);
                } else if id.name == "Tilemap Editor Add" {
                    let mut clear_selection = false;

                    if let Some(editor) = ui
                        .canvas
                        .get_layout(Some(&"Tilemap Editor".to_string()), None)
                    {
                        if let Some(editor) = editor.as_rgba_layout() {
                            let mut tiles = vec![];

                            if self.add_mode == Single {
                                let sequence = editor
                                    .rgba_view_mut()
                                    .as_rgba_view()
                                    .unwrap()
                                    .selection_as_sequence();
                                for region in sequence.regions {
                                    let mut tile = Tile::default();
                                    let mut s = TheRGBARegionSequence::default();
                                    s.regions.push(region);
                                    tile.sequence = s;
                                    tiles.push(tile);
                                }
                            } else if self.add_mode == Anim {
                                let mut tile = Tile::default();
                                let sequence = editor
                                    .rgba_view_mut()
                                    .as_rgba_view()
                                    .unwrap()
                                    .selection_as_sequence();
                                tile.sequence = sequence;
                                tiles.push(tile);
                            } else if self.add_mode == Multi {
                                let mut tile = Tile::default();
                                let dim = editor
                                    .rgba_view_mut()
                                    .as_rgba_view()
                                    .unwrap()
                                    .selection_as_dim();

                                let mut grid_size = 16;

                                if let Some(t) = project.get_tilemap(self.curr_tilemap_id) {
                                    grid_size = t.grid_size;
                                }

                                let region = TheRGBARegion::new(
                                    dim.x as usize * grid_size as usize,
                                    dim.y as usize * grid_size as usize,
                                    dim.width as usize * grid_size as usize,
                                    dim.height as usize * grid_size as usize,
                                );

                                tile.sequence = TheRGBARegionSequence::new();
                                tile.sequence.regions.push(region);
                                tiles.push(tile);
                            }

                            for mut tile in tiles {
                                if let Some(text_line_edit) =
                                    ui.get_text_line_edit("Tilemap Editor Name Edit")
                                {
                                    tile.name = text_line_edit.text();
                                }

                                // if let Some(block_widget) = ui
                                //     .canvas
                                //     .get_widget(Some(&"Tilemap Editor Block".to_string()), None)
                                // {
                                //     tile.blocking = block_widget.state() == TheWidgetState::Selected;
                                // }

                                if let Some(role_widget) =
                                    ui.get_drop_down_menu("Tilemap Editor Role")
                                {
                                    let index = role_widget.selected_index();
                                    tile.role = TileRole::from_index(index as u8).unwrap();
                                }

                                // Only add if non-empty
                                if !tile.sequence.regions.is_empty() {
                                    if let Some(layout) = ui
                                        .canvas
                                        .get_layout(Some(&"Tilemap Tile List".to_string()), None)
                                    {
                                        let list_layout_id = layout.id().clone();
                                        if let Some(list_layout) = layout.as_list_layout() {
                                            let mut item = TheListItem::new(TheId::named_with_id(
                                                "Tilemap Tile",
                                                tile.id,
                                            ));
                                            item.set_text(tile.name.clone());
                                            let mut sub_text = if tile.blocking {
                                                "Blocking".to_string()
                                            } else {
                                                "Non-Blocking".to_string()
                                            };
                                            sub_text +=
                                                ("  ".to_string() + tile.role.to_string()).as_str();
                                            item.set_sub_text(sub_text);
                                            item.set_state(TheWidgetState::Selected);
                                            item.set_size(42);
                                            item.set_associated_layout(list_layout_id);
                                            if let Some(t) =
                                                project.get_tilemap(self.curr_tilemap_id)
                                            {
                                                item.set_icon(
                                                    tile.sequence.regions[0]
                                                        .scale(&t.buffer, 36, 36),
                                                );
                                            }
                                            list_layout.deselect_all();
                                            let id = item.id().clone();
                                            list_layout.add_item(item, ctx);
                                            ctx.ui.send_widget_state_changed(
                                                &id,
                                                TheWidgetState::Selected,
                                            );

                                            clear_selection = true;
                                            redraw = true;
                                        }
                                    }

                                    if let Some(tilemap) = project.get_tilemap(self.curr_tilemap_id)
                                    {
                                        tilemap.tiles.push(tile);
                                        self.set_tilemap(tilemap, ui, ctx);
                                    }

                                    let tiles = project.extract_tiles();
                                    RUSTERIX
                                        .write()
                                        .unwrap()
                                        .assets
                                        .set_rgba_tiles(tiles.clone());

                                    ctx.ui.send(TheEvent::Custom(
                                        TheId::named("Update Tilepicker"),
                                        TheValue::Empty,
                                    ));
                                    //self.update_tiles(ui, ctx, project);
                                }
                            }
                        }
                    }

                    // Clear the selection if successful
                    if clear_selection {
                        if let Some(editor) = ui
                            .canvas
                            .get_layout(Some(&"Tilemap Editor".to_string()), None)
                        {
                            if let Some(editor) = editor.as_rgba_layout() {
                                editor
                                    .rgba_view_mut()
                                    .as_rgba_view()
                                    .unwrap()
                                    .set_selection(FxHashSet::default());
                            }
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Tilemap Editor Clear"),
                                TheWidgetState::Clicked,
                            ))
                        }
                    }
                }
            }
            TheEvent::KeyCodeDown(TheValue::KeyCode(key)) => {
                if *key == TheKeyCode::Escape {
                    self.clear(ui);
                }
            }
            TheEvent::ValueChanged(_id, _value) => {}
            _ => {}
        }
        redraw
    }
}
