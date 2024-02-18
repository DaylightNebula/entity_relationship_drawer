use std::f32::consts::PI;

use egui::{epaint::{PathShape, RectShape}, pos2, Align2, Color32, FontId, Pos2, Rect, Shape, Stroke, Ui};

use crate::{objects::{Object, ObjectType}, AppState};

pub fn draw_object(
    object: &mut Object,
    ui: &mut Ui,
    state: &mut AppState
) -> Vec<Shape> {
    // calculate center point
    let center = pos2(
        state.clip.width() / 2.0 + state.clip.min.x + object.x + state.scroll_offset.x,
        state.clip.height() / 2.0 + state.clip.min.y + object.y + state.scroll_offset.y
    );

    // check if hovering
    let is_selected = Some(object.id) == state.selected;
    let is_hovering = (center.x - state.mouse_position.x).abs() <= object.width / 2.0 && (center.y - state.mouse_position.y).abs() <= object.height / 2.0;
    let color = if is_hovering || is_selected { Color32::BLUE } else { Color32::BLACK };

    if object.dragging && !state.dragging { object.dragging = false; }

    // if me selected
    if is_selected {
        // if necessary, deselect
        if state.click && !state.skip_click_check && !is_hovering { state.selected = None; }
        // otherwise, if dragging and hover, mark dragging
        else if state.dragging && is_hovering {
            object.dragging = true;
        }

        // do drag
        if object.dragging {
            object.x = -state.clip.width() / 2.0 + state.mouse_position.x - state.scroll_offset.x;
            object.y = -state.clip.height() / 2.0 - (object.height) + state.mouse_position.y - state.scroll_offset.y;
        }
    } 
    // select me if not already selected but hovered
    else if state.click && is_hovering { state.selected = Some(object.id); }

    ui.fonts(|fonts| {
        let font_id = FontId { size: 14.0, family: egui::FontFamily::Monospace };

        // update widths and heights
        let text_width = object.name.char_indices().into_iter().map(|(_, char)| fonts.glyph_width(&font_id, char)).sum::<f32>();
        let text_height = font_id.size;
        object.width = text_width + 20.0;
        object.height = text_height + 20.0;

        match object.object_type {
            ObjectType::Entity => vec![
                Shape::Rect(RectShape {
                    rect: Rect { 
                        min: pos2(-object.width / 2.0 + center.x, -object.height / 2.0 + center.y), 
                        max: pos2(object.width / 2.0 + center.x, object.height / 2.0 + center.y)
                    },
                    rounding: 0.0.into(),
                    fill: Color32::WHITE, 
                    stroke: Stroke { width: 2.0, color }, 
                    fill_texture_id: egui::TextureId::default(), 
                    uv: Rect { min: pos2(0.0, 0.0), max: pos2(1.0, 1.0) }
                }),
                Shape::text(
                    fonts, 
                    center, 
                    Align2::CENTER_CENTER, 
                    &object.name, 
                    font_id, 
                    color
                )
            ],

            ObjectType::EntityDependent => vec![
                Shape::Rect(RectShape {
                    rect: Rect { 
                        min: pos2(-object.width / 2.0 + center.x - 5.0, -object.height / 2.0 + center.y - 5.0), 
                        max: pos2(object.width / 2.0 + center.x + 5.0, object.height / 2.0 + center.y + 5.0) 
                    },
                    rounding: 0.0.into(),
                    fill: Color32::WHITE, 
                    stroke: Stroke { width: 2.0, color }, 
                    fill_texture_id: egui::TextureId::default(), 
                    uv: Rect { min: pos2(0.0, 0.0), max: pos2(1.0, 1.0) }
                }),
                Shape::Rect(RectShape {
                    rect: Rect { 
                        min: pos2(-object.width / 2.0 + center.x, -object.height / 2.0 + center.y), 
                        max: pos2(object.width / 2.0 + center.x, object.height / 2.0 + center.y)
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
                    &object.name, 
                    font_id, 
                    color
                )
            ],

            ObjectType::Relationship { .. } => vec![
                Shape::Path(PathShape { 
                    points: vec![
                        pos2(center.x, -object.width / 2.0 + center.y),
                        pos2(-object.width / 2.0 + center.x, center.y),
                        pos2(center.x, object.width / 2.0 + center.y),
                        pos2(object.width / 2.0 + center.x, center.y)
                    ], 
                    closed: true, 
                    fill: Color32::WHITE, 
                    stroke: Stroke { width: 2.0, color }
                }),
                Shape::text(
                    fonts, 
                    center, 
                    Align2::CENTER_CENTER, 
                    &object.name, 
                    font_id, 
                    color
                )
            ],

            ObjectType::RelationshipDependent { .. } => vec![
                Shape::Path(PathShape { 
                    points: vec![
                        pos2(center.x, -object.width / 2.0 + center.y - 5.0),
                        pos2(-object.width / 2.0 + center.x - 5.0, center.y),
                        pos2(center.x, object.width / 2.0 + center.y + 5.0),
                        pos2(object.width / 2.0 + center.x + 5.0, center.y)
                    ], 
                    closed: true, 
                    fill: Color32::WHITE, 
                    stroke: Stroke { width: 2.0, color }
                }),
                Shape::Path(PathShape { 
                    points: vec![
                        pos2(center.x, -object.width / 2.0 + center.y),
                        pos2(-object.width / 2.0 + center.x, center.y),
                        pos2(center.x, object.width / 2.0 + center.y),
                        pos2(object.width / 2.0 + center.x, center.y)
                    ], 
                    closed: true, 
                    fill: Color32::TRANSPARENT, 
                    stroke: Stroke { width: 2.0, color }
                }),
                Shape::text(
                    fonts, 
                    center, 
                    Align2::CENTER_CENTER, 
                    &object.name, 
                    font_id, 
                    color
                )
            ],

            ObjectType::Parameter => vec![
                Shape::Path(PathShape { 
                    points: (0 .. 100).map(|idx| {
                        let perc = idx as f32 / 100.0;
                        pos2(
                            f32::cos(perc * 2.0 * PI) * (object.width / 2.0) + center.x, 
                            f32::sin(perc * 2.0 * PI) * (object.height / 2.0) + center.y
                        )
                    }).collect(), 
                    closed: true, 
                    fill: Color32::WHITE, 
                    stroke: Stroke { width: 2.0, color }
                }),
                Shape::text(
                    fonts, 
                    center, 
                    Align2::CENTER_CENTER, 
                    &object.name, 
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
                            f32::cos(perc * 2.0 * PI) * (object.width / 2.0) + center.x, 
                            f32::sin(perc * 2.0 * PI) * (object.height / 2.0) + center.y
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
                    &object.name, 
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
                            f32::cos(perc * 2.0 * PI) * (object.width / 2.0) + center.x, 
                            f32::sin(perc * 2.0 * PI) * (object.height / 2.0) + center.y
                        )
                    }).collect(), 
                    closed: true, 
                    fill: Color32::WHITE, 
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
                    &object.name, 
                    font_id, 
                    color
                )
            ],
        }
    })
}