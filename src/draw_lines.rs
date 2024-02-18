use std::f32::consts::PI;

use egui::{pos2, vec2, Align2, Color32, FontId, Shape, Stroke, Ui, Vec2};

use crate::{objects::{CardType, Object, ObjectType, Polymorph}, AppState};

pub fn draw_link(
    card_ids: &mut Vec<u32>,
    union_ids: &mut Vec<u32>,
    a: &Object,
    b: &Object,
    ui: &mut Ui,
    state: &mut AppState,
    minmax: &String
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

    // draw lines
    let mut shapes = match use_double {
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
        false => vec![Shape::line(vec![[primary[0].x, primary[0].y].into(), [primary[1].x, primary[1].y].into()], Stroke { width: 2.0, color: Color32::BLACK })]
    };

    // get center point and above and below on the line
    let center = (primary[0] + primary[1]) / 2.0;
    let center_a = pos2(
        f32::cos(a_to_b + (PI / 2.0)),
        f32::sin(a_to_b + (PI / 2.0))
    );
    let center_b = pos2(
        f32::cos(a_to_b - (PI / 2.0)),
        f32::sin(a_to_b - (PI / 2.0))
    );
    let (high, low) = if center_a.y > center_b.y { (center_b, center_a) } else { (center_a, center_b) };

    // get cardinality if possible
    let mut card_type: Option<(CardType, u32)> = None;
    match &a.object_type {
        ObjectType::Relationship { card } => { card_type = Some((card.clone(), a.id)); }
        ObjectType::RelationshipDependent { card } => { card_type = Some((card.clone(), a.id)); }
        _ => {}
    }
    match &b.object_type {
        ObjectType::Relationship { card } => { card_type = Some((card.clone(), b.id)); }
        ObjectType::RelationshipDependent { card } => { card_type = Some((card.clone(), b.id)); }
        _ => {}
    }

    // draw cardinality if necessary
    if let Some((card, card_id)) = card_type {
        let font_id = FontId { size: 14.0, family: egui::FontFamily::Monospace };

        let card = if !card_ids.contains(&card_id) {
            match card {
                CardType::OneToOne => "1",
                CardType::OneToMany => "1",
                CardType::ManyToOne => "N",
                CardType::ManyToMany => "N"
            }
        } else {
            match card {
                CardType::OneToOne => "1",
                CardType::OneToMany => "N",
                CardType::ManyToOne => "1",
                CardType::ManyToMany => "N"
            }
        };
        card_ids.push(card_id);

        // draw
        ui.fonts(|fonts| {
            shapes.push(Shape::text(    
                fonts,        
                [
                    high.x * 10.0 + center.x,
                    high.y * 10.0 + center.y
                ].into(),
                Align2::CENTER_CENTER, 
                card, 
                font_id, 
                Color32::BLACK
            ));
        });
    }

    // draw min, max if necessary
    if !minmax.is_empty() {
        // draw
        ui.fonts(|fonts| {
            let font_id = FontId { size: 14.0, family: egui::FontFamily::Monospace };
            let width = minmax.char_indices().into_iter().map(|(_, char)| fonts.glyph_width(&font_id, char)).sum::<f32>() / 2.0;

            shapes.push(Shape::text(    
                fonts,        
                [
                    low.x * width + center.x,
                    low.y * width + center.y
                ].into(),
                Align2::CENTER_CENTER, 
                minmax, 
                font_id, 
                Color32::BLACK
            ));
        });
    }

    // draw unions if necessary
    let mut union_info: Option<(u32, bool, f32)> = None;
    match &a.object_type {
        ObjectType::Polymorph { poly } => {
            union_info = Some((
                a.id,
                matches!(poly, Polymorph::Union),
                a_to_b
            ));
        }
        _ => {}
    }
    match &b.object_type {
        ObjectType::Polymorph { poly } => {
            union_info = Some((
                b.id,
                matches!(poly, Polymorph::Union),
                b_to_a
            ));
        }
        _ => {}
    }
    if let Some((union_id, invert, direction)) = union_info {
        let is_first = union_ids.contains(&union_id);
        union_ids.push(union_id);
        let draw_u = if invert { !is_first } else { is_first };

        // draw U
        if draw_u {
            // calculate offset
            let offset_line = [
                f32::cos(direction) * 8.0,
                f32::sin(direction) * 8.0
            ];

            // draw side lines
            shapes.push(Shape::LineSegment { 
                points: [
                    [
                        center_a.x * 8.0 + offset_line[0] + center.x,
                        center_a.y * 8.0 + offset_line[1] + center.y,
                    ].into(),
                    [
                        center_a.x * 8.0 + -offset_line[0] + center.x,
                        center_a.y * 8.0 + -offset_line[1] + center.y,
                    ].into(),
                ], 
                stroke: Stroke { width: 2.0, color: Color32::BLACK } 
            });
            shapes.push(Shape::LineSegment { 
                points: [
                    [
                        center_b.x * 8.0 + offset_line[0] + center.x,
                        center_b.y * 8.0 + offset_line[1] + center.y,
                    ].into(),
                    [
                        center_b.x * 8.0 + -offset_line[0] + center.x,
                        center_b.y * 8.0 + -offset_line[1] + center.y,
                    ].into(),
                ], 
                stroke: Stroke { width: 2.0, color: Color32::BLACK } 
            });

            // draw arc
            shapes.push(Shape::QuadraticBezier(egui::epaint::QuadraticBezierShape { 
                points: [
                    [
                        center_b.x * 8.0 + -offset_line[0] + center.x,
                        center_b.y * 8.0 + -offset_line[1] + center.y,
                    ].into(),
                    [
                        offset_line[0] * -2.0 + center.x,
                        offset_line[1] * -2.0 + center.y
                    ].into(),
                    [
                        center_a.x * 8.0 + -offset_line[0] + center.x,
                        center_a.y * 8.0 + -offset_line[1] + center.y,
                    ].into(),
                ],
                closed: false,
                fill: Color32::TRANSPARENT, 
                stroke: Stroke { width: 2.0, color: Color32::BLACK }
            }));
        }
    }

    shapes
}

// gets a point around an object by an angle
pub fn get_point_around_object(
    object: &Object,
    rad: f32,
    state: &AppState
) -> Vec2 {
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
            vec2(
                direction.x * mult + center.x,
                direction.y * mult + center.y
            )
        },

        // get edge on relationship diamond
        ObjectType::Relationship { .. } |
        ObjectType::RelationshipDependent { .. } => {
            let x_part = (-rad.abs() + (PI / 2.0)) / (PI / 2.0);
            let a_part = rad / (PI / 2.0);
            let y_part = if a_part >= 0.0 { -(-a_part + 1.0).abs() + 1.0 } else { (-(a_part + 1.0).abs() + 1.0) * -1.0 };
            vec2(
                (object.width / 2.0) * x_part + center.x,
                (object.width / 2.0) * y_part + center.y
            )
        },

        // get edge on parameter circle
        ObjectType::Parameter { .. } |
        ObjectType::FunctionParameter { .. } => Vec2 { 
            x: rad.cos() * (object.width / 2.0) + center.x, 
            y: rad.sin() * (object.height / 2.0) + center.y
        },
        
        ObjectType::Polymorph { .. } => Vec2 { 
            x: rad.cos() * 15.0 + center.x, 
            y: rad.sin() * 15.0 + center.y
        }
    }
    
}