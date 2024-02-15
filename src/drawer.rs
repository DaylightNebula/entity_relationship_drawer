use std::f32::consts::PI;

use egui::{epaint::{PathShape, RectShape}, pos2, Align2, Color32, FontId, Pos2, Rect, Shape, Stroke, Ui};

use crate::{objects::{Object, ObjectType}, AppState};

pub fn draw_link(
    a: &Object,
    b: &Object,
    _ui: &mut Ui,
    state: &mut AppState
) -> Vec<Shape> {
    let use_double = a.object_type.use_double_link() || b.object_type.use_double_link();

    // get some angles
    let a_to_b = f32::atan2(a.y - b.y, a.x - b.x);
    let b_to_a = f32::atan2(b.y - a.y, b.x - a.x);

    // line
    let primary = vec![
        get_point_around_object(a, b_to_a, state),
        get_point_around_object(b, a_to_b, state)
    ];
        
    match use_double {
        true => {
            let offset = pos2(
                (a_to_b - (PI / 4.0)).cos() * 3.0,
                (a_to_b - (PI / 4.0)).sin() * 3.0
            );
            vec![
                Shape::line(
                    vec![
                        pos2(primary[0].x + offset.x, primary[0].y + offset.y),
                        pos2(primary[1].x + offset.x, primary[1].y + offset.y),
                    ], 
                    Stroke { width: 2.0, color: Color32::BLACK }
                ),
                Shape::line(
                    vec![
                        pos2(primary[0].x - offset.x, primary[0].y - offset.y),
                        pos2(primary[1].x - offset.x, primary[1].y - offset.y),
                    ], 
                    Stroke { width: 2.0, color: Color32::BLACK }
                ),
            ]
        },
        false => vec![Shape::line(primary, Stroke { width: 2.0, color: Color32::BLACK })]
    }
}

// gets a point around an object by an angle
pub fn get_point_around_object(
    object: &Object,
    rad: f32,
    state: &AppState
) -> Pos2 {
    let center = pos2(
        state.clip.width() / 2.0 + state.clip.min.x + object.x + state.scroll_offset.x,
        state.clip.height() / 2.0 + state.clip.min.y + object.y + state.scroll_offset.y
    );

    match object.object_type {
        // get edge on entity square
        ObjectType::Entity |
        ObjectType::EntityDependent => {
            let direction = pos2(rad.cos() * 10.0, rad.sin() * 10.0);
            let a_mult = (object.width / 2.0) / direction.x;
            let b_mult = (object.height / 2.0) / direction.y;
            let mult = (a_mult.abs()).min(b_mult.abs());
            pos2(
                direction.x * mult + center.x,
                direction.y * mult + center.y
            )
        },

        // get edge on relationship diamond
        ObjectType::Relationship |
        ObjectType::RelationshipDependent => {
            let x_part = (-rad.abs() + (PI / 2.0)) / (PI / 2.0);
            let a_part = rad / (PI / 2.0);
            let y_part = if a_part >= 0.0 { -(-a_part + 1.0).abs() + 1.0 } else { (-(a_part + 1.0).abs() + 1.0) * -1.0 };
            pos2(
                (object.width / 2.0) * x_part + center.x,
                (object.width / 2.0) * y_part + center.y
            )
        },

        // get edge on parameter circle
        ObjectType::Parameter |
        ObjectType::FunctionParameter |
        ObjectType::KeyParameter => Pos2 { 
            x: rad.cos() * (object.width / 2.0) + center.x, 
            y: rad.sin() * (object.height / 2.0) + center.y
        },
    }
    
}

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
            object.x = -state.clip.width() / 2.0 + state.mouse_position.x;
            object.y = -state.clip.height() / 2.0 - (object.height) + state.mouse_position.y;
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

            ObjectType::Relationship => vec![
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

            ObjectType::RelationshipDependent => vec![
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