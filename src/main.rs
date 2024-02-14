use egui::{pos2, Pos2, Rect, Visuals};
use objects::Objects;

pub mod objects;

pub struct App {
    pub objects: Objects
}

#[derive(Debug)]
pub struct AppState {
    pub clip: Rect,
    pub mouse_position: Pos2
}

impl App {
    pub fn from_context(context: &eframe::CreationContext<'_>) -> Self {
        // set visuals
        context.egui_ctx.set_visuals(Visuals::light());
        
        // create objects
        let mut me = Self { objects: Objects::default() };
        let test = me.objects.add(objects::ObjectType::FunctionParameter, 0.0, 0.0);
        test.name = "Test Me".to_string();
        me
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // create top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |_ui| {
                    
                });
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

                // setup state
                let mouse_position = ctx.input(|input| input.pointer.interact_pos().unwrap_or(pos2(0.0, 0.0)));
                let state = AppState { clip, mouse_position };

                // draw objects
                self.objects.objects.iter_mut().for_each(|obj| shapes.extend(obj.draw(ui, &state)));

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
