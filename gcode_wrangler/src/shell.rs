use serialport::{Error, SerialPort};

use std::thread;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use std::io;
use std::io::BufRead;

pub enum PortCmd {
    RUN,
    PAUSE,
    STOP,
}

pub struct SerialChannel {
    _sender: Sender<String>,
    receiver: Receiver<String>,
    command: Receiver<PortCmd>,
    status: PortCmd,
    port: Box<dyn SerialPort>,
}

impl SerialChannel {
    pub fn new(
        port_name: &str,
        baud_rate: u32,
    ) -> Result<(Sender<String>, Receiver<String>, Sender<PortCmd>, Self), Error> {
        match serialport::new(port_name, baud_rate).open() {
            Ok(port) => {
                let (inbound_tx, inbound_rx) = mpsc::channel(1024);
                let (outbound_tx, outbound_rx) = mpsc::channel(1024);
                let (cmd_tx, cmd_rx) = mpsc::channel(1024);

                Ok((
                    inbound_tx,
                    outbound_rx,
                    cmd_tx,
                    SerialChannel {
                        _sender: outbound_tx,
                        receiver: inbound_rx,
                        command: cmd_rx,
                        status: PortCmd::RUN,
                        port: port,
                    },
                ))
            }
            Err(e) => Err(e),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(cmd) = self.command.try_recv() {
                self.status = cmd;
            }
            match self.status {
                PortCmd::RUN => {
                    match self.port.bytes_to_read() {
                        Ok(0) => (),
                        Ok(n) => {
                            let mut buffer: Vec<u8> = vec![0; n as usize];
                            self.port.read_exact(&mut buffer).unwrap();
                            print!("{}", String::from_utf8_lossy(&buffer));
                        }
                        Err(_) => println!("Failed to read"),
                    }

                    if let Ok(msg) = self.receiver.try_recv() {
                        self.port
                            .write_all(format!("{}\n", msg).as_bytes())
                            .unwrap()
                    }
                }
                PortCmd::PAUSE => (),
                PortCmd::STOP => break,
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let maybe_channel = SerialChannel::new("/dev/ttyACM1", 115200);
    let (tx, mut rx, cmd, mut channel) = maybe_channel.expect("failed to open serial port");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    thread::spawn(move || channel.run());

    loop {
        if let Ok(line) = rx.try_recv() {
            println!("Received {}", line);
        }
        if let Some(l) = lines.next() {
            match l {
                Ok(line) => tx.send(line).await.unwrap(),
                Err(_) => {
                    println!("Exiting");
                    break;
                }
            }
        }
    }
    cmd.send(PortCmd::STOP).await.unwrap();

    Ok(())
}
