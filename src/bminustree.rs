use serde::{Deserialize, Serialize};

pub const NODE_COUNT: usize = 3;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BMinusNode {
    pub keys: Vec<String>,
    pub children: Vec<BMinusNode>
}

impl BMinusNode {
    pub fn create(input: Vec<String>) -> Self {
        match input.len() {
            0 => panic!("Must be given input!"),
            1 ..= 3 => Self { keys: input, children: vec![] },
            4 ..= 7 => {
                let mut mid = input.len() / 2;
                if input.len() % 2 == 0 { mid -= 1; }
                println!("Mid {mid}");
                Self {
                    keys: vec![input[mid].clone()],
                    children: vec![
                        BMinusNode::create(input[0..mid].to_vec()),
                        BMinusNode::create(input[mid + 1 .. input.len()].to_vec())
                    ]
                }
            },
            8 ..= 10 => {
                let (a, b) = match input.len() {
                    8 => (1, 4),
                    9 => (1, 5),
                    10 => (2, 6),
                    _ => panic!("RUN")
                };

                Self {
                    keys: vec![input[a].clone(), input[b].clone()],
                    children: vec![
                        BMinusNode::create(input[0 .. a].to_vec()),
                        BMinusNode::create(input[a + 1 .. b].to_vec()),
                        BMinusNode::create(input[b + 1 .. input.len()].to_vec()),
                    ]
                }
            },
            _ => {
                let (a, b, c) = match input.len() {
                    11 => (1, 3, 6),
                    _ => panic!("RUN")
                };

                Self {
                    keys: vec![input[a].clone(), input[b].clone(), input[c].clone()],
                    children: vec![
                        BMinusNode::create(input[0 .. a].to_vec()),
                        BMinusNode::create(input[a + 1 .. b].to_vec()),
                        BMinusNode::create(input[b + 1 .. c].to_vec()),
                        BMinusNode::create(input[c + 1 .. input.len()].to_vec()),
                    ]
                }
            }
        }
    }

    pub fn debug(&self) {
        self.debug_internal(String::new());
    }

    fn debug_internal(&self, header: String) {
        print!("{header}[");
        self.keys.iter().enumerate().for_each(|a| {
            print!("{}", a.1.replace("\r", "").replace("\n", ""));
            if a.0 < NODE_COUNT - 1 { print!(", ") }
        });
        println!("]");
        let header = format!("{header} - ");
        self.children.iter().for_each(|a| a.debug_internal(header.clone()));
    }
}