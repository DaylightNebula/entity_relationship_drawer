use std::f32::consts::PI;

use egui::{epaint::{PathShape, RectShape}, pos2, Align2, Color32, FontId, Pos2, Rect, Shape, Stroke, Ui};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Objects {
    pub objects: Vec<Object>,
    pub next_id: u32
}

impl Objects {
    pub fn add(&mut self, object_type: ObjectType, x: f32, y: f32) -> &mut Object {
        self.objects.push(Object { id: self.next_id, x, y, width: 0.0, height: 0.0, name: String::new(), object_type, dragging: false });
        self.next_id += 1;
        self.objects.iter_mut().last().expect("Physics just broke")
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Object {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub object_type: ObjectType,
    pub dragging: bool
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ObjectType {
    #[default]
    Entity,
    Relationship,
    Parameter,
    EntityDependent,
    RelationshipDependent,
    FunctionParameter,
    KeyParameter
}

impl Object {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        state: &mut AppState
    ) -> Vec<Shape> {
        // calculate center point
        let center = pos2(
            state.clip.width() / 2.0 + state.clip.min.x + self.x + state.scroll_offset.x,
            state.clip.height() / 2.0 + state.clip.min.y + self.y + state.scroll_offset.y
        );

        // check if hovering
        let is_selected = Some(self.id) == state.selected;
        let is_hovering = (center.x - state.mouse_position.x).abs() <= self.width / 2.0 && (center.y - state.mouse_position.y).abs() <= self.height / 2.0;
        let color = if is_hovering || is_selected { Color32::BLUE } else { Color32::BLACK };

        if self.dragging && !state.dragging { self.dragging = false; }

        // if me selected
        if is_selected {
            let mut skip_click_check = false;

            // draw window
            egui::Window::new("Edit Element")
                .show(ui.ctx(), |ui| {
                    ui.label(format!("ID {:?}", self.id));

                    // if mouse contained, make sure to cancel click checks
                    if ui.rect_contains_pointer(ui.clip_rect()) { skip_click_check = true; }

                    // select type
                    let mut combo_changed = false;
                    egui::ComboBox::from_label("Object Type")
                        .selected_text(format!("{:?}", self.object_type))
                        .show_ui(ui, |ui| {
                            // yes I know doing this twice is kinda hacky
                            if ui.rect_contains_pointer(ui.clip_rect()) { skip_click_check = true; }

                            // options
                            let a = ui.selectable_value(&mut self.object_type, ObjectType::Entity, "Entity");
                            let b = ui.selectable_value(&mut self.object_type, ObjectType::EntityDependent, "Entity Dependent");
                            let c = ui.selectable_value(&mut self.object_type, ObjectType::Relationship, "Relationship");
                            let d = ui.selectable_value(&mut self.object_type, ObjectType::RelationshipDependent, "Relationship Dependent");
                            let e = ui.selectable_value(&mut self.object_type, ObjectType::Parameter, "Parameter");
                            let f = ui.selectable_value(&mut self.object_type, ObjectType::FunctionParameter, "Functional Parameter");
                            let g = ui.selectable_value(&mut self.object_type, ObjectType::KeyParameter, "Key Parameter");

                            // update combo changed
                            if a.clicked() || b.clicked() || c.clicked() || d.clicked() || e.clicked() || f.clicked() || g.clicked() { combo_changed = true; }
                        });

                    // edit name
                    let edit = ui.text_edit_singleline(&mut self.name);

                    // do text formatting
                    if edit.changed() || combo_changed {
                        self.name = self.name.replace(" ", "_");
                        match self.object_type {
                            ObjectType::Entity | 
                            ObjectType::EntityDependent | 
                            ObjectType::Relationship | 
                            ObjectType::RelationshipDependent => {
                                self.name = self.name.to_uppercase();
                            },
                            ObjectType::Parameter |
                            ObjectType::KeyParameter |
                            ObjectType::FunctionParameter => {}
                        }
                    }

                    // delete button
                    if ui.button("Delete").clicked() || state.delete {
                        state.to_delete.push(self.id); 
                        state.selected = None;
                    }
                });

            // if necessary, deselect
            if state.click && !skip_click_check && !is_hovering { state.selected = None; }
            // otherwise, if dragging and hover, mark dragging
            else if state.dragging && is_hovering {
                self.dragging = true;
            }

            // do drag
            if self.dragging {
                self.x = -state.clip.width() / 2.0 + state.mouse_position.x;
                self.y = -state.clip.height() / 2.0 - (self.height) + state.mouse_position.y;
            }
        } 
        // select me if not already selected but hovered
        else if state.click && is_hovering { state.selected = Some(self.id); }

        ui.fonts(|fonts| {
            let font_id = FontId { size: 14.0, family: egui::FontFamily::Monospace };

            // update widths and heights
            let text_width = self.name.char_indices().into_iter().map(|(_, char)| fonts.glyph_width(&font_id, char)).sum::<f32>();
            let text_height = font_id.size;
            self.width = text_width + 20.0;
            self.height = text_height + 20.0;

            match self.object_type {
                ObjectType::Entity => vec![
                    Shape::Rect(RectShape {
                        rect: Rect { 
                            min: pos2(-self.width / 2.0 + center.x, -self.height / 2.0 + center.y), 
                            max: pos2(self.width / 2.0 + center.x, self.height / 2.0 + center.y)
                        },
                        rounding: 0.0.into(),
                        fill: Color32::TRANSPARENT, 
                        stroke: Stroke { width: 2.0, color }, 
                        fill_texture_id: egui::TextureId::default(), 
                        uv: Rect { min: pos2(0.0, 0.0), max: pos2(1.0, 1.0) }
                    }),
                    Shape::text(
                        fonts, 
                        center, 
                        Align2::CENTER_CENTER, 
                        &self.name, 
                        font_id, 
                        color
                    )
                ],

                ObjectType::EntityDependent => vec![
                    Shape::Rect(RectShape {
                        rect: Rect { 
                            min: pos2(-self.width / 2.0 + center.x - 5.0, -self.height / 2.0 + center.y - 5.0), 
                            max: pos2(self.width / 2.0 + center.x + 5.0, self.height / 2.0 + center.y + 5.0) 
                        },
                        rounding: 0.0.into(),
                        fill: Color32::TRANSPARENT, 
                        stroke: Stroke { width: 2.0, color }, 
                        fill_texture_id: egui::TextureId::default(), 
                        uv: Rect { min: pos2(0.0, 0.0), max: pos2(1.0, 1.0) }
                    }),
                    Shape::Rect(RectShape {
                        rect: Rect { 
                            min: pos2(-self.width / 2.0 + center.x, -self.height / 2.0 + center.y), 
                            max: pos2(self.width / 2.0 + center.x, self.height / 2.0 + center.y)
                        },
                        rounding: 0.0.into(),
                        fill: Color32::TRANSPARENT, 
                        stroke: Stroke { width: 2.0, color }, 
                        fill_texture_id: egui::TextureId::default(), 
                        uv: Rect { min: pos2(0.0, 0.0), max: pos2(1.0, 1.0) }
                    }),
                    Shape::text(
                        fonts, 
                        center, 
                        Align2::CENTER_CENTER, 
                        &self.name, 
                        font_id, 
                        color
                    )
                ],

                ObjectType::Relationship => vec![
                    Shape::Path(PathShape { 
                        points: vec![
                            pos2(center.x, -self.width / 2.0 + center.y),
                            pos2(-self.width / 2.0 + center.x, center.y),
                            pos2(center.x, self.width / 2.0 + center.y),
                            pos2(self.width / 2.0 + center.x, center.y)
                        ], 
                        closed: true, 
                        fill: Color32::TRANSPARENT, 
                        stroke: Stroke { width: 2.0, color }
                    }),
                    Shape::text(
                        fonts, 
                        center, 
                        Align2::CENTER_CENTER, 
                        &self.name, 
                        font_id, 
                        color
                    )
                ],

                ObjectType::RelationshipDependent => vec![
                    Shape::Path(PathShape { 
                        points: vec![
                            pos2(center.x, -self.width / 2.0 + center.y),
                            pos2(-self.width / 2.0 + center.x, center.y),
                            pos2(center.x, self.width / 2.0 + center.y),
                            pos2(self.width / 2.0 + center.x, center.y)
                        ], 
                        closed: true, 
                        fill: Color32::TRANSPARENT, 
                        stroke: Stroke { width: 2.0, color }
                    }),
                    Shape::Path(PathShape { 
                        points: vec![
                            pos2(center.x, -self.width / 2.0 + center.y - 5.0),
                            pos2(-self.width / 2.0 + center.x - 5.0, center.y),
                            pos2(center.x, self.width / 2.0 + center.y + 5.0),
                            pos2(self.width / 2.0 + center.x + 5.0, center.y)
                        ], 
                        closed: true, 
                        fill: Color32::TRANSPARENT, 
                        stroke: Stroke { width: 2.0, color }
                    }),
                    Shape::text(
                        fonts, 
                        center, 
                        Align2::CENTER_CENTER, 
                        &self.name, 
                        font_id, 
                        color
                    )
                ],

                ObjectType::Parameter => vec![
                    Shape::Path(PathShape { 
                        points: (0 .. 100).map(|idx| {
                            let perc = idx as f32 / 100.0;
                            pos2(
                                f32::cos(perc * 2.0 * PI) * (self.width / 2.0) + center.x, 
                                f32::sin(perc * 2.0 * PI) * (self.height / 2.0) + center.y
                            )
                        }).collect(), 
                        closed: true, 
                        fill: Color32::TRANSPARENT, 
                        stroke: Stroke { width: 2.0, color }
                    }),
                    Shape::text(
                        fonts, 
                        center, 
                        Align2::CENTER_CENTER, 
                        &self.name, 
                        font_id, 
                        color
                    )
                ],

                ObjectType::FunctionParameter => {
                    let mut result = Vec::new();

                    result.extend(Shape::dashed_line( 
                        &(0 .. 100).map(|idx| {
                            let perc = idx as f32 / 100.0;
                            pos2(
                                f32::cos(perc * 2.0 * PI) * (self.width / 2.0) + center.x, 
                                f32::sin(perc * 2.0 * PI) * (self.height / 2.0) + center.y
                            )
                        }).collect::<Vec<Pos2>>(), 
                        Stroke { width: 2.0, color },
                        5.0,
                        5.0
                    ));

                    result.push(Shape::text(
                        fonts, 
                        center, 
                        Align2::CENTER_CENTER, 
                        &self.name, 
                        font_id, 
                        color
                    ));

                    result
                },

                ObjectType::KeyParameter=> vec![
                    Shape::Path(PathShape { 
                        points: (0 .. 100).map(|idx| {
                            let perc = idx as f32 / 100.0;
                            pos2(
                                f32::cos(perc * 2.0 * PI) * (self.width / 2.0) + center.x, 
                                f32::sin(perc * 2.0 * PI) * (self.height / 2.0) + center.y
                            )
                        }).collect(), 
                        closed: true, 
                        fill: Color32::TRANSPARENT, 
                        stroke: Stroke { width: 2.0, color }
                    }),
                    Shape::LineSegment { 
                        points: [
                            pos2(text_width / 2.0 + center.x, center.y + (text_height / 2.0)), 
                            pos2(-text_width / 2.0 + center.x, center.y + (text_height / 2.0))
                        ], 
                        stroke: Stroke { width: 2.0, color }
                    },
                    Shape::text(
                        fonts, 
                        center, 
                        Align2::CENTER_CENTER, 
                        &self.name, 
                        font_id, 
                        color
                    )
                ],
            }
        })
    }
}
