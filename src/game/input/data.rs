use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

use bevy::prelude::{Component, KeyCode};
use crossbeam_channel::{unbounded, Receiver, Sender};
use serde::Deserialize;
use wait_timeout::ChildExt;

use crate::game::movement::prelude::Direction;

#[derive(Component)]
pub struct RandomAi;

#[derive(Component, Debug, Clone, Copy, Deserialize)]
pub struct KeyboardInput {
    pub north: KeyCode,
    pub east: KeyCode,
    pub south: KeyCode,
    pub west: KeyCode,
}

#[derive(Component)]
pub struct CustomAi {
    stdin: Sender<String>,
    stdout: Receiver<String>,
    stderr: Receiver<String>,
    child: Child,

    pub silent: bool,
}
impl CustomAi {
    fn spawn_comms_threads(
        child: &mut Child,
        output: Sender<String>,
        input: Receiver<String>,
        err: Sender<String>,
    ) {
        let mut stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        // Spawn a thread to TALK to the process
        thread::spawn(move || {
            for line in input {
                if stdin.write_all(line.as_bytes()).is_err() {
                    return;
                }
            }
        });

        // Spawn a thread to LISTEN to the process stdout
        thread::spawn(move || {
            let mut f = BufReader::new(stdout);

            loop {
                let mut buf = String::new();
                if f.read_line(&mut buf).is_ok() {
                    let msg = buf.trim().to_string();
                    output.send(msg).unwrap_or_default();
                } else {
                    return;
                }
            }
        });

        // Spawn a thread to LISTEN to the process stderr
        thread::spawn(move || {
            let mut stderr_reader = BufReader::new(stderr);

            loop {
                // Stderr
                let mut buf = String::new();
                if stderr_reader.read_line(&mut buf).is_ok() {
                    let msg = buf.trim().to_string();
                    err.send(msg).unwrap_or_default();
                } else {
                    return;
                }
            }
        });
    }

    pub fn new(command: String, args: Vec<String>, silent: bool) -> Self {
        let (tx1, rx1) = unbounded();
        let (tx2, rx2) = unbounded();
        let (tx3, rx3) = unbounded();

        let mut child = Command::new(&command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| panic!("Could not spawn process {command}: {e:?}"));

        Self::spawn_comms_threads(&mut child, tx1, rx2, tx3);
        Self {
            stdin: tx2,
            stdout: rx1,
            stderr: rx3,
            child,
            silent,
        }
    }

    pub fn recv(&self) -> Option<Direction> {
        match self.stdout.try_recv() {
            Ok(answer) => answer.parse().ok(),
            Err(_) => None,
        }
    }
    pub fn recv_err(&self) -> Option<String> {
        self.stderr.try_recv().ok()
    }

    pub fn send(&self, msg: String) {
        self.stdin.send(msg).unwrap_or_default();
    }
}
impl Drop for CustomAi {
    fn drop(&mut self) {
        if self
            .child
            .wait_timeout(Duration::from_millis(500))
            .unwrap()
            .is_none()
        {
            if let Err(e) = self.child.kill() {
                println!("Could not kill process {}: {}", self.child.id(), e)
            }
        }
    }
}

#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuiltinAi {
    Easy,
    Medium,
    Hard,
}
