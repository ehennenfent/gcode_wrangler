use crate::{
    Activate, Deactivate, EndProgram, GCode, Home, LinearMove,
    MachineType, StepperControl,
    Vec3,
};

struct Marlin;
impl MachineType for Marlin {}

impl GCode<Marlin> for Activate {
    fn render(&self) -> String {
        <LinearMove as GCode<Marlin>>::render(&LinearMove {
            target: Vec3 {
                x: Some(0.0),
                y: Some(0.0),
                z: Some(-5.0),
            },
            feedrate: None,
        })
    }
}

impl GCode<Marlin> for Deactivate {
    fn render(&self) -> String {
        <LinearMove as GCode<Marlin>>::render(&LinearMove {
            target: Vec3 {
                x: Some(0.0),
                y: Some(0.0),
                z: Some(5.0),
            },
            feedrate: None,
        })
    }
}

impl GCode<Marlin> for Home {
    fn render(&self) -> String {
        let mut parts: Vec<&str> = vec!["G28"];
        if self.x {
            parts.push("X")
        };
        if self.y {
            parts.push("Y")
        };
        if self.z {
            parts.push("Z")
        };

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

impl GCode<Marlin> for EndProgram {
    fn render(&self) -> String {
        "M5".to_string()
    }
}
