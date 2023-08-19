pub mod generic;
pub mod grbl;
pub mod marlin;
pub mod models;

pub trait MachineType {
    fn preamble() -> Vec<Operation>;
}

pub trait GCode<T: MachineType> {
    fn render(&self) -> String;

    // I don't really like this pattern, but without specialization it's the easiest way to
    // pass around compatible objects
    fn to_op(self) -> Operation;
}

pub enum Position {
    Absolute,
    Relative,
}

pub enum Units {
    Inches,
    Millimeters,
}

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
                parts.push("G92".to_string());
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

pub struct LinearMove {
    target: Vec3,
    feedrate: Option<u32>,
}

pub struct LinearDraw {
    target: Vec3,
    feedrate: Option<u32>,
}

pub struct Pause {
    ms: u32,
}

pub struct SetPositionMode {
    positioning: Position,
}

pub struct SetCurrentPosition {
    current: Vec3,
}

pub struct Activate {}
pub struct Deactivate {}

pub struct SetUnits {
    units: Units,
}

pub struct Home {
    x: bool,
    y: bool,
    z: bool,
}

pub struct StepperControl {
    x: StepperState,
    y: StepperState,
    z: StepperState,
}

pub struct EndProgram {}

pub struct SetXY {}

pub enum Operation {
    LinearMove {op: LinearMove},
    LinearDraw {op: LinearDraw},
    Pause {op: Pause},
    SetPositionMode {op: SetPositionMode},
    SetCurrentPosition {op: SetCurrentPosition},
    Activate {op: Activate},
    Deactivate {op: Deactivate},
    SetUnits {op: SetUnits},
    Home {op: Home},
    StepperControl {op: StepperControl},
    EndProgram {op: EndProgram},
    SetXY {op: SetXY},
}