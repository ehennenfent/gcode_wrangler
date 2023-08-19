use serde::{Deserialize, Serialize};
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use crate::Flavor;

#[derive(Clone, Deserialize, Default, Serialize)]
pub struct Vec2D {
    x: f32,
    y: f32,
}

impl Hash for Vec2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_le_bytes().hash(state);
        self.y.to_le_bytes().hash(state);
    }
}

#[derive(Deserialize, Hash, Default)]
pub struct Movement {
    dest: Vec2D,
    pen_down: bool,
}

#[derive(Clone, Serialize)]
pub struct MachineDetails {
    dimensions: Vec2D,
    flavor: Flavor,
    device: String,
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
        }
    }
}
