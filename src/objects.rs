use egui::{Ui, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Objects {
    pub objects: Vec<Object>
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
        aspect: f32,
        mouse_position: &Vec2
    ) {

    }
}
