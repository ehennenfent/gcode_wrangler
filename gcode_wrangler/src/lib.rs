use serde::{Deserialize, Serialize};

pub mod models;

#[derive(Serialize, Clone)]
pub enum Flavor {
    GRBL,
    Marlin,
}

#[derive(Debug)]
pub enum Position {
    Absolute,
    Relative,
}

#[derive(Debug)]
pub enum Units {
    Inches,
    Millimeters,
}

#[derive(Debug)]
pub enum StepperState {
    Enabled,
    Disabled,
}

pub struct SimulationState {
    real_position: Vec3,
    virtual_position: Vec3,
    max_travel: Vec3,
    min_travel: Vec3,
    positioning: Position,
    feedrate: u32,
    active: bool,
}

#[derive(Debug)]
pub struct Vec3 {
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
}

impl Vec3 {
    fn to_string(&self) -> Result<String, &'static str> {
        match (self.x, self.y, self.z) {
            (None, None, None) => Err("At least one dimension must be provided"),
            (_, _, _) => {
                let mut parts: Vec<String> = Vec::new();
                if let Some(x) = self.x {
                    parts.push(format!("X{}", x))
                }
                if let Some(y) = self.y {
                    parts.push(format!("Y{}", y))
                }
                if let Some(z) = self.z {
                    parts.push(format!("Z{}", z))
                }
                Ok(parts.join(" "))
            }
        }
    }
}

#[derive(Debug)]
pub enum GCode {
    Activate,
    Deactivate,
    EndProgram,
    SetXY,
    Pause(u32),
    SetPositionMode(Position),
    SetCurrentPosition(Vec3),
    SetUnits(Units),

    LinearMove {
        target: Vec3,
        feedrate: Option<u32>,
    },

    LinearDraw {
        target: Vec3,
        feedrate: Option<u32>,
    },

    Home {
        x: bool,
        y: bool,
        z: bool,
    },

    StepperControl {
        x: StepperState,
        y: StepperState,
        z: StepperState,
    },
}

impl GCode {
    fn render(&self, flavor: &Flavor) -> String {
        match self {
            GCode::Activate => match flavor {
                Flavor::GRBL => "M3 S254\nG4 P300".to_string(),
                Flavor::Marlin => GCode::LinearMove {
                    target: Vec3 {
                        x: Some(0.0),
                        y: Some(0.0),
                        z: Some(-5.0),
                    },
                    feedrate: None,
                }
                .render(flavor),
            },
            GCode::Deactivate => match flavor {
                Flavor::GRBL => "M3 S65\nG4 P300".to_string(),
                Flavor::Marlin => GCode::LinearMove {
                    target: Vec3 {
                        x: Some(0.0),
                        y: Some(0.0),
                        z: Some(5.0),
                    },
                    feedrate: None,
                }
                .render(flavor),
            },
            GCode::EndProgram => match flavor {
                Flavor::GRBL => "M5\nM2".to_string(),
                Flavor::Marlin => "M5".to_string(),
            },
            GCode::Home { x, y, z } => match flavor {
                Flavor::GRBL => "$H".to_string(),
                Flavor::Marlin => {
                    let mut parts: Vec<&str> = vec!["G28"];
                    if *x {
                        parts.push("X")
                    };
                    if *y {
                        parts.push("Y")
                    };
                    if *z {
                        parts.push("Z")
                    };

                    parts.join(" ")
                }
            },
            GCode::StepperControl { x, y, z } => match flavor {
                Flavor::GRBL => unimplemented!(),
                Flavor::Marlin => {
                    let mut enable: Vec<&str> = vec!["M17"];
                    let mut disable: Vec<&str> = vec!["M18"];

                    match x {
                        crate::StepperState::Enabled => enable.push("X"),
                        crate::StepperState::Disabled => disable.push("X"),
                    }
                    match y {
                        crate::StepperState::Enabled => enable.push("Y"),
                        crate::StepperState::Disabled => disable.push("Y"),
                    }
                    match z {
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
            },
            GCode::SetXY => "G17".to_string(),
            GCode::Pause(ms) => {
                format!("G4 P{}", ms)
            }
            GCode::SetPositionMode(pmode) => match pmode {
                Position::Absolute => "G90",
                Position::Relative => "G91",
            }
            .to_string(),
            GCode::SetCurrentPosition(current) => {
                format!("G92 {}", current.to_string().unwrap())
            }
            GCode::SetUnits(units) => match units {
                crate::Units::Inches => "G20",
                crate::Units::Millimeters => "G21",
            }
            .to_string(),
            GCode::LinearMove { target, feedrate } => {
                let mut parts: Vec<String> = Vec::new();
                parts.push("G0".to_string());
                if let Some(feedrate) = feedrate {
                    parts.push(format!("F{}", feedrate))
                }
                parts.push(target.to_string().unwrap());
                parts.join(" ")
            }
            GCode::LinearDraw { target, feedrate } => {
                let mut parts: Vec<String> = Vec::new();
                parts.push("G1".to_string());
                if let Some(feedrate) = feedrate {
                    parts.push(format!("F{}", feedrate))
                }
                parts.push(target.to_string().unwrap());
                parts.join(" ")
            }
        }
    }
}
