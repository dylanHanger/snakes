use std::{
    fs::File,
    io::{BufWriter, Write},
};

use bevy::prelude::Resource;

// Define the struct that will hold a BufWriter and implement the Copy trait
#[derive(Resource)]
pub struct ReplayWriter {
    writer: BufWriter<File>,
}
impl ReplayWriter {
    pub fn new(file: File) -> Self {
        Self {
            writer: BufWriter::new(file),
        }
    }
}

// Implement the Clone trait for the ReplayWriter struct
impl Clone for ReplayWriter {
    fn clone(&self) -> ReplayWriter {
        // Create a new BufWriter from the file in the original ReplayWriter
        let writer = BufWriter::new(self.writer.get_ref().try_clone().unwrap());

        // Return a new ReplayWriter with the new BufWriter
        ReplayWriter { writer }
    }
}

// Implement the Write trait for the ReplayWriter struct
impl Write for ReplayWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
