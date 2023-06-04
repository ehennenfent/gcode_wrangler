fn main() {
    println!("Test sync v2!");
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

enum Position {
    Absolute,
    Relative,
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

trait GCode {
    fn render(&self) -> String;
}

impl GCode for LinearDraw {
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

impl GCode for LinearMove {
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

impl GCode for Pause {
    fn render(&self) -> String {
        format!("G4 P{}", self.ms)
    }
}

struct SetPositionMode {
    positioning: Position,
}

impl GCode for SetPositionMode {
    fn render(&self) -> String {
        match self.positioning {
            Position::Absolute => "G90",
            Position::Relative => "G91",
        }
        .to_string()
    }
}

struct SetCurrentPosition {
    current: Vec3,
}

impl GCode for SetCurrentPosition {
    fn render(&self) -> String {
        format!("G92 {}", self.current.to_string().unwrap())
    }
}
