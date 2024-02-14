use std::f32::consts::PI;

use egui::{epaint::{PathShape, RectShape}, pos2, Align2, Color32, FontId, Pos2, Rect, Shape, Stroke, Ui};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Objects {
    pub objects: Vec<Object>,
    pub next_id: u32
}

impl Objects {
    pub fn add(&mut self, object_type: ObjectType, x: f32, y: f32) -> &mut Object {
        self.objects.push(Object { id: self.next_id, x, y, width: 0.0, height: 0.0, name: String::new(), object_type });
        self.next_id += 1;
        self.objects.iter_mut().last().expect("Physics just broke")
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Object {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub object_type: ObjectType
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ObjectType {
    #[default]
    Entity,
    Relationship,
    Parameter,
    EntityDependent,
    RelationshipDependent,
    FunctionParameter
}

impl Object {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        state: &AppState
    ) -> Vec<Shape> {
        ui.fonts(|fonts| {
            let font_id = FontId { size: 14.0, family: egui::FontFamily::Monospace };

            // calculate center point
            let center = pos2(
                state.clip.width() / 2.0 + state.clip.min.x + self.x,
                state.clip.height() / 2.0 + state.clip.min.y + self.y
            );

            // update widths and heights
            let text_width = self.name.char_indices().into_iter().map(|(_, char)| fonts.glyph_width(&font_id, char)).sum::<f32>();
            let text_height = font_id.size;
            self.width = text_width + 20.0;
            self.height = text_height + 20.0;

            // check if hovering
            let is_hovering = (center.x - state.mouse_position.x).abs() <= self.width / 2.0 && (center.y - state.mouse_position.y).abs() <= self.height / 2.0;
            let color = if is_hovering { Color32::BLUE } else { Color32::BLACK };

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
                }
            }
        })
    }
}
