use egui::{FontFamily, FontId, Rect, TextStyle, Vec2, Visuals};
use egui_plot::{Legend, Plot, PlotPoint};
use objects::Objects;

pub mod objects;

pub struct App {
    pub objects: Objects
}

#[derive(Debug, Default)]
pub struct AppState {
    pub mouse_position: Vec2,
    pub vaspect: f32,
    pub haspect: f32
}

impl App {
    pub fn from_context(context: &eframe::CreationContext<'_>) -> Self {
        // set visuals
        context.egui_ctx.set_visuals(Visuals::light());

        // set font sizes
        context.egui_ctx.style_mut(|style| {
            style.text_styles = [
                (TextStyle::Heading, FontId::new(30.0, FontFamily::Proportional)),
                (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
                (TextStyle::Monospace, FontId::new(14.0, FontFamily::Proportional)),
                (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
                (TextStyle::Small, FontId::new(15.0, FontFamily::Proportional)),
            ].into();
        });
        
        let mut me = Self { objects: Objects::default() };
        let test = me.objects.add(objects::ObjectType::Entity, 0.0, 0.0);
        test.name = "Test Me".to_string();
        me
    }

    // fn circle(&self) -> Line {
    //     let n = 512;
    //     let circle_points: PlotPoints = (0..=n)
    //         .map(|i| {
    //             let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
    //             let r = 0.05;
    //             [
    //                 r * t.cos() + 0.0 as f64,
    //                 r * t.sin() + 0.0 as f64,
    //             ]
    //         })
    //         .collect();
    //     Line::new(circle_points)
    //         .color(Color32::from_rgb(0, 0, 0))
    //         .style(egui_plot::LineStyle::Dashed { length: 10.0 })
    // }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // create app state
        let mut state = AppState::default();
        ctx.input(|input| {
            let rect = input.viewport().inner_rect.unwrap();
            state.vaspect = rect.height() / rect.width();
            state.haspect = rect.width() / rect.height();
        });

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
            // create plot
            let _response = Plot::new("drawing")
                .legend(Legend::default())
                .show_axes(false)
                .show_grid(false)
                .show_x(false)
                .show_y(false)
                .allow_zoom(false)
                .allow_boxed_zoom(false)
                .data_aspect(1.0)
                .auto_bounds(false.into())
                .show(ui, |ui| {
                    // update app state
                    let pointer = ui.pointer_coordinate();
                    if pointer.is_some() {
                        let pointer = pointer.unwrap();
                        state.mouse_position.x = pointer.x as f32;
                        state.mouse_position.y = pointer.y as f32;
                    }

                    // draw objects
                    self.objects.objects.iter_mut().for_each(|object| {
                        object.draw(ui, &mut state);
                    });

                    // ui.line(self.circle());
                    // ui.text(Text::new([0.0, 0.0].into(), "Test Text").color(Color32::from_rgb(0, 0, 0)));
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
