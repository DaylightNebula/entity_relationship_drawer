use std::{fs::File, io::Write, path::PathBuf, process::Command, str::FromStr};

use egui::{pos2, Pos2, Rect, Visuals};
use native_dialog::*;
use objects::Objects;

pub mod objects;
pub mod screenshot;

pub struct App {
    pub objects: Objects,
    pub scroll_offset: Pos2,
    pub selected: Option<u32>,
    pub saved_to: Option<PathBuf>
}

#[derive(Debug)]
pub struct AppState {
    pub clip: Rect,
    pub scroll_offset: Pos2,
    pub mouse_position: Pos2,
    pub selected: Option<u32>,
    pub click: bool,
    pub delete: bool,
    pub dragging: bool,
    pub to_delete: Vec<u32>
}

impl App {
    pub fn from_context(context: &eframe::CreationContext<'_>) -> Self {
        // set visuals
        context.egui_ctx.set_visuals(Visuals::light());
        
        // create objects
        Self { objects: Objects::default(), scroll_offset: Pos2::default(), selected: None, saved_to: None }
    }

    pub fn save_as(&mut self) {
        // get save location
        let path = FileDialog::new()
            .set_location("~")
            .add_filter("Entity Relationship File", &["er"])
            .show_save_single_file()
            .unwrap();
    
        // do save
        if path.is_some() {
            self.save(path.unwrap());
        }
    }

    pub fn save(&mut self, path: PathBuf) {
        let to_save = serde_json::to_string(&self.objects);
        let file = File::create(path.clone());
        if file.is_ok() && to_save.is_ok() {
            let _ = file.unwrap().write(to_save.unwrap().as_bytes());
            self.saved_to = Some(path);
        } else {
            println!("Save error, file: {:?}, to_save: {:?}", file, to_save);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // create top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // create open button
                    if ui.button("Open").clicked() {
                        // get open path
                        let path = FileDialog::new()
                            .set_location("~")
                            .add_filter("Entity Relationship File", &["er"])
                            .show_open_single_file()
                            .unwrap();

                        // do open
                        if path.is_some() {
                            self.objects = serde_json::from_str(std::fs::read_to_string(path.clone().unwrap()).unwrap().as_str()).unwrap();
                            self.saved_to = Some(path.unwrap());
                        }
                    }

                    // create save button
                    if ui.button("Save").clicked() { if self.saved_to.is_some() { self.save(self.saved_to.clone().unwrap()) } else { self.save_as() }; ui.close_menu(); }
                    if ui.button("Save As").clicked() { self.save_as(); ui.close_menu(); }

                    // create export button
                    if ui.button("Export").clicked() {
                        // make sure saved
                        if self.saved_to.is_none() { self.save_as() }
                        if self.saved_to.is_none() { return }

                        // take screen shot
                        let args = std::env::args().collect::<Vec<String>>();
                        let start_path = args.first().expect("Rules broke");
                        Command::new(start_path)
                            .args(["screenshot", self.saved_to.clone().unwrap().to_str().unwrap()])
                            .output()
                            .expect("Screen shot failed!");

                        ui.close_menu();
                    }
                });
                if ui.button("Create").clicked() {
                    let item = self.objects.add(objects::ObjectType::Entity, 0.0, 0.0);
                    self.selected = Some(item.id);
                    ui.close_menu();
                }
                ui.add_space(16.0);
            });
        });

        // create canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            // create frame to draw too
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                // setup ui
                let (_, clip) = ui.allocate_space(ui.available_size());
                ui.set_clip_rect(clip);
                let mut shapes = vec![];

                // read input
                let (mouse_position, click, delete, dragging) = ctx.input(|input| {
                    // middle click drag
                    if input.pointer.is_decidedly_dragging() && input.pointer.button_down(egui::PointerButton::Secondary) {
                        let drag_delta = input.pointer.delta();
                        self.scroll_offset += drag_delta;
                    }

                    // reset scroll
                    if input.key_down(egui::Key::Space) {
                        self.scroll_offset = Pos2::default();
                    }

                    // get pointer position
                    (
                        input.pointer.interact_pos().unwrap_or(pos2(0.0, 0.0)), 
                        input.pointer.button_clicked(egui::PointerButton::Primary),
                        input.key_down(egui::Key::Delete),
                        input.pointer.button_down(egui::PointerButton::Primary)
                    )
                });

                // setup state
                let mut state = AppState { clip, mouse_position, scroll_offset: self.scroll_offset, selected: self.selected, click, delete, dragging, to_delete: Vec::new() };

                // draw objects
                self.objects.objects.iter_mut().for_each(|obj| shapes.extend(obj.draw(ui, &mut state)));

                // sync
                self.selected = state.selected;
                state.to_delete.iter().for_each(|id| {
                    let idx = self.objects.objects.iter().position(|o| o.id == *id).unwrap();
                    self.objects.objects.remove(idx);
                });

                // finalize draw
                ui.painter().extend(shapes);
            });
        });
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() >= 3 && args[1] == "screenshot" {
        println!("Screenshot");
        let path = PathBuf::from_str(args[2].as_str());

        // do screenshot
        let objects: Objects = serde_json::from_str(std::fs::read_to_string(path.unwrap()).unwrap().as_str()).unwrap();
        screenshot::screenshot(objects);
    } else {
        // create default window options
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1280.0, 720.0])
                .with_min_inner_size([800.0, 600.0]),
            ..Default::default()
        };

        // run a eframe app
        eframe::run_native(
            "Entity Relationship Editor", 
            native_options, 
            Box::new(|ctx| Box::new(App::from_context(ctx)))
        ).unwrap();
    }
}
