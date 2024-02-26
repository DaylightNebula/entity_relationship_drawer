use std::{fs::File, io::Write, path::PathBuf, process::Command, str::FromStr};

use draw_lines::draw_link;
use draw_object::draw_object;
use egui::{pos2, Color32, Pos2, Rect, Visuals};
use native_dialog::*;
use objects::{CardType, Link, Object, ObjectType, Objects};

pub mod draw_lines;
pub mod draw_object;
pub mod objects;
pub mod screenshot;
pub mod bminustree;

pub struct App {
    pub objects: Objects,
    pub scroll_offset: Pos2,
    pub selected: Option<u32>,
    pub saved_to: Option<PathBuf>,
    pub search: String,
    pub clip: Rect
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
    pub skip_click_check: bool
}

impl App {
    pub fn from_context(context: &eframe::CreationContext<'_>) -> Self {
        // set visuals
        context.egui_ctx.set_visuals(Visuals::light());

        let mut objects = Objects::default();
        let content = std::fs::read_to_string("bminustreetest.txt").unwrap();
        let content: Vec<String> = content.split("\n").map(|a| a.into()).collect();
        let mut content: Vec<String> = content[0..11].into();
        content.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        objects.create_tree(content);
        
        // create objects
        Self { objects, scroll_offset: Pos2::default(), selected: None, saved_to: None, search: String::new(), clip: Rect { min: Pos2::default(), max: Pos2::default() } }
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
                        let output = Command::new(start_path)
                            .args(["screenshot", self.saved_to.clone().unwrap().to_str().unwrap()])
                            .output()
                            .expect("Screen shot failed!");
                        println!("Export output: {:?}", output);

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

        // read input
        let (mouse_position, click, delete, dragging) = ctx.input(|input| {
            let mouse_position = input.pointer.interact_pos().unwrap_or(pos2(0.0, 0.0));

            // middle click drag
            if input.pointer.is_decidedly_dragging() && input.pointer.button_down(egui::PointerButton::Secondary) {
                // let drag_delta = input.pointer.delta();
                // self.scroll_offset += drag_delta;
            }

            if self.selected.is_none() && input.key_released(egui::Key::Tab) {
                let item = self.objects.add(
                    objects::ObjectType::Entity, 
                    -self.clip.width() / 2.0 - 10.0 + mouse_position.x - self.scroll_offset.x, 
                    -self.clip.height() / 2.0 - 30.0 + mouse_position.y - self.scroll_offset.y
                );
                self.selected = Some(item.id);
            }

            // reset scroll
            if input.key_down(egui::Key::Space) {
                self.scroll_offset = Pos2::default();
            }

            // get pointer position
            (
                mouse_position, 
                input.pointer.button_clicked(egui::PointerButton::Primary),
                input.key_down(egui::Key::Delete),
                input.pointer.button_down(egui::PointerButton::Primary)
            )
        });

        
        // if something is selected, draw selection edit window
        let mut skip_click_check = false;
        if self.selected.is_some() {
            let found = self.objects.objects.iter().find(|a| a.name.eq_ignore_ascii_case(self.search.as_str()) && Some(a.id) != self.selected && !self.search.is_empty()).cloned();
            let mut connected_to = self.objects.links.iter_mut()
                .filter(|a| Some(a.a) == self.selected || Some(a.b) == self.selected)
                .collect::<Vec<&mut Link>>();
            let mut connected_to = connected_to.iter_mut()
                .map(|link| {
                    (self.objects.objects.clone().iter().find(|a| (a.id == link.a || a.id == link.b) && Some(a.id) != self.selected).unwrap().clone(), link)
                })
                .collect::<Vec<(Object, &mut &mut Link)>>();
            let selected = self.objects.objects.iter_mut().find(|a| Some(a.id) == self.selected).unwrap();
            let mut to_remove: Option<u32> = None;
            let mut remove_link: Option<(u32, u32)> = None;
            let mut link = false;

            // draw window
            egui::Window::new("Edit Element")
                .show(ctx, |ui| {
                    ui.label(format!("ID {:?}", selected.id));

                    // if mouse contained, make sure to cancel click checks
                    if ui.rect_contains_pointer(ui.clip_rect()) { skip_click_check = true; }

                    // select type
                    let mut combo_changed = false;
                    egui::ComboBox::from_label("Object Type")
                        .selected_text(format!("{:?}", selected.object_type))
                        .show_ui(ui, |ui| {
                            // yes I know doing this twice is kinda hacky
                            if ui.rect_contains_pointer(ui.clip_rect()) { skip_click_check = true; }

                            // options
                            let a = ui.selectable_value(
                                &mut selected.object_type, 
                                ObjectType::Entity, 
                                "Entity"
                            );
                            let b = ui.selectable_value(
                                &mut selected.object_type, 
                                ObjectType::EntityDependent, 
                                "Entity Dependent"
                            );
                            let c = ui.selectable_value(
                                &mut selected.object_type, 
                                ObjectType::Relationship { card: objects::CardType::OneToOne }, 
                                "Relationship"
                            );
                            let d = ui.selectable_value(
                                &mut selected.object_type, 
                                ObjectType::RelationshipDependent { card: objects::CardType::OneToOne }, 
                                "Relationship Dependent"
                            );
                            let e = ui.selectable_value(
                                &mut selected.object_type, 
                                ObjectType::Parameter { is_id: false }, 
                                "Parameter"
                            );
                            let f = ui.selectable_value(
                                &mut selected.object_type, 
                                ObjectType::FunctionParameter { is_id: false }, 
                                "Functional Parameter"
                            );
                            let g = ui.selectable_value(
                                &mut selected.object_type, 
                                ObjectType::Polymorph { poly: objects::Polymorph::Union }, 
                                "Polymorph"
                            );

                            // update combo changed
                            if a.clicked() || b.clicked() || c.clicked() || d.clicked() || e.clicked() || f.clicked() || g.clicked() { combo_changed = true; }
                        });

                    // edit name
                    let edit = ui.text_edit_singleline(&mut selected.name);

                    match &mut selected.object_type {
                        ObjectType::Relationship { card } |
                        ObjectType::RelationshipDependent { card } => {
                            egui::ComboBox::from_label("Card Type")
                                .selected_text(format!("{:?}", card))
                                .show_ui(ui, |ui| {
                                    // yes I know doing this twice is kinda hacky
                                    if ui.rect_contains_pointer(ui.clip_rect()) { skip_click_check = true; }

                                    let a = ui.selectable_value(card, CardType::OneToOne, "One To One");
                                    let b = ui.selectable_value(card, CardType::OneToMany, "One To Many");
                                    let c = ui.selectable_value(card, CardType::ManyToOne, "Many To One");
                                    let d = ui.selectable_value(card, CardType::ManyToMany, "Many To Many");

                                    // update combo changed
                                    if a.clicked() || b.clicked() || c.clicked() || d.clicked() { combo_changed = true; }
                                });
                        },
                        ObjectType::Parameter { is_id } |
                        ObjectType::FunctionParameter { is_id } => {
                            ui.checkbox(is_id, "Is ID?");
                        }
                        ObjectType::Polymorph { poly } => {
                            egui::ComboBox::from_label("Polymorph Type")
                                .selected_text(format!("{:?}", poly))
                                .show_ui(ui, |ui| {
                                    // yes I know doing this twice is kinda hacky
                                    if ui.rect_contains_pointer(ui.clip_rect()) { skip_click_check = true; }

                                    let a = ui.selectable_value(poly, objects::Polymorph::Union, "Union");
                                    let b = ui.selectable_value(poly, objects::Polymorph::Disjoint, "Disjoin");
                                    let c = ui.selectable_value(poly, objects::Polymorph::Overlapping, "Overlapping");

                                    // update combo changed
                                    if a.clicked() || b.clicked() || c.clicked() { combo_changed = true; }
                                });
                        }
                        _ => {}
                    }

                    // do text formatting
                    if edit.changed() || combo_changed {
                        // selected.name = selected.name;
                        match selected.object_type {
                            ObjectType::Entity | 
                            ObjectType::EntityDependent | 
                            ObjectType::Relationship { .. } | 
                            ObjectType::RelationshipDependent { .. } => {
                                selected.name = selected.name.to_uppercase().replace(" ", "_");
                            },
                            ObjectType::Parameter { .. } |
                            ObjectType::FunctionParameter { .. }  => {}
                            ObjectType::Polymorph { .. } => { selected.name = format!("{}_poly", selected.name.to_lowercase().replace("_poly", "")) }
                        }
                    }

                    // add links
                    ui.collapsing("Links", |ui| {
                        connected_to.iter_mut().for_each(|(other, other_link)| {
                            ui.horizontal(|ui| {
                                // let other = self.objects.objects.iter().find(|a| a.id == other.a || a.id == other.b).unwrap();
                                ui.label(format!("-> {}", other.name));
                                if ui.button("Remove").clicked() {
                                    remove_link = Some((selected.id, other.id));
                                }
                                ui.text_edit_singleline(&mut other_link.minmax);
                            });
                        });
                    });

                    // add link search bar
                    ui.horizontal(|ui| {
                        // attempt to find object we are searching for
                        ui.style_mut().visuals.extreme_bg_color = if found.is_some() { Color32::GREEN } else { Color32::RED };

                        ui.text_edit_singleline(&mut self.search);
                        if ui.button("Link").clicked() && found.is_some() {
                            link = true;
                        }
                    });

                    // delete button
                    if delete {
                        to_remove = Some(selected.id);
                        self.selected = None;
                    }
                });

            // remove marked element if necessary
            if to_remove.is_some() {
                // remove original object
                let idx = self.objects.objects.iter().position(|o| o.id == to_remove.unwrap()).unwrap();
                let object = self.objects.objects.remove(idx);

                // remove links
                self.objects.links.iter().enumerate()
                    .filter(|(_, a)| a.a == object.id || a.b == object.id)
                    .for_each(|(a, _)| { self.objects.objects.remove(a); });
            }

            if remove_link.is_some() {
                let (a, b) = remove_link.unwrap();
                let idx = self.objects.links.iter().position(|link| (link.a == a || link.a == b) && (link.b == a || link.b == b));
                if idx.is_some() { self.objects.links.remove(idx.unwrap()); }
            }

            if link {
                self.objects.links.push(Link { a: self.selected.unwrap(), b: found.unwrap().id, minmax: String::new() });
            }
        } else {
            if !self.search.is_empty() { self.search = String::new() }
        }

        // create canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            // create frame to draw too
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                // setup ui
                let (_, clip) = ui.allocate_space(ui.available_size());
                ui.set_clip_rect(clip);
                self.clip = clip;
                let mut shapes = vec![];

                // setup state
                let mut state = AppState { clip, mouse_position, scroll_offset: self.scroll_offset, selected: self.selected, click, delete, dragging, skip_click_check };

                // draw objects
                let mut card_ids = Vec::new();
                let mut union_ids = Vec::new();
                self.objects.objects.iter_mut().for_each(|obj| shapes.extend(draw_object(obj, ui, &mut state)));
                self.objects.links.iter().for_each(|link| {
                    let a = self.objects.objects.iter().find(|a| a.id == link.a).unwrap();
                    let b = self.objects.objects.iter().find(|a| a.id == link.b).unwrap();
                    shapes.extend(draw_link(&mut card_ids, &mut union_ids, a, b, ui, &mut state, &link.minmax));
                });

                // sync
                self.selected = state.selected;

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
