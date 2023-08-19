use crate::{Activate, Deactivate, EndProgram, GCode, Home, MachineType};

struct Grbl;
impl MachineType for Grbl {
    fn preamble() -> Vec<Box<dyn GCode<Self>>> {
        vec![]
    }
}

impl GCode<Grbl> for Activate {
    fn render(&self) -> String {
        "M3 S254\nG4 P300".to_string()
    }
}

impl GCode<Grbl> for Deactivate {
    fn render(&self) -> String {
        "M3 S65\nG4 P300".to_string()
    }
}

impl GCode<Grbl> for Home {
    fn render(&self) -> String {
        "$H".to_string()
    }
}

impl GCode<Grbl> for EndProgram {
    fn render(&self) -> String {
        "M5\nM2".to_string()
    }
}
