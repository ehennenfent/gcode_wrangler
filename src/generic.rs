use crate::{
    GCode, LinearDraw, LinearMove, MachineType, Pause, Position, SetCurrentPosition,
    SetPositionMode, Home, SetUnits, Vec3, Activate, Deactivate, StepperControl, EmergencyStop, EndProgram, SetXY,
};

impl<T: MachineType> GCode<T> for SetXY {
    fn render(&self) -> String {
        "G17".to_string()
    }
}