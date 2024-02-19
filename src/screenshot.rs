use egui::{pos2, Event, Pos2, Rect, ViewportCommand, Visuals};
use native_dialog::FileDialog;

use crate::{draw_lines::draw_link, draw_object::draw_object, objects::Objects, AppState};

pub struct ScreenshotApp {
    objects: Objects,
    size: Rect
}

impl eframe::App for ScreenshotApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // create state
        let mut state = &mut AppState { 
            clip: Rect { min: Pos2::default(), max: Pos2 { x: self.size.width(), y: self.size.height() } }, 
            scroll_offset: Pos2::default(),
            click: false,
            delete: false,
            mouse_position: pos2(-100.0, -100.0),
            selected: None,
            dragging: false,
            skip_click_check: false
        };

        // draw
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut shapes = vec![];
            let mut card_ids = Vec::new();
            let mut union_ids = Vec::new();
            self.objects.objects.iter_mut().for_each(|object| {
                shapes.extend(draw_object(object, ui, state));
            });
            self.objects.links.iter().for_each(|link| {
                let a = self.objects.objects.iter().find(|a| a.id == link.a).unwrap();
                let b = self.objects.objects.iter().find(|a| a.id == link.b).unwrap();
                shapes.extend(draw_link(&mut card_ids, &mut union_ids, a, b, ui, &mut state, &link.minmax));
            });
            ui.painter().extend(shapes);
        });
        
        ctx.input(|input| {
            input.raw.events.iter().for_each(|event| {
                println!("Event {:?}", event);
                match event {
                    Event::Screenshot { image, .. } => {
                        println!("Screenshot {:?}", image);
                        let path = FileDialog::new()
                            .set_location("~")
                            .add_filter("PNG", &["png"])
                            .show_save_single_file()
                            .unwrap();
                        println!("Path {:?}", path);
                        if path.is_some() {
                            image::save_buffer(
                                &path.unwrap(), 
                                image.as_raw(), 
                                image.width() as u32,
                                image.height() as u32, 
                                image::ColorType::Rgba8
                            ).unwrap();
                        } else {
                            println!("Could not save!");
                        }
                        panic!("Forcing exit!");
                    },
                    _ => {}
                }
            });
        });
    }
}

pub fn screenshot(objects: Objects) {
    // get size of objects
    // let mut size = Rect { min: Pos2 { x: f32::MAX, y: f32::MAX }, max: Pos2 { x: f32::MIN, y: f32::MIN } };
    // objects.objects.iter().for_each(|object| {
    //     let min = pos2(object.x - object.width, object.y - object.width);
    //     let max = pos2(object.x + object.width, object.y + object.width);
    //     if min.x < size.min.x { size.min.x = min.x; }
    //     if min.y < size.min.y { size.min.y = min.y; }
    //     if max.x > size.max.x { size.max.x = max.x; }
    //     if max.y > size.max.y { size.max.y = max.y; }
    // });

    let width = 1920.0;
    let height = 1080.0;
    println!("Size {} {}", width, height);

    // run screen shot app
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([width, height])
            .with_min_inner_size([width, height]),
        ..Default::default()
    };

    // run screenshot app
    let app = ScreenshotApp { objects, size: Rect { min: pos2(0.0, 0.0), max: pos2(1920.0, 1080.0) } };
    eframe::run_native("screenshot", native_options, Box::new(|ctx| {
        ctx.egui_ctx.set_visuals(Visuals::light());
        ctx.egui_ctx.send_viewport_cmd(ViewportCommand::Screenshot);
        Box::new(app)
    })).unwrap();
}