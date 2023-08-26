use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::hash::{Hash, Hasher};

use crate::Flavor;

#[derive(Clone, Deserialize, Default, Serialize, Copy, Debug)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Hash for Vec2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_le_bytes().hash(state);
        self.y.to_le_bytes().hash(state);
    }
}

#[derive(Deserialize, Hash, Default)]
pub struct Movement {
    pub dest: Vec2D,
    pub pen_down: bool,
}

#[derive(Clone, Serialize)]
pub struct MachineDetails {
    pub dimensions: Vec2D,
    pub flavor: Flavor,
    pub device: String,
    pub port: String,
    pub baud_rate: u32,
}

impl From<HashMap<String, String>> for MachineDetails {
    fn from(fromval: HashMap<String, String>) -> Self {
        MachineDetails {
            dimensions: Vec2D {
                x: fromval
                    .get("xdim")
                    .expect("Missing config value: xdim")
                    .parse()
                    .unwrap(),
                y: fromval
                    .get("ydim")
                    .expect("Missing config value: ydim")
                    .parse()
                    .unwrap(),
            },
            flavor: {
                if let Some(maybe_match) = fromval.get("flavor") {
                    match maybe_match.as_str() {
                        "GRBL" => Flavor::GRBL,
                        "Marlin" => Flavor::Marlin,
                        unknown => {
                            panic!("Unknown gcode flavor: {unknown}")
                        }
                    }
                } else {
                    panic!("Missing config value: flavor")
                }
            },
            device: fromval
                .get("name")
                .expect("Missing config value: name")
                .to_owned(),
            port: fromval
                .get("port")
                .expect("Missing config value: port")
                .to_owned(),
            baud_rate: fromval
                .get("baud_rate")
                .expect("Missing config calue: baud_rate")
                .parse()
                .unwrap(),
        }
    }
}
