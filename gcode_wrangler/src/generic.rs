use crate::{
    GCode, LinearDraw, LinearMove, MachineType, Pause, Position, SetCurrentPosition,
    SetPositionMode, SetUnits, SetXY,
};

impl<T: MachineType> GCode<T> for SetXY {
    fn render(&self) -> String {
        "G17".to_string()
    }
}

impl<T: MachineType> GCode<T> for LinearDraw {
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
    fn render(&self) -> String {
        format!("G4 P{}", self.ms)
    }
}

impl<T: MachineType> GCode<T> for SetCurrentPosition {
    fn render(&self) -> String {
        format!("G92 {}", self.current.to_string().unwrap())
    }
}

impl<T: MachineType> GCode<T> for SetPositionMode {
    fn render(&self) -> String {
        match self.positioning {
            Position::Absolute => "G90",
            Position::Relative => "G91",
        }
        .to_string()
    }
}

impl<T: MachineType> GCode<T> for SetUnits {
    fn render(&self) -> String {
        match self.units {
            crate::Units::Inches => "G20",
            crate::Units::Millimeters => "G21",
        }
        .to_string()
    }
}
