use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Objects {
    pub objects: Vec<Object>,
    pub links: Vec<Link>,
    pub next_id: u32
}

impl Objects {
    // adds a object to this state
    pub fn add(&mut self, object_type: ObjectType, x: f32, y: f32) -> &mut Object {
        self.objects.push(Object { id: self.next_id, x, y, width: 0.0, height: 0.0, name: String::new(), object_type, dragging: false });
        self.next_id += 1;
        self.objects.iter_mut().last().expect("Physics just broke")
    }

    // removes a link between A and B
    pub fn remove_link(&mut self, a: u32, b: u32) -> bool {
        let idx = self.links.iter().position(|link| (link.a == a || link.a == b) && (link.b == a || link.b == b));
        if idx.is_some() {
            self.links.remove(idx.unwrap());
            true
        } else { false }
    }

    // creates a link between A and B
    pub fn link(&mut self, a: u32, b: u32) { self.links.push(Link { a, b }); }

    // gets a link with the given node
    pub fn get_link(&self, node: u32) -> Option<&Link> { self.links.iter().find(|a| a.a == node || a.b == node) }
    pub fn get_link_mut(&mut self, node: u32) -> Option<&mut Link> { self.links.iter_mut().find(|a| a.a == node || a.b == node) }
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

impl ObjectType {
    pub fn use_double_link(&self) -> bool {
        match self {
            ObjectType::Entity => false,
            ObjectType::Relationship => false,
            ObjectType::Parameter => false,
            ObjectType::EntityDependent => true,
            ObjectType::RelationshipDependent => true,
            ObjectType::FunctionParameter => false,
            ObjectType::KeyParameter => false
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Link {
    pub a: u32,
    pub b: u32
}
