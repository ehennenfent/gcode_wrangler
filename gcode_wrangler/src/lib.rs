pub mod generic;
pub mod grbl;
pub mod marlin;

pub trait MachineType {
    fn preamble() -> Vec<Box<dyn GCode<Self>>>;
}

trait GCode<T: MachineType> {
    fn render(&self) -> String;
}

enum Position {
    Absolute,
    Relative,
}

enum Units {
    Inches,
    Millimeters,
}

enum StepperState {
    Enabled,
    Disabled,
}

struct SimulationState {
    real_position: Vec3,
    virtual_position: Vec3,
    max_travel: Vec3,
    min_travel: Vec3,
    positioning: Position,
    feedrate: u32,
    active: bool,
}

struct Vec3 {
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

struct LinearMove {
    target: Vec3,
    feedrate: Option<u32>,
}

struct LinearDraw {
    target: Vec3,
    feedrate: Option<u32>,
}

struct Pause {
    ms: u32,
}

struct SetPositionMode {
    positioning: Position,
}

struct SetCurrentPosition {
    current: Vec3,
}

struct Activate {}
struct Deactivate {}

struct SetUnits {
    units: Units,
}

struct Home {
    x: bool,
    y: bool,
    z: bool,
}

struct StepperControl {
    x: StepperState,
    y: StepperState,
    z: StepperState,
}

struct EndProgram {}

struct SetXY {}
