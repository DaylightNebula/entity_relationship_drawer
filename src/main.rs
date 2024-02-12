use std::f64::consts::TAU;

use egui::{remap, Color32, FontFamily, FontId, TextStyle, Visuals};
use egui_plot::{Legend, Line, Plot, PlotPoints, Text};
use objects::Objects;

pub mod objects;

pub struct App {
    pub objects: Objects
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
        
        Self { objects: Objects::default() }
    }

    fn circle(&self) -> Line {
        let n = 512;
        let circle_points: PlotPoints = (0..=n)
            .map(|i| {
                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                let r = 0.05;
                [
                    r * t.cos() + 0.0 as f64,
                    r * t.sin() + 0.0 as f64,
                ]
            })
            .collect();
        Line::new(circle_points)
            .color(Color32::from_rgb(0, 0, 0))
            .style(egui_plot::LineStyle::Dashed { length: 10.0 })
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
                    ui.line(self.circle());
                    ui.text(Text::new([0.0, 0.0].into(), "Test Text").color(Color32::from_rgb(0, 0, 0)));
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
