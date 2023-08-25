use models::{MachineDetails, Movement, Vec2D};
use serde::Serialize;
use std::ops::Add;
use std::{thread, time};

use tokio::sync::mpsc::{Receiver as SingleReceiver, Sender as MultiSender};
use tokio::sync::watch::{Receiver, Sender};
use tokio::sync::{mpsc, watch};

use serialport::{Error, SerialPort};

pub mod models;

#[derive(Serialize, Clone)]
pub enum Flavor {
    GRBL,
    Marlin,
}

impl Flavor {
    fn render(&self, operations: &Vec<GCode>) -> Vec<String> {
        operations.into_iter().map(|op| op.render(self)).collect()
    }
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

// pub struct SimulationState {
//     real_position: Vec3,
//     virtual_position: Vec3,
//     max_travel: Vec3,
//     min_travel: Vec3,
//     positioning: Position,
//     feedrate: u32,
//     active: bool,
// }

#[derive(Debug, Clone, Copy)]
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

    pub fn preamble(flavor: &Flavor) -> Vec<GCode> {
        match flavor {
            Flavor::GRBL => vec![
                GCode::SetUnits(Units::Millimeters),
                GCode::SetPositionMode(Position::Absolute),
                GCode::SetCurrentPosition(Vec3 {
                    x: Some(0.0),
                    y: Some(0.0),
                    z: Some(0.0),
                }),
            ],
            Flavor::Marlin => todo!(),
        }
    }

    pub fn footer(flavor: &Flavor) -> Vec<GCode> {
        match flavor {
            Flavor::GRBL => vec![
                GCode::Deactivate,
                GCode::LinearMove {
                    target: Vec3 {
                        x: Some(0.0),
                        y: Some(0.0),
                        z: None,
                    },
                    feedrate: None,
                },
                GCode::EndProgram,
            ],
            Flavor::Marlin => todo!(),
        }
    }
}

impl From<Vec2D> for Vec3 {
    fn from(value: Vec2D) -> Self {
        Vec3 {
            x: Some(value.x),
            y: Some(value.y),
            z: None,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: match (self.x, rhs.x) {
                (None, None) => None,
                (Some(left), Some(right)) => Some(left + right),
                _ => unimplemented!(), // yes, yes, I know
            },
            y: match (self.y, rhs.y) {
                (None, None) => None,
                (Some(left), Some(right)) => Some(left + right),
                _ => unimplemented!(),
            },
            z: match (self.z, rhs.z) {
                (None, None) => None,
                (Some(left), Some(right)) => Some(left + right),
                _ => unimplemented!(),
            },
        }
    }
}

pub fn to_gcode(movements: &Vec<Movement>, position_mode: Position) -> Vec<GCode> {
    // accepts either position mode, but always produces absolute coordinates based on
    // starting at (0, 0).

    let mut active: bool = false;
    let mut position = Vec3 {
        x: Some(0.0),
        y: Some(0.0),
        z: None,
    };

    let mut as_gcode: Vec<GCode> = Vec::new();

    for mv in movements {
        let dest: Vec3 = match position_mode {
            Position::Absolute => Vec3::from(mv.dest),
            Position::Relative => position + Vec3::from(mv.dest),
        };

        match (active, mv.pen_down) {
            (true, true) => {
                as_gcode.push(GCode::LinearDraw {
                    target: dest,
                    feedrate: None,
                });
            }
            (true, false) => {
                as_gcode.push(GCode::Deactivate);
                as_gcode.push(GCode::LinearMove {
                    target: dest,
                    feedrate: None,
                });
                active = mv.pen_down;
            }
            (false, true) => {
                as_gcode.push(GCode::Activate);
                as_gcode.push(GCode::LinearDraw {
                    target: dest,
                    feedrate: None,
                });
                active = mv.pen_down;
            }
            (false, false) => {
                as_gcode.push(GCode::LinearMove {
                    target: dest,
                    feedrate: None,
                });
            }
        }

        position = dest;
    }

    as_gcode
}

pub enum PortCmd {
    WAIT,
    SEND(Vec<String>),
    RUN,
    PAUSE,
    STOP,
}

pub struct SerialChannel {
    progress: Sender<usize>,
    command: SingleReceiver<PortCmd>,
    status: PortCmd,
    port: Box<dyn SerialPort>,
    buffer: Vec<String>,
}

impl SerialChannel {
    pub fn new(
        machine: &MachineDetails,
    ) -> Result<(Receiver<usize>, MultiSender<PortCmd>, Self), Error> {
        match serialport::new(machine.port.clone(), machine.baud_rate).open() {
            Ok(port) => {
                let (cmd_tx, cmd_rx) = mpsc::channel(64);
                let (progress_tx, progress_rx) = watch::channel(0usize);
                Ok((
                    progress_rx,
                    cmd_tx,
                    SerialChannel {
                        progress: progress_tx,
                        command: cmd_rx,
                        status: PortCmd::WAIT,
                        port: port,
                        buffer: Vec::new(),
                    },
                ))
            }
            Err(e) => Err(e),
        }
    }

    pub fn run(&mut self) {
        // we're not gonna need to send gcode at more than 30fps, so we can just make
        // this thread sleep most of the time rather than futz about with async
        let thread_delay = time::Duration::from_millis(35);

        loop {
            match self.command.try_recv() {
                Ok(cmd) => match &cmd {
                    PortCmd::WAIT => {
                        self.buffer.clear();
                        self.status = PortCmd::WAIT;
                    }
                    PortCmd::SEND(cmds) => {
                        self.buffer.clear();
                        self.buffer.extend(cmds.iter().rev().cloned());
                    }
                    PortCmd::PAUSE => self.status = PortCmd::PAUSE,
                    PortCmd::STOP => {
                        self.buffer.clear();
                        self.status = PortCmd::STOP;
                        break;
                    }
                    PortCmd::RUN => self.status = PortCmd::RUN,
                },
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    panic!("Command channel closed unexpectedly")
                }
                Err(mpsc::error::TryRecvError::Empty) => (),
            }

            match &self.status {
                PortCmd::STOP => break,
                PortCmd::PAUSE => (),
                PortCmd::WAIT => (),
                PortCmd::RUN => match self.buffer.pop() {
                    Some(next_cmd) => {
                        if let Err(e) = self.port.write_all(format!("{}\n", next_cmd).as_bytes()) {
                            println!("Failed to write to serial port: {:?}", e);
                            self.status = PortCmd::WAIT
                        } else {
                            self.progress
                                .send(self.buffer.len())
                                .expect("Progress channel closed unexpectedly")
                        }
                    }
                    None => self.status = PortCmd::WAIT,
                },
                PortCmd::SEND(_) => (),
            }
            thread::sleep(thread_delay);
        }
        println!("Serial monitor exiting");
    }
}

pub fn to_program(gcode: &Vec<GCode>, flavor: Flavor) -> Vec<String> {
    let mut rendered: Vec<String> = vec![];

    rendered.extend(flavor.render(&GCode::preamble(&flavor)));
    rendered.extend(flavor.render(gcode));
    rendered.extend(flavor.render(&GCode::footer(&flavor)));

    rendered
}
