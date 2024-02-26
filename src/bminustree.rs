use serde::{Deserialize, Serialize};

pub const NODE_COUNT: usize = 3;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BMinusNode {
    pub keys: Vec<String>,
    pub children: Vec<BMinusNode>
}

impl BMinusNode {
    pub fn insert(&mut self, _key: impl Into<String>) {
        todo!();
    }

    pub fn debug(&self) {
        self.debug_internal(String::new());
    }

    fn debug_internal(&self, header: String) {
        print!("{header} [");
        self.keys.iter().enumerate().for_each(|a| {
            print!("{}", a.1);
            if a.0 < NODE_COUNT - 1 { print!(", ") }
        });
        println!("{header} ]");
        let header = format!("{header} - ");
        self.children.iter().for_each(|a| a.debug_internal(header.clone()));
    }
}