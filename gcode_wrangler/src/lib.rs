use pyo3::prelude::*;

pub mod generic;
pub mod grbl;
pub mod marlin;

trait MachineType {}

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

#[pyclass]
struct LinearMove {
    target: Vec3,
    feedrate: Option<u32>,
}

#[pyclass]
struct LinearDraw {
    target: Vec3,
    feedrate: Option<u32>,
}

#[pyclass]
struct Pause {
    ms: u32,
}

#[pyclass]
struct SetPositionMode {
    positioning: Position,
}

#[pyclass]
struct SetCurrentPosition {
    current: Vec3,
}

#[pyclass]
struct Activate {}
#[pyclass]
struct Deactivate {}

#[pyclass]
struct SetUnits {
    units: Units,
}

#[pyclass]
struct Home {
    x: bool,
    y: bool,
    z: bool,
}

#[pyclass]
struct StepperControl {
    x: StepperState,
    y: StepperState,
    z: StepperState,
}

#[pyclass]
struct EndProgram {}

#[pyclass]
struct SetXY {}

#[pymodule]
fn gcode_wrangler(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinearMove>()?;
    m.add_class::<LinearDraw>()?;
    m.add_class::<Pause>()?;
    m.add_class::<Activate>()?;
    m.add_class::<Deactivate>()?;

    Ok(())
}