use crate::{
    GCode, LinearDraw, LinearMove, MachineType, Pause, Position, SetCurrentPosition,
    SetPositionMode, Home, SetUnits, Vec3, Activate, Deactivate, StepperControl, EmergencyStop, EndProgram,
};

struct Grbl;
impl MachineType for Grbl {}

impl GCode<Grbl> for LinearDraw {
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

impl GCode<Grbl> for LinearMove {
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

impl GCode<Grbl> for Pause {
    fn render(&self) -> String {
        format!("G4 P{}", self.ms)
    }
}

impl GCode<Grbl> for SetCurrentPosition {
    fn render(&self) -> String {
        format!("G92 {}", self.current.to_string().unwrap())
    }
}

impl GCode<Grbl> for SetPositionMode {
    fn render(&self) -> String {
        match self.positioning {
            Position::Absolute => "G90",
            Position::Relative => "G91",
        }
        .to_string()
    }
}

impl GCode<Grbl> for Activate {
    fn render(&self) -> String {
        "M3 S254\nG4 P300".to_string()    }
}

impl GCode<Grbl> for Deactivate {
    fn render(&self) -> String {
        "M3 S65\nG4 P300".to_string()
    }
}

impl GCode<Grbl> for SetUnits {
    fn render(&self) -> String {
        match self.units {
            crate::Units::Inches => "G20",
            crate::Units::Millimeters => "G21",
        }.to_string()
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