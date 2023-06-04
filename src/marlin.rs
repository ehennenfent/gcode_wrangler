use crate::{
    GCode, LinearDraw, LinearMove, MachineType, Pause, Position, SetCurrentPosition,
    SetPositionMode, Home, SetUnits, Vec3, Activate, Deactivate, StepperControl, EmergencyStop, EndProgram,
};

struct Marlin;
impl MachineType for Marlin {}

impl GCode<Marlin> for LinearDraw {
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

impl GCode<Marlin> for LinearMove {
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

impl GCode<Marlin> for Pause {
    fn render(&self) -> String {
        format!("G4 P{}", self.ms)
    }
}

impl GCode<Marlin> for SetCurrentPosition {
    fn render(&self) -> String {
        format!("G92 {}", self.current.to_string().unwrap())
    }
}

impl GCode<Marlin> for SetPositionMode {
    fn render(&self) -> String {
        match self.positioning {
            Position::Absolute => "G90",
            Position::Relative => "G91",
        }
        .to_string()
    }
}

impl GCode<Marlin> for Activate {
    fn render(&self) -> String {
        <LinearMove as GCode<Marlin>>::render(&LinearMove{target: Vec3{x: Some(0.0), y: Some(0.0), z: Some(-5.0)}, feedrate: None})
    }
}

impl GCode<Marlin> for Deactivate {
    fn render(&self) -> String {
        <LinearMove as GCode<Marlin>>::render(&LinearMove{target: Vec3{x: Some(0.0), y: Some(0.0), z: Some(5.0)}, feedrate: None})
    }
}

impl GCode<Marlin> for SetUnits {
    fn render(&self) -> String {
        match self.units {
            crate::Units::Inches => "G20",
            crate::Units::Millimeters => "G21",
        }.to_string()
    }
}

impl GCode<Marlin> for Home {
    fn render(&self) -> String {
        let mut parts: Vec<&str> = vec!["G28"];
        if self.x {parts.push("X")};
        if self.y {parts.push("Y")};
        if self.z {parts.push("Z")};

        parts.join(" ")
    }
}

impl GCode<Marlin> for StepperControl {
    fn render(&self) -> String {
        let mut enable: Vec<&str> = vec!["M17"];
        let mut disable: Vec<&str> = vec!["M18"];

        match self.x {
            crate::StepperState::Enabled => enable.push("X"),
            crate::StepperState::Disabled => disable.push("X"),
        }
        match self.y {
            crate::StepperState::Enabled => enable.push("Y"),
            crate::StepperState::Disabled => disable.push("Y"),
        }
        match self.z {
            crate::StepperState::Enabled => enable.push("Z"),
            crate::StepperState::Disabled => disable.push("Z"),
        }

        let mut lines: Vec<String> = Vec::new();

        if disable.len() > 1 {
            lines.push(disable.join(" "))
        }
        if enable.len() > 1 {
            lines.push(enable.join(" "))
        }

        lines.join("\n")
    }
}

impl GCode<Marlin> for EmergencyStop {
    fn render(&self) -> String {
        "M112".to_string()
    }
}

impl GCode<Marlin> for EndProgram {
    fn render(&self) -> String {
        "M5".to_string()
    }
}