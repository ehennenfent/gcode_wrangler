use crate::{
    GCode, LinearDraw, LinearMove, MachineType, Pause, Position, SetCurrentPosition,
    SetPositionMode, SetUnits, SetXY, Operation
};

impl<T: MachineType> GCode<T> for SetXY {
    fn to_op(self) -> Operation {
        Operation::SetXY { op: self }
    }

    fn render(&self) -> String {
        "G17".to_string()
    }
}

impl<T: MachineType> GCode<T> for LinearDraw {
    fn to_op(self) -> Operation {
        Operation::LinearDraw { op: self }
    }

    fn render(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        parts.push("G1".to_string());
        if let Some(feedrate) = self.feedrate {
            parts.push(format!("F{}", feedrate))
        }
        parts.push(self.target.to_string().unwrap());
        parts.join(" ")
    }
}

impl<T: MachineType> GCode<T> for LinearMove {
    fn to_op(self) -> Operation {
        Operation::LinearMove { op: self }
    }

    fn render(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        parts.push("G0".to_string());
        if let Some(feedrate) = self.feedrate {
            parts.push(format!("F{}", feedrate))
        }
        parts.push(self.target.to_string().unwrap());
        parts.join(" ")
    }
}

impl<T: MachineType> GCode<T> for Pause {
    fn to_op(self) -> Operation {
        Operation::Pause { op: self }
    }

    fn render(&self) -> String {
        format!("G4 P{}", self.ms)
    }
}

impl<T: MachineType> GCode<T> for SetCurrentPosition {
    fn to_op(self) -> Operation {
        Operation::SetCurrentPosition { op: self }
    }

    fn render(&self) -> String {
        format!("G92 {}", self.current.to_string().unwrap())
    }
}

impl<T: MachineType> GCode<T> for SetPositionMode {
    fn to_op(self) -> Operation {
        Operation::SetPositionMode { op: self }
    }

    fn render(&self) -> String {
        match self.positioning {
            Position::Absolute => "G90",
            Position::Relative => "G91",
        }
        .to_string()
    }
}

impl<T: MachineType> GCode<T> for SetUnits {
    fn to_op(self) -> Operation {
        Operation::SetUnits { op: self }
    }

    fn render(&self) -> String {
        match self.units {
            crate::Units::Inches => "G20",
            crate::Units::Millimeters => "G21",
        }
        .to_string()
    }
}
