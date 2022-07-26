use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
    thread,
};

use bevy::prelude::{Component, KeyCode};
use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::movement::prelude::Direction;

#[derive(Component)]
pub struct RandomMoves;

#[derive(Component)]
pub struct KeyboardMoves {
    pub north: KeyCode,
    pub east: KeyCode,
    pub south: KeyCode,
    pub west: KeyCode,
}
impl KeyboardMoves {
    pub fn wasd() -> Self {
        Self {
            north: KeyCode::W,
            east: KeyCode::D,
            south: KeyCode::S,
            west: KeyCode::A,
        }
    }
}

#[derive(Component)]
pub struct ExternalMoves {
    sender: Sender<String>,
    receiver: Receiver<String>,
}
impl ExternalMoves {
    fn spawn_comms_threads(child: &mut Child, sender: Sender<String>, receiver: Receiver<String>) {
        let mut stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        // Spawn a thread to TALK to the process
        thread::spawn(move || {
            for line in receiver {
                if let Err(e) = stdin.write_all(line.as_bytes()) {
                    panic!("Could not talk to child process: {:?}", e);
                }
            }
        });

        // Spawn a thread to LISTEN to the process
        thread::spawn(move || {
            let mut f = BufReader::new(stdout);

            loop {
                let mut buf = String::new();
                if let Err(e) = f.read_line(&mut buf) {
                    panic!("Could not listen to child process: {:?}", e);
                } else {
                    let msg = buf.trim().to_string();
                    if let Err(e) = sender.send(msg) {
                        panic!("{:?}", e);
                    }
                }
            }
        });
    }

    pub fn new(command: String, args: Vec<String>) -> Self {
        let (tx1, rx1) = unbounded();
        let (tx2, rx2) = unbounded();

        let mut child = Command::new(&command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| panic!("Could not spawn process {command}: {e:?}"));

        Self::spawn_comms_threads(&mut child, tx1, rx2);
        Self {
            sender: tx2,
            receiver: rx1,
        }
    }

    pub fn recv(&self) -> Option<Direction> {
        match self.receiver.try_recv() {
            Ok(answer) => answer.parse().ok(),
            Err(_) => None,
        }
    }

    pub fn send(&self, msg: String) {
        self.sender.send(msg).unwrap_or_default();
    }
}

#[derive(Component, PartialEq, Eq, PartialOrd, Ord)]
pub enum AiMoves {
    Easy,
    Medium,
    Hard,
}
