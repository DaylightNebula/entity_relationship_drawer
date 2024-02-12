use egui::Color32;
use egui_plot::{Line, PlotUi};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Objects {
    pub objects: Vec<Object>,
    pub next_id: u32
}

impl Objects {
    pub fn add(&mut self, object_type: ObjectType, x: f64, y: f64) -> &mut Object {
        self.objects.push(Object { id: self.next_id, x, y, width: 0.0, height: 0.0, name: String::new(), object_type });
        self.next_id += 1;
        self.objects.iter_mut().last().expect("Physics just broke")
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Object {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
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
        ui: &mut PlotUi,
        state: &AppState
    ) {
        // update widths and heights
        let text_width = 0.0;
        let text_height = 0.0;
        self.width = text_width + 0.05;
        self.height = text_height + 0.05;

        match self.object_type {
            ObjectType::Entity => {
                ui.line(Line::new(vec![
                    [-self.width / 2.0 + self.x, -self.height / 2.0 + self.y],
                    [self.width / 2.0 + self.x, -self.height / 2.0 + self.y],
                    [self.width / 2.0 + self.x, self.height / 2.0 + self.y],
                    [-self.width / 2.0 + self.x, self.height / 2.0 + self.y],
                    [-self.width / 2.0 + self.x, -self.height / 2.0 + self.y],
                ]).color(Color32::BLACK));
            },
            ObjectType::Relationship => todo!(),
            ObjectType::Parameter => todo!(),
            ObjectType::EntityDependent => todo!(),
            ObjectType::RelationshipDependent => todo!(),
            ObjectType::FunctionParameter => todo!(),
        }
    }
}
