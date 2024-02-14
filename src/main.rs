use egui::{pos2, Pos2, Rect, Visuals};
use objects::Objects;

pub mod objects;

pub struct App {
    pub objects: Objects,
    pub scroll_offset: Pos2,
    pub selected: Option<u32>
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
        Self { objects: Objects::default(), scroll_offset: Pos2::default(), selected: None }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // create top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |_ui| {
                    
                });
                // ui.menu_button("Create", |ui| {
                //     if ui.button("Entity").clicked() {
                //         let item = self.objects.add(objects::ObjectType::Entity, 0.0, 0.0);
                //         self.selected = Some(item.id);
                //         ui.close_menu();
                //     }
                // });
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
