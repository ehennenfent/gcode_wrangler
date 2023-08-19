use crate::{Activate, Deactivate, EndProgram, GCode, Home, MachineType, Operation};

struct Grbl;
impl MachineType for Grbl {
    fn preamble() -> Vec<Operation> {
        vec![]
    }
}

impl GCode<Grbl> for Activate {
    fn to_op(self) -> Operation {
        Operation::Activate { op: self }
    }

    fn render(&self) -> String {
        "M3 S254\nG4 P300".to_string()
    }
}

impl GCode<Grbl> for Deactivate {
    fn to_op(self) -> Operation {
        Operation::Deactivate { op: self }
    }

    fn render(&self) -> String {
        "M3 S65\nG4 P300".to_string()
    }
}

impl GCode<Grbl> for Home {
    fn to_op(self) -> Operation {
        Operation::Home { op: self }
    }

    fn render(&self) -> String {
        "$H".to_string()
    }
}

impl GCode<Grbl> for EndProgram {
    fn to_op(self) -> Operation {
        Operation::EndProgram { op: self }
    }

    fn render(&self) -> String {
        "M5\nM2".to_string()
    }
}
